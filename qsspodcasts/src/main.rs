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
use data_caches::podcast_state_cache_builder::build_podcast_state_cache;
use data_transport::{DataReceiver, DataSender};
use path_providing::{default_path_provider::DefaultPathProvider, path_provider::PathProvider};
use podcast_player::players::gstreamer_mp3_player::GStreamerMp3Player;

use tokio::sync::mpsc::channel;

/// Lame podcast manager
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Shows a popup with basic information about how to use the app
    #[clap(short, long)]
    show_first_start_popup: bool,
}

fn build_data_transfer_endpoints<T>(slots: usize) -> (DataSender<T>, DataReceiver<T>) {
    let (sender, reader) = channel(slots);
    (DataSender::new(sender), DataReceiver::new(reader))
}

async fn build_app_components<Drawer: frontend::ui_drawers::ui_drawer::UiDrawer + Default>(
) -> (CommandEngine, Frontend<Drawer>, AutocompleterMessageProxy) {
    let path_provider = Arc::new(DefaultPathProvider {});
    let mp3_player = GStreamerMp3Player::build(path_provider.clone()).await;

    let (command_sender, command_reader) = build_data_transfer_endpoints(10);

    let (output_sender, output_reader) = build_data_transfer_endpoints(10);
    let (notifications_sender, notifications_reader) = build_data_transfer_endpoints(10);
    let (autocompletion_request_sender, autocompletion_request_reader) =
        build_data_transfer_endpoints(10);
    let (autocompletion_response_sender, autocompletion_response_reader) =
        build_data_transfer_endpoints(10);

    let core = BusinessCore::new_in_arc(
        mp3_player.clone(),
        path_provider.clone(),
        Some(notifications_sender.clone()),
    )
    .await;

    let executor = CommandExecutor::new(core, Some(autocompletion_request_sender.clone()));
    let command_engine = CommandEngine::new(
        executor,
        Some(command_reader),
        Some(output_sender),
        Some(notifications_sender),
    );

    let podcast_state_cache = build_podcast_state_cache(path_provider)
        .await
        .expect("Building of podcast state cache failed");

    let frontend = Frontend::new(
        command_sender,
        output_reader,
        notifications_reader,
        autocompletion_request_sender,
        autocompletion_response_reader,
        mp3_player,
        Box::<Drawer>::default(),
        podcast_state_cache,
    );

    let autocompleter_engine = Autocompleter::new(build_command_autocompletion_data_list());
    let autocompleter = AutocompleterMessageProxy::new(
        autocompleter_engine,
        autocompletion_request_reader,
        autocompletion_response_sender,
    );

    (command_engine, frontend, autocompleter)
}

fn is_first_start() -> bool {
    let path_provider = DefaultPathProvider {};

    !path_provider.app_dir_path().is_dir()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut command_engine, mut frontend, mut autocompleter) = build_app_components::<
        frontend::ui_drawers::minimalistic_ui_drawer::MinimalisticUiDrawer,
    >()
    .await;
    let cli = Args::parse();
    let is_first_start = is_first_start() || cli.show_first_start_popup;
    let command_frontend_future = frontend.run(is_first_start);
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
