//! Rust code generator for the 5G Service Based Interface data definitions and service stubs.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
#[allow(unused)]
use openapiv3::*;

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

    fn find_missing_files_if_any(
        &mut self,
        aux_files: &[&str],
        schema_only: bool,
    ) -> std::io::Result<()> {
        // First get all references
        // Now we get All references that are used by any of the specs. This is a bit involved. If
        // we are generating 'models' only, we can get those for the `components/schemas`  only,
        for v in self.specs.values() {
            let references = Self::get_dependency_for_spec(&v.spec, schema_only);
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
        for (ref_file_name, reference_set) in &self.references {
            for reference in reference_set {
                let file_values = reference.split("#").collect::<Vec<&str>>();
                let (file, values) = (file_values[0], file_values[1]);
                let spec = if aux_map.is_some() && !file.is_empty() {
                    aux_map.as_ref().unwrap().get(file)
                } else {
                    let spec_module = if !file.is_empty() {
                        self.specs.get(file)
                    } else {
                        self.specs.get(ref_file_name)
                    };
                    if spec_module.is_some() {
                        Some(&spec_module.unwrap().spec)
                    } else {
                        None
                    }
                }
                // Spec has to be Some or else we'd  have gotten missing files error
                .unwrap();

                // We now have a reference and a spec, let's try to resolve that.
                let components: _ = reference.rsplit("/").collect::<Vec<_>>();
                let component = components[0];
                let schemas = &spec.components.as_ref().unwrap().schemas;
                let schema = schemas.get(component);
                match schema.unwrap() {
                    ReferenceOr::Reference { reference } => {
                        unresolved_items.push((component.to_string(), reference.to_string()))
                    }
                    ReferenceOr::Item(s) => resolved_items.push(component.to_string()),
                }
            }
        }

        if unresolved_items.is_empty() {
            println!("resolved components: {:#?}", resolved_items);
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

    fn get_dependency_for_spec(spec: &OpenAPI, schema_only: bool) -> Vec<String> {
        let mut references = vec![];

        if spec.components.is_some() {
            let components = spec.components.as_ref().unwrap();
            for (_k, schema) in &components.schemas {
                references.extend(Self::get_references_for_reference_or_schema(schema));
            }

            if !schema_only {
                for r in components.request_bodies.values() {
                    references.extend(Self::get_references_for_reference_or_request_body(r));
                }
            }
        }

        if !schema_only {
            for path in &spec.paths.paths {
                match path.1 {
                    openapiv3::ReferenceOr::Reference { reference } => {
                        references.push(reference.clone());
                    }
                    ReferenceOr::Item(p) => {
                        references.extend(Self::get_references_for_path_item(p));
                    }
                }
            }
        }

        references
    }
}
