use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, help="Enable debug", action=clap::ArgAction::Count)]
    debug: u8,

    #[arg(short, long, required = true, name = "CONFIG_FILE", env = "AMF_CONFIG")]
    config: String,
}

#[tokio::main(flavor = "current_thread")]
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

    let config = "ngap:\n addrs:\n - 127.0.0.1";
    let amf_config = serde_yaml::from_str(config)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "config parse error"))?;

    let mut amf = taxila_amf::Amf::from_config(&amf_config)?;

    amf.run().await
}
