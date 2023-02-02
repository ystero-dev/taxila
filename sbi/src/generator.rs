//! Rust code generator for the 5G Service Based Interface data definitions and service stubs.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
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
    _module: String,
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
                    _module: module_name.to_string(),
                },
            );
        }

        let mut all_references = vec![];
        for v in self.specs.values() {
            let references = Self::get_dependency_for_spec(&v.spec);
            all_references.extend(references);
        }

        let mut uniq = BTreeSet::new();
        for reference in &all_references {
            let source = reference.split('#').next().unwrap().to_string();
            uniq.insert(source);
        }

        println!(
            "total_references: {}, uniq: {:#?}",
            all_references.len(),
            uniq
        );

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
                        _module: module_name.to_string(),
                    },
                );
            }
        }

        let mut all_references = vec![];
        for v in self.specs.values() {
            let references = Self::get_dependency_for_spec(&v.spec);
            all_references.extend(references);
        }

        let mut uniq = BTreeSet::new();
        for reference in &all_references {
            let source = reference.split('#').next().unwrap().to_string();
            uniq.insert(source);
        }

        println!(
            "total_references: {}, uniq: {:#?}",
            all_references.len(),
            uniq
        );
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

    fn get_references_for_schema_anyschema(any: &AnySchema) -> Vec<String> {
        let mut references = vec![];

        for (_, prop) in &any.properties {
            references.extend(Self::get_references_for_reference_or_box_schema(prop));
        }
        if any.additional_properties.is_some() {
            let ap = any.additional_properties.as_ref().unwrap();
            if let AdditionalProperties::Schema(s) = ap {
                references.extend(Self::get_references_for_reference_or_schema(s))
            }
        }
        references.extend(Self::get_references_for_vec_reference_or_schema(
            &any.one_of,
        ));
        references.extend(Self::get_references_for_vec_reference_or_schema(
            &any.any_of,
        ));
        references.extend(Self::get_references_for_vec_reference_or_schema(
            &any.all_of,
        ));

        if any.items.is_some() {
            let items = any.items.as_ref().unwrap();
            references.extend(Self::get_references_for_reference_or_box_schema(items));
        }

        references
    }

    fn get_references_for_schema_type(t: &Type) -> Vec<String> {
        let mut references = vec![];
        match t {
            Type::Object(o) => {
                for (_, value) in &o.properties {
                    references.extend(Self::get_references_for_reference_or_box_schema(value));
                }

                if o.additional_properties.is_some() {
                    let ap = o.additional_properties.as_ref().unwrap();
                    if let AdditionalProperties::Schema(s) = ap {
                        references.extend(Self::get_references_for_reference_or_schema(s))
                    }
                }
            }
            Type::Array(a) => {
                if a.items.is_some() {
                    references.extend(Self::get_references_for_reference_or_box_schema(
                        a.items.as_ref().unwrap(),
                    ));
                }
            }
            _ => {}
        }

        references
    }

    fn get_references_for_reference_or_box_schema(r: &ReferenceOr<Box<Schema>>) -> Vec<String> {
        let mut references = vec![];
        match r {
            ReferenceOr::Reference { reference } => {
                println!("reference : {}", reference);
                references.push(reference.clone());
            }
            ReferenceOr::Item(schema) => references.extend(Self::get_references_for_schema(schema)),
        }
        references
    }

    fn get_references_for_reference_or_schema(r: &ReferenceOr<Schema>) -> Vec<String> {
        let mut references = vec![];
        match r {
            ReferenceOr::Reference { reference } => {
                println!("reference : {}", reference);
                references.push(reference.clone());
            }
            ReferenceOr::Item(schema) => references.extend(Self::get_references_for_schema(schema)),
        }

        references
    }

    fn get_references_for_vec_reference_or_schema(refs: &Vec<ReferenceOr<Schema>>) -> Vec<String> {
        let mut references = vec![];
        for r in refs {
            references.extend(Self::get_references_for_reference_or_schema(r));
        }

        references
    }

    fn get_references_for_schema(schema: &Schema) -> Vec<String> {
        let mut references = vec![];
        match &schema.schema_kind {
            SchemaKind::Type(t) => references.extend(Self::get_references_for_schema_type(t)),
            SchemaKind::OneOf { one_of } => {
                references.extend(Self::get_references_for_vec_reference_or_schema(one_of))
            }
            SchemaKind::AnyOf { any_of } => {
                references.extend(Self::get_references_for_vec_reference_or_schema(any_of))
            }
            SchemaKind::AllOf { all_of } => {
                references.extend(Self::get_references_for_vec_reference_or_schema(all_of))
            }
            SchemaKind::Not { not } => {
                references.extend(Self::get_references_for_reference_or_schema(not))
            }
            SchemaKind::Any(a) => references.extend(Self::get_references_for_schema_anyschema(a)),
        }

        references
    }

    fn get_references_for_media_type(media_type: &MediaType) -> Vec<String> {
        let mut references = vec![];

        if media_type.schema.is_some() {
            references.extend(Self::get_references_for_reference_or_schema(
                media_type.schema.as_ref().unwrap(),
            ))
        }
        references
    }

    fn get_references_for_request_body(req_body: &RequestBody) -> Vec<String> {
        let mut references = vec![];

        for media_type in req_body.content.values() {
            references.extend(Self::get_references_for_media_type(media_type));
        }

        references
    }

    fn get_references_for_reference_or_request_body(
        value: &ReferenceOr<RequestBody>,
    ) -> Vec<String> {
        let mut references = vec![];

        match value {
            ReferenceOr::Reference { reference } => {
                println!("reference: {}", reference);
                references.push(reference.clone());
            }
            ReferenceOr::Item(req_body) => {
                references.extend(Self::get_references_for_request_body(req_body));
            }
        }

        references
    }

    fn get_references_for_parameters(params: &Vec<ReferenceOr<Parameter>>) -> Vec<String> {
        let mut references = vec![];
        for param in params {
            match param {
                ReferenceOr::Reference { reference } => {
                    println!("reference: {}", reference);
                    references.push(reference.clone());
                }
                ReferenceOr::Item(i) => {
                    let pdata = i.clone().parameter_data();
                    match pdata.format {
                        ParameterSchemaOrContent::Schema(s) => {
                            references.extend(Self::get_references_for_reference_or_schema(&s))
                        }
                        ParameterSchemaOrContent::Content(c) => {
                            for (_, media_type) in c {
                                references.extend(Self::get_references_for_media_type(&media_type));
                            }
                        }
                    }
                }
            }
        }

        references
    }

    fn get_references_for_response(response: &Response) -> Vec<String> {
        let mut references = vec![];
        for (_content_type, media_type) in &response.content {
            if media_type.schema.is_some() {
                references.extend(Self::get_references_for_reference_or_schema(
                    media_type.schema.as_ref().unwrap(),
                ));
            }
        }

        references
    }

    fn get_references_for_reference_or_response(value: &ReferenceOr<Response>) -> Vec<String> {
        let mut references = vec![];

        match value {
            ReferenceOr::Reference { reference } => {
                println!("reference: {}", reference);
                references.push(reference.clone());
            }
            ReferenceOr::Item(r) => references.extend(Self::get_references_for_response(r)),
        }

        references
    }

    fn get_references_for_responses(responses: &Responses) -> Vec<String> {
        let mut references = vec![];
        for (_status_code, response) in &responses.responses {
            references.extend(Self::get_references_for_reference_or_response(response));
        }

        if responses.default.is_some() {
            references.extend(Self::get_references_for_reference_or_response(
                responses.default.as_ref().unwrap(),
            ));
        }

        references
    }

    fn get_references_for_reference_or_callbacks(
        callbacks: &IndexMap<String, ReferenceOr<Callback>>,
    ) -> Vec<String> {
        let mut references = vec![];

        for (_, callback) in callbacks {
            match callback {
                ReferenceOr::Reference { reference } => {
                    println!("reference: {}", reference);
                    references.push(reference.clone());
                }
                ReferenceOr::Item(callback_map) => {
                    for (_, p) in callback_map {
                        references.extend(Self::get_references_for_path_item(p));
                    }
                }
            }
        }

        references
    }

    fn get_references_for_path_item(path_item: &PathItem) -> Vec<String> {
        let mut references = vec![];

        if path_item.get.is_some() {
            references.extend(Self::get_references_for_operation(
                path_item.get.as_ref().unwrap(),
            ));
        }
        if path_item.put.is_some() {
            references.extend(Self::get_references_for_operation(
                path_item.put.as_ref().unwrap(),
            ));
        }
        if path_item.post.is_some() {
            references.extend(Self::get_references_for_operation(
                path_item.post.as_ref().unwrap(),
            ));
        }
        if path_item.delete.is_some() {
            references.extend(Self::get_references_for_operation(
                path_item.delete.as_ref().unwrap(),
            ));
        }
        if path_item.patch.is_some() {
            references.extend(Self::get_references_for_operation(
                path_item.patch.as_ref().unwrap(),
            ));
        }

        references
    }

    fn get_references_for_operation(op: &Operation) -> Vec<String> {
        let mut references = vec![];
        references.extend(Self::get_references_for_parameters(&op.parameters));
        references.extend(Self::get_references_for_responses(&op.responses));
        references.extend(Self::get_references_for_reference_or_callbacks(
            &op.callbacks,
        ));
        if op.request_body.is_some() {
            references.extend(Self::get_references_for_reference_or_request_body(
                op.request_body.as_ref().unwrap(),
            ));
        }

        references
    }

    fn get_dependency_for_spec(spec: &OpenAPI) -> Vec<String> {
        let mut references = vec![];
        for path in &spec.paths.paths {
            match path.1 {
                openapiv3::ReferenceOr::Reference { reference } => {
                    println!("reference: {}", reference);
                    references.push(reference.clone());
                }
                ReferenceOr::Item(p) => {
                    references.extend(Self::get_references_for_path_item(p));
                }
            }
        }

        eprintln!("references total: {}", references.len());

        if spec.components.is_some() {
            let components = spec.components.as_ref().unwrap();
            for (_k, schema) in &components.schemas {
                references.extend(Self::get_references_for_reference_or_schema(schema));
            }

            for r in components.request_bodies.values() {
                references.extend(Self::get_references_for_reference_or_request_body(r));
            }
        }

        references
    }
}
