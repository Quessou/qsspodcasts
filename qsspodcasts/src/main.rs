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

    if args.add_url != "" {
        if let Err(_) = core.add_url(&args.add_url) {
            println!("Error registering the URL");
        }
        return Ok(());
    }
    let toto = core.download_some_random_podcast();
    let result = toto.await;
    //if let Err(_) = {
    //    println!("Download failed !")
    //} else {
    //    println!("Download successful !");
    //}

    println!("Is finished !");
    Ok(())
}