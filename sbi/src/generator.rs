//! Rust code generator for the 5G Service Based Interface data definitions and service stubs.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

#[allow(unused)]
use openapiv3::*;

#[derive(Debug, Clone)]
pub struct Generator {
    specs_dir: PathBuf,
    specs: HashMap<String, SpecModule>,
}

#[derive(Debug, Clone)]
struct SpecModule {
    spec: OpenAPI,
    module: String,
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
            let spec = self.parse_spec_from_file(entry)?;
            self.specs.insert(
                entry.as_ref().to_str().unwrap().to_string(),
                SpecModule {
                    spec,
                    module: module_name.to_string(),
                },
            );
        }

        for (_, v) in &self.specs {
            Self::get_dependency_for_spec(&v.spec);
        }

        Ok(())
    }

    pub fn generate_all(&mut self, module_name: &str) -> std::io::Result<()> {
        for entry in self.specs_dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().unwrap() == "yaml" {
                let spec = self.parse_spec_from_file(path.file_name().unwrap())?;
                self.specs.insert(
                    path.file_name().unwrap().to_str().unwrap().to_string(),
                    SpecModule {
                        spec,
                        module: module_name.to_string(),
                    },
                );
            }
        }

        for (_, v) in &self.specs {
            Self::get_dependency_for_spec(&v.spec);
        }

        Ok(())
    }

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

    fn get_references_for_schema_anyschema(any: &AnySchema) {
        for (_, prop) in &any.properties {
            Self::get_references_for_reference_or_box_schema(&prop);
        }
        if any.additional_properties.is_some() {
            let ap = any.additional_properties.as_ref().unwrap();
            match ap {
                AdditionalProperties::Schema(s) => {
                    Self::get_references_for_reference_or_schema(&*s)
                }
                _ => {}
            }
        }
        Self::get_references_for_vec_reference_or_schema(&any.one_of);
        Self::get_references_for_vec_reference_or_schema(&any.any_of);
        Self::get_references_for_vec_reference_or_schema(&any.all_of);

        if any.items.is_some() {
            let items = any.items.as_ref().unwrap();
            Self::get_references_for_reference_or_box_schema(items);
        }
    }

    fn get_references_for_schema_type(t: &Type) {
        match t {
            Type::Object(o) => {
                for (_, value) in &o.properties {
                    Self::get_references_for_reference_or_box_schema(value);
                }

                if o.additional_properties.is_some() {
                    let ap = o.additional_properties.as_ref().unwrap();
                    match ap {
                        AdditionalProperties::Schema(s) => {
                            Self::get_references_for_reference_or_schema(&*s)
                        }
                        _ => {}
                    }
                }
            }
            Type::Array(a) => {
                if a.items.is_some() {
                    Self::get_references_for_reference_or_box_schema(&a.items.as_ref().unwrap());
                }
            }
            _ => {}
        }
    }

    fn get_references_for_reference_or_box_schema(r: &ReferenceOr<Box<Schema>>) {
        match r {
            ReferenceOr::Reference { reference } => {
                println!("reference : {}", reference);
            }
            ReferenceOr::Item(schema) => Self::get_references_for_schema(&*schema),
        }
    }

    fn get_references_for_reference_or_schema(r: &ReferenceOr<Schema>) {
        match r {
            ReferenceOr::Reference { reference } => {
                println!("reference : {}", reference);
            }
            ReferenceOr::Item(schema) => Self::get_references_for_schema(&schema),
        }
    }

    fn get_references_for_vec_reference_or_schema(refs: &Vec<ReferenceOr<Schema>>) {
        for r in refs {
            Self::get_references_for_reference_or_schema(r);
        }
    }

    fn get_references_for_schema(schema: &Schema) {
        match &schema.schema_kind {
            SchemaKind::Type(t) => Self::get_references_for_schema_type(t),
            SchemaKind::OneOf { one_of } => {
                Self::get_references_for_vec_reference_or_schema(&one_of)
            }
            SchemaKind::AnyOf { any_of } => {
                Self::get_references_for_vec_reference_or_schema(&any_of)
            }
            SchemaKind::AllOf { all_of } => {
                Self::get_references_for_vec_reference_or_schema(&all_of)
            }
            SchemaKind::Not { not } => Self::get_references_for_reference_or_schema(&*not),
            SchemaKind::Any(a) => Self::get_references_for_schema_anyschema(a),
        }
    }

    fn get_references_for_parameters(params: &Vec<ReferenceOr<Parameter>>) {
        for param in params {
            match param {
                ReferenceOr::Reference { reference } => {
                    println!("reference: {}", reference);
                }
                ReferenceOr::Item(i) => {
                    let pdata = i.clone().parameter_data();
                    match pdata.format {
                        ParameterSchemaOrContent::Schema(s) => match s {
                            ReferenceOr::Reference { reference } => {
                                println!("reference: {}", reference);
                            }
                            ReferenceOr::Item(_) => {}
                        },
                        ParameterSchemaOrContent::Content(c) => {
                            for (_, media_type) in c {
                                if media_type.schema.is_some() {
                                    let s = media_type.schema.unwrap();
                                    match s {
                                        ReferenceOr::Reference { reference } => {
                                            println!("reference: {}", reference);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_references_for_response(response: &Response) {
        for (_content_type, media_type) in &response.content {
            if media_type.schema.is_some() {
                Self::get_references_for_reference_or_schema(media_type.schema.as_ref().unwrap());
            }
        }
    }

    fn get_references_for_responses(responses: &Responses) {
        for (_status_code, response) in &responses.responses {
            match response {
                ReferenceOr::Reference { reference } => {
                    println!("reference: {}", reference)
                }
                ReferenceOr::Item(r) => Self::get_references_for_response(r),
            }
        }
    }

    fn get_references_for_operation(op: &Operation) {
        Self::get_references_for_parameters(&op.parameters);
        Self::get_references_for_responses(&op.responses);
    }

    fn get_dependency_for_spec(spec: &OpenAPI) -> BTreeSet<String> {
        for path in &spec.paths.paths {
            match path.1 {
                openapiv3::ReferenceOr::Reference { reference } => {
                    println!("reference: {}", reference)
                }
                ReferenceOr::Item(PathItem {
                    get,
                    put,
                    post,
                    delete,
                    patch,
                    ..
                }) => {
                    if get.is_some() {
                        Self::get_references_for_operation(&get.as_ref().unwrap());
                    }
                    if put.is_some() {
                        Self::get_references_for_operation(&put.as_ref().unwrap());
                    }
                    if post.is_some() {
                        Self::get_references_for_operation(&post.as_ref().unwrap());
                    }
                    if delete.is_some() {
                        Self::get_references_for_operation(&delete.as_ref().unwrap());
                    }
                    if patch.is_some() {
                        Self::get_references_for_operation(&patch.as_ref().unwrap());
                    }
                }
            }
        }

        if spec.components.is_some() {
            let components = spec.components.as_ref().unwrap();
            for (_k, schema) in &components.schemas {
                Self::get_references_for_reference_or_schema(schema);
            }
        }

        BTreeSet::new()
    }
}
