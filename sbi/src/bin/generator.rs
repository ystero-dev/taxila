//! A simple generator utility for generating specs files.

use sbi5g::Generator;

fn main() -> std::io::Result<()> {
    let mut generator = Generator::from_path("specs")?;

    generator.generate(&[("TS29571_CommonData.yaml.yaml", "common_data")])?;

    eprintln!("generator: {:#?}", generator);

    Ok(())
}
