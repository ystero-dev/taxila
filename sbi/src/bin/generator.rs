//! A simple generator utility for generating specs files.

use sbi5g::Generator;

fn main() -> std::io::Result<()> {
    let mut generator = Generator::from_path("specs")?;

    //generator.generate_all("sbi")?;
    generator.generate(&[("TS29518_Namf_Communication.yaml", "amf_communication")])?;

    eprintln!("generator: {:#?}", generator);

    Ok(())
}
