use business_core::business_core::BusinessCore;
use clap::Parser;

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
    let args = Args::parse();

    let mut core = BusinessCore::new();
    core.initialize();
    core.build_podcasts().await;

    if !args.add_url.is_empty() {
        if core.add_url(&args.add_url).is_err() {
            println!("Error registering the URL");
        }
        return Ok(());
    }
    let play_future = core.download_some_random_podcast();

    if futures::join!(play_future).0.is_err() {
        println!("Not working !")
    }

    Ok(())
}
