use clap::Parser;
use clipstash::{data::AppDatabase, web::renderer::Renderer};
use dotenv::dotenv;
use tokio::runtime::Runtime;

use std::path::PathBuf;
#[derive(Parser, Debug)]
#[command(name = "httpd")]
#[command(bin_name = "httpd")]
struct Cli {
    #[arg(short, long, default_value = "sqlite:data.db")]
    connection_string: String,
    #[arg(short, long, default_value = "templates/")]
    template_directory: PathBuf,
}

fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    let rt = Runtime::new().expect("failed to spawn tokio runtime");

    let handle = rt.handle().clone();

    rt.block_on(async move {
        let renderer = Renderer::new(cli.template_directory);
        let database = AppDatabase::new(&cli.connection_string).await;

        let config = clipstash::RocketConfig { renderer, database };

        clipstash::rocket(config)
            .launch()
            .await
            .expect("failed to launch rocket server");
    })
}
