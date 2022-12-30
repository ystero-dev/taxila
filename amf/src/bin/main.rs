#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let config = "ngap:\n addrs:\n - 127.0.0.1";

    let amf_config = serde_yaml::from_str(config)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "config parse error"))?;

    let mut amf = taxila_amf::Amf::from_config(&amf_config)?;

    amf.run().await
}
