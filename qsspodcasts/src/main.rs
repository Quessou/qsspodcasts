use business_core::business_core::BusinessCore;
use clap::Parser;
use frontend::terminal_frontend::Frontend;

use std::rc::Rc;
use std::sync::Arc;

use tokio::sync::Mutex as TokioMutex;

use command_management::{command_engine::CommandEngine, command_executor::CommandExecutor};
use data_transport::{data_receiver::DataReceiver, data_sender::DataSender};
use path_providing::default_path_provider::DefaultPathProvider;
use podcast_player::players::gstreamer_mp3_player::GStreamerMp3Player;

use tokio::sync::mpsc::channel;

/// Lame podcast manager
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Url to register to retrieve a podcast
    #[clap(short, long, default_value = "")]
    add_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path_provider = DefaultPathProvider {};
    let mp3_player = Arc::new(TokioMutex::new(GStreamerMp3Player::new(Box::new(
        path_provider,
    ))));

    let (command_sender, command_reader) = channel(10);
    let (command_sender, command_reader) = (
        DataSender::new(command_sender),
        DataReceiver::new(command_reader),
    );
    let (output_sender, output_reader) = channel(10);
    let (output_sender, output_reader) = (
        DataSender::new(output_sender),
        DataReceiver::new(output_reader),
    );
    let (notifications_sender, notifications_reader) = channel(10);
    let (notifications_sender, notifications_reader) = (
        DataSender::new(notifications_sender),
        DataReceiver::new(notifications_reader),
    );

    let core = BusinessCore::new(
        mp3_player.clone(),
        Rc::new(path_provider),
        Some(notifications_sender),
    );

    let executor = CommandExecutor::new(core);
    let mut command_engine =
        CommandEngine::new(executor, Some(command_reader), Some(output_sender));

    let mut frontend = Frontend::new(
        command_sender,
        output_reader,
        notifications_reader,
        mp3_player.clone(),
        Box::new(frontend::ui_drawers::minimalistic_ui_drawer::MinimalisticUiDrawer::new()),
    );

    let command_frontend_future = frontend.run();
    let command_engine_future = command_engine.run();
    if futures::join!(command_frontend_future, command_engine_future)
        .0
        .is_err()
    {
        println!("Not working !");
    }

    Ok(())
}
