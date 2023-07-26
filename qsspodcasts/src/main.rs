use autocomplete_server::{Autocompleter, AutocompleterMessageProxy};
use business_core::business_core::BusinessCore;
use clap::Parser;
use frontend::terminal_frontend::Frontend;

use std::rc::Rc;
use std::sync::Arc;

use tokio::sync::Mutex as TokioMutex;

use command_management::{
    autocompletion::autocompletion_data_list_build::build_command_autocompletion_data_list,
    command_engine::CommandEngine, command_executor::CommandExecutor,
};
use data_transport::{DataReceiver, DataSender};
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

fn build_data_transfer_endpoints<T>(slots: usize) -> (DataSender<T>, DataReceiver<T>) {
    let (sender, reader) = channel(slots);
    (DataSender::new(sender), DataReceiver::new(reader))
}

fn build_app_components<Drawer: frontend::ui_drawers::ui_drawer::UiDrawer + Default>(
) -> (CommandEngine, Frontend<Drawer>, AutocompleterMessageProxy) {
    let path_provider = DefaultPathProvider {};
    let mp3_player = Arc::new(TokioMutex::new(GStreamerMp3Player::new(Box::new(
        path_provider,
    ))));

    let (command_sender, command_reader) = build_data_transfer_endpoints(10);

    let (output_sender, output_reader) = build_data_transfer_endpoints(10);
    let (notifications_sender, notifications_reader) = build_data_transfer_endpoints(10);
    let (autocompletion_request_sender, autocompletion_request_reader) =
        build_data_transfer_endpoints(10);
    let (autocompletion_response_sender, autocompletion_response_reader) =
        build_data_transfer_endpoints(10);

    let core = BusinessCore::new(
        mp3_player.clone(),
        Rc::new(path_provider),
        Some(notifications_sender.clone()),
    );

    let executor = CommandExecutor::new(core, Some(autocompletion_request_sender.clone()));
    let command_engine = CommandEngine::new(
        executor,
        Some(command_reader),
        Some(output_sender),
        Some(notifications_sender),
    );

    let frontend = Frontend::new(
        command_sender,
        output_reader,
        notifications_reader,
        autocompletion_request_sender,
        autocompletion_response_reader,
        mp3_player,
        Box::<Drawer>::default(),
    );

    let autocompleter_engine = Autocompleter::new(build_command_autocompletion_data_list());
    let autocompleter = AutocompleterMessageProxy::new(
        autocompleter_engine,
        autocompletion_request_reader,
        autocompletion_response_sender,
    );

    (command_engine, frontend, autocompleter)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut command_engine, mut frontend, mut autocompleter) = build_app_components::<
        frontend::ui_drawers::minimalistic_ui_drawer::MinimalisticUiDrawer,
    >();
    let command_frontend_future = frontend.run();
    let command_engine_future = command_engine.run();
    let autocompleter_future = autocompleter.run();
    if futures::join!(
        command_frontend_future,
        command_engine_future,
        autocompleter_future
    )
    .0
    .is_err()
    {
        println!("Not working !");
    }

    Ok(())
}
