//! Rust code generator for the 5G Service Based Interface data definitions and service stubs.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
#[allow(unused)]
use openapiv3::*;

use super::schema::resolve_schema_component;
use super::utils::{get_dependent_refs_for_spec, get_references_for_schema};

#[derive(Debug, Clone)]
pub struct Generator {
    specs_dir: PathBuf,
    specs: HashMap<String, SpecModule>, // A HashMap of ModuleName -> Parsed Specs
    references: HashMap<String, BTreeSet<String>>, // A HashMap of FileName -> References
    aux_files: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
struct SpecModule {
    spec_file_name: String,
    spec: OpenAPI,
    _module: String,
}

impl Generator {
    /// Create an Instance of the [`Generator`] from the given Directory.
    ///
    /// Expects the given path to be a directory, which will contain one or more OpenAPI
    /// specifications that will later be processed and the code is then generated for these
    /// specification.
    pub fn from_path<P: AsRef<Path> + std::cmp::Ord>(specs_dir: P) -> std::io::Result<Self> {
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
                references: HashMap::new(),
                aux_files: None,
            })
        }
    }

    /// Generate Definitions from the files passed as `(file, module)` tuples.
    ///
    /// Additionally, `aux_files` are specified from where referred definitions are 'hand picked'
    /// so that all the definitions in the original set of `files` can be resolved.
    ///
    /// `schema_only` is a switch used for resolving references in the path "/components/schemas"
    /// only. (This is useful for example when generating data models.)
    pub fn generate<P>(
        &mut self,
        files_modules: &[(P, &str)],
        aux_files: &[&str],
        schema_only: bool,
    ) -> std::io::Result<()>
    where
        P: AsRef<Path> + std::cmp::Ord + std::fmt::Debug,
    {
        // First we 'simply parse' the specs
        for (entry, module_name) in files_modules {
            let spec = self.parse_spec_from_file(entry)?;
            let spec_file_name = entry.as_ref().to_str().unwrap().to_string();
            self.specs.insert(
                entry.as_ref().to_str().unwrap().to_string(),
                SpecModule {
                    spec_file_name,
                    spec,
                    _module: module_name.to_string(),
                },
            );
        }

        // We Now have collected unique references In all files.
        // Check if missing files if any?
        self.find_missing_files_if_any(aux_files, schema_only)?;

        let _ = self
            .aux_files
            .replace(aux_files.iter().map(|&x| x.to_string()).collect());

        self.generate_for_schemas()?;

        Ok(())
    }

    /// Generate Rust code for all the files in the directory (See also: [`Self::from_path`])
    ///
    /// module_name: Name of the module to be used for output
    /// schema_only: Flag selecting whether only `components/schemas` to be consiered for code
    /// generation
    pub fn generate_all(&mut self, module_name: &str, schema_only: bool) -> std::io::Result<()> {
        let mut input_set = BTreeSet::new();
        for entry in self.specs_dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().unwrap() == "yaml" {
                let spec_path = path.file_name().unwrap();
                let spec_path_string = spec_path.to_str().unwrap().to_string();
                let spec = self.parse_spec_from_file(spec_path)?;
                let dependent_string = spec_path_string.clone();
                let spec_file_name = spec_path_string.clone();
                input_set.insert(dependent_string);
                self.specs.insert(
                    spec_path_string,
                    SpecModule {
                        spec_file_name,
                        spec,
                        _module: module_name.to_string(),
                    },
                );
            }
        }

        // Find missing files if any
        self.find_missing_files_if_any(&[], schema_only)?;

        self.generate_for_schemas()?;

        Ok(())
    }

    // Parses the OpenAPI specification from a Yaml File. Errors out on any error.
    fn parse_spec_from_file<P: AsRef<Path>>(&self, file: P) -> std::io::Result<OpenAPI> {
        let file_name = file.as_ref().to_string_lossy().to_string();
        eprintln!("generating for {}", file_name);

        let full_path = self.specs_dir.canonicalize()?.join(&file_name);
        let reader = std::fs::File::open(full_path)?;
        let spec: OpenAPI = serde_yaml::from_reader(reader).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Yaml Error: {}", e))
        })?;

        Ok(spec)
    }

    // Based on the input file set and auxilary file set, report if any files that are referenced
    // are missing, If they are missing, it's an error condition that is propagated to the User.
    fn find_missing_files_if_any(
        &mut self,
        aux_files: &[&str],
        schema_only: bool,
    ) -> std::io::Result<()> {
        // First get all references
        // Now we get All references that are used by any of the specs. This is a bit involved. If
        // we are generating 'models' only, we can get those for the `components/schemas`  only,
        for v in self.specs.values() {
            let references = get_dependent_refs_for_spec(&v.spec, schema_only);
            let reference_set = BTreeSet::from_iter(references.iter().map(|v| v.clone()));
            self.references
                .insert(v.spec_file_name.clone(), reference_set);
        }

        let mut missing_files = BTreeSet::new();
        for reference_set in self.references.values() {
            for reference in reference_set {
                let split = reference.split("#").collect::<Vec<&str>>();
                let (referred_file, _referred_ref) = (split[0], split[1]);
                if self
                    .references
                    .keys()
                    .find(|&file_name| file_name == referred_file)
                    .is_none()
                    && aux_files.iter().find(|&&f| f == referred_file).is_none()
                {
                    if !referred_file.is_empty() {
                        missing_files.insert(referred_file.clone());
                    }
                }
            }
        }

        if missing_files.is_empty() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Some of the Schema Objects cannot be resolved due to missing Spec Files: {}",
                    missing_files
                        .iter()
                        .cloned()
                        .collect::<Vec<&str>>()
                        .join(", ")
                ),
            ))
        }
    }

    // The actual function that generates the code for schemas.
    fn generate_for_schemas(&mut self) -> std::io::Result<()> {
        let aux_map = if self.aux_files.is_some() {
            let mut aux_map = IndexMap::<String, OpenAPI>::new();
            for file in self.aux_files.as_ref().unwrap() {
                let spec = self.parse_spec_from_file(file)?;
                aux_map.insert(file.to_string(), spec);
            }
            Some(aux_map)
        } else {
            None
        };

        let mut unresolved_items = vec![];
        let mut resolved_items = vec![];

        // We are going through set of files - given as input to generate and references in each of
        // those files. For any file, the local referene should not be resolved, as those `Schema`
        // objects will be resolved separately when we resolve all `components/schemas/*`.
        let mut aux_inner_references = BTreeSet::<(String, String)>::new();
        for (ref_file_name, reference_set) in &self.references {
            for reference in reference_set {
                let file_values = reference.split("#").collect::<Vec<&str>>();
                let (file, _values) = (file_values[0], file_values[1]);
                if file.is_empty() {
                    println!("skipping generation for local reference: {}", reference);
                    // Lcal reference, do nothing
                    continue;
                }
                println!("generating for reference: {}", reference);
                if aux_map.is_some() {
                    let spec = aux_map.as_ref().unwrap().get(file).unwrap();

                    // We now have a reference and a spec, let's try to resolve that.
                    let components: _ = reference.rsplit("/").collect::<Vec<_>>();
                    let component = components[0];
                    let schemas = &spec.components.as_ref().unwrap().schemas;
                    let schema = schemas.get(component);
                    match schema.unwrap() {
                        ReferenceOr::Reference { reference } => {
                            unresolved_items.push((component.to_string(), reference.to_string()))
                        }
                        ReferenceOr::Item(s) => {
                            resolved_items.push(resolve_schema_component(component, s));
                            let mut inner_schemas = vec![s];
                            let mut loop_count = 1;
                            loop {
                                let mut inner_refs = vec![];
                                for schema in &inner_schemas {
                                    inner_refs.extend(get_references_for_schema(schema));
                                }
                                println!(
                                    "loop_count:{}, inner_refs: {:#?}",
                                    loop_count, inner_refs
                                );

                                for inner in &inner_refs {
                                    aux_inner_references.insert((file.to_string(), inner.clone()));
                                }
                                inner_schemas.drain(..);
                                for inner in &inner_refs {
                                    let components: _ = inner.rsplit("/").collect::<Vec<_>>();
                                    let component = components[0];
                                    let schemas = &spec.components.as_ref().unwrap().schemas;
                                    let schema = schemas.get(component);
                                    match schema.unwrap() {
                                        ReferenceOr::Item(s) => inner_schemas.push(s),
                                        _ => {}
                                    }
                                }
                                if inner_schemas.is_empty() {
                                    break;
                                }
                                loop_count += 1;
                            }
                        }
                    }
                }
            }
        }
        println!("aux_inner_references: {:#?}", aux_inner_references);

        for (aux_file, aux_ref) in aux_inner_references {
            let spec = aux_map.as_ref().unwrap().get(&aux_file).unwrap();

            // We now have a reference and a spec, let's try to resolve that.
            let components: _ = aux_ref.rsplit("/").collect::<Vec<_>>();
            let component = components[0];
            let schemas = &spec.components.as_ref().unwrap().schemas;
            let schema = schemas.get(component);
            match schema.unwrap() {
                ReferenceOr::Reference { reference } => {
                    unresolved_items.push((component.to_string(), reference.to_string()))
                }
                ReferenceOr::Item(s) => {
                    resolved_items.push(resolve_schema_component(component, s));
                }
            }
        }

        for spec_module in self.specs.values() {
            let spec = &spec_module.spec;

            if spec.components.is_none() {
                continue;
            }
            let components = spec.components.as_ref().unwrap();
            for (component, schema) in &components.schemas {
                match schema {
                    ReferenceOr::Reference { reference } => {
                        unresolved_items.push((component.to_string(), reference.to_string()))
                    }
                    ReferenceOr::Item(s) => {
                        resolved_items.push(resolve_schema_component(component, s));
                    }
                }
            }
        }

        if unresolved_items.is_empty() {
            println!(
                "resolved components: {:#?}",
                resolved_items
                    .into_iter()
                    .flatten()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            );
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Unresolved Items: {}",
                    unresolved_items
                        .iter()
                        .map(|r| r.1.clone())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ))
        }
    }
}
