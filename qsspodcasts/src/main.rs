use business_core::business_core::BusinessCore;
use clap::Parser;
use frontend::terminal_frontend::Frontend;

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
    let mut core = BusinessCore::new();
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
