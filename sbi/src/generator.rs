//! Rust code generator for the 5G Service Based Interface data definitions and service stubs.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use openapiv3::OpenAPI;

#[derive(Debug, Clone)]
pub struct Generator {
    specs_dir: PathBuf,
    specs: HashMap<String, OpenAPI>,
}

impl Generator {
    pub fn from_path<P: AsRef<Path>>(specs_dir: P) -> std::io::Result<Self> {
        let specs_dir: PathBuf = specs_dir.as_ref().into();

        if !specs_dir.metadata()?.is_dir() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{}: Not a directory", specs_dir.to_string_lossy()),
            ))
        } else {
            Ok(Self {
                specs_dir,
                specs: HashMap::new(),
            })
        }
    }

    pub fn generate<P: AsRef<Path>>(&mut self, files_modules: &[(P, &str)]) -> std::io::Result<()> {
        for (entry, module_name) in files_modules {
            self.generate_for_file(entry, module_name)?;
        }

        Ok(())
    }

    fn generate_for_file<P: AsRef<Path>>(
        &mut self,
        file: P,
        _module_name: &str,
    ) -> std::io::Result<()> {
        let file_name = file.as_ref().to_string_lossy().to_string();
        let full_path = self.specs_dir.canonicalize()?.join(&file_name);
        let reader = std::fs::File::open(full_path)?;
        let spec: OpenAPI = serde_yaml::from_reader(reader).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Yaml Error: {}", e))
        })?;
        self.specs.insert(file_name, spec);

        Ok(())
    }
}
