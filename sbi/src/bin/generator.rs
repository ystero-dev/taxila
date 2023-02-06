//! A simple generator utility for generating specs files.

use sbi5g::Generator;

fn main() -> std::io::Result<()> {
    let mut generator = Generator::from_path("specs")?;

    /*
    generator.generate_all("sbi", /* schema_only */ true)?;
    */

    generator.generate(
        // file_modules
        &[("TS29509_Nausf_UEAuthentication.yaml", "common_data")],
        // aux_files:
        &["TS29503_Nudm_UEAU.yaml", "TS29571_CommonData.yaml"],
        // schema_only
        true,
    )?;

    //eprintln!("generator: {:#?}", generator);

    Ok(())
}
