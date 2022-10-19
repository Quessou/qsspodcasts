use business_core::business_core::BusinessCore;
use clap::Parser;
use frontend::terminal_frontend::Frontend;

use std::rc::Rc;
use std::sync::Arc;

use tokio::sync::Mutex as TokioMutex;

use path_providing::default_path_provider::DefaultPathProvider;
use podcast_player::players::gstreamer_mp3_player::GStreamerMp3Player;

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
    let mut core = BusinessCore::new(mp3_player, Rc::new(path_provider));
    core.initialize();
    core.build_podcasts().await;

    let mut frontend = Frontend::new(
        core,
        Box::new(frontend::ui_drawers::minimalistic_ui_drawer::MinimalisticUiDrawer::new()),
    );

    let command_frontend_future = frontend.run();
    if futures::join!(command_frontend_future).0.is_err() {
        println!("Not working !");
    }

    Ok(())
}
