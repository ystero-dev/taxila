//! A simple generator utility for generating specs files.

use sbi5g::Generator;

fn main() -> std::io::Result<()> {
    let mut generator = Generator::from_path("specs")?;

    /*
    generator.generate_all("sbi", /* schema_only */ true)?;
    */

    generator.generate(
        // file_modules
        &[
            ("TS29571_CommonData.yaml", "common_data"),
            ("TS29509_Nausf_UEAuthentication.yaml", "common_data"),
            ("TS29509_Nausf_SoRProtection.yaml", "common_data"),
        ],
        // aux_files:
        &[
            "TS29510_Nnrf_AccessToken.yaml",
            "TS29514_Npcf_PolicyAuthorization.yaml",
            "TS29572_Nlmf_Location.yaml",
            "TS29503_Nudm_UEAU.yaml",
        ],
        // schema_only
        true,
    )?;

    //eprintln!("generator: {:#?}", generator);

    Ok(())
}
