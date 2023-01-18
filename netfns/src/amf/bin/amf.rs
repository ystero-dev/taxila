use std::fs::File;
use std::io::Read;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about="AMF: Accesss and Mobility Management Function (netns-5g)", long_about = None)]
struct Cli {
    #[arg(short, help="Enable debug", action=clap::ArgAction::Count)]
    debug: u8,

    #[arg(short, long, required = true, name = "CONFIG_FILE", env = "AMF_CONFIG")]
    config: String,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let level = if cli.debug > 0 {
        if cli.debug == 1 {
            "debug"
        } else {
            "trace"
        }
    } else {
        "info"
    };

    let env = env_logger::Env::default().filter_or("MY_LOG_LEVEL", level);
    env_logger::init_from_env(env);

    let mut config_file = File::open(cli.config)?;
    let mut config = String::new();
    config_file.read_to_string(&mut config)?;

    let amf_config = serde_yaml::from_str(&config).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("config parse error. {}", e),
        )
    })?;

    let amf = netfns_5g::Amf::from_config(amf_config)?;

    amf.run().await
}
