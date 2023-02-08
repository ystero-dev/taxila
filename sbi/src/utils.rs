//! Utility functions for going through OpenAPI Specifications
//!
//!

use indexmap::IndexMap;
#[allow(unused)]
use openapiv3::*;

// Get references for `AnySchema`
fn get_references_for_schema_anyschema(any: &AnySchema) -> Vec<String> {
    let mut references = vec![];

    for (_, prop) in &any.properties {
        references.extend(get_references_for_reference_or_box_schema(prop));
    }
    if any.additional_properties.is_some() {
        let ap = any.additional_properties.as_ref().unwrap();
        if let AdditionalProperties::Schema(s) = ap {
            references.extend(get_references_for_reference_or_schema(s))
        }
    }
    references.extend(get_references_for_vec_reference_or_schema(&any.one_of));
    references.extend(get_references_for_vec_reference_or_schema(&any.any_of));
    references.extend(get_references_for_vec_reference_or_schema(&any.all_of));

    if any.items.is_some() {
        let items = any.items.as_ref().unwrap();
        references.extend(get_references_for_reference_or_box_schema(items));
    }

    references
}

// Get references for `Type` (`SchemaKind::Type`)
fn get_references_for_schema_type(t: &Type) -> Vec<String> {
    let mut references = vec![];
    match t {
        Type::Object(o) => {
            for (_, value) in &o.properties {
                references.extend(get_references_for_reference_or_box_schema(value));
            }

            if o.additional_properties.is_some() {
                let ap = o.additional_properties.as_ref().unwrap();
                if let AdditionalProperties::Schema(s) = ap {
                    references.extend(get_references_for_reference_or_schema(s))
                }
            }
        }
        Type::Array(a) => {
            if a.items.is_some() {
                references.extend(get_references_for_reference_or_box_schema(
                    a.items.as_ref().unwrap(),
                ));
            }
        }
        _ => {}
    }

    references
}

// Get references for a `ReferenceOr<Box<Schema>` object.
//
// Typically a boxed schema is insed a `Type`
fn get_references_for_reference_or_box_schema(r: &ReferenceOr<Box<Schema>>) -> Vec<String> {
    let mut references = vec![];
    match r {
        ReferenceOr::Reference { reference } => {
            println!("reference : {}", reference);
            references.push(reference.clone());
        }
        ReferenceOr::Item(schema) => references.extend(get_references_for_schema(schema)),
    }
    references
}

// Get references for a `ReferenceOr<Schema>` object
fn get_references_for_reference_or_schema(r: &ReferenceOr<Schema>) -> Vec<String> {
    let mut references = vec![];
    match r {
        ReferenceOr::Reference { reference } => {
            println!("reference : {}", reference);
            references.push(reference.clone());
        }
        ReferenceOr::Item(schema) => references.extend(get_references_for_schema(schema)),
    }

    references
}

// Get references for a `Vec<ReferenceOr<Schema>>`
//
// Essentially, gets references for each of the elements of the Vec
fn get_references_for_vec_reference_or_schema(refs: &Vec<ReferenceOr<Schema>>) -> Vec<String> {
    let mut references = vec![];
    for r in refs {
        references.extend(get_references_for_reference_or_schema(r));
    }

    references
}

// Get references for a `Schema` Object.
//
// Calls the function to get references for each of the `SchemaKind`.
pub(crate) fn get_references_for_schema(schema: &Schema) -> Vec<String> {
    let mut references = vec![];
    match &schema.schema_kind {
        SchemaKind::Type(t) => references.extend(get_references_for_schema_type(t)),
        SchemaKind::OneOf { one_of } => {
            references.extend(get_references_for_vec_reference_or_schema(one_of))
        }
        SchemaKind::AnyOf { any_of } => {
            references.extend(get_references_for_vec_reference_or_schema(any_of))
        }
        SchemaKind::AllOf { all_of } => {
            references.extend(get_references_for_vec_reference_or_schema(all_of))
        }
        SchemaKind::Not { not } => references.extend(get_references_for_reference_or_schema(not)),
        SchemaKind::Any(a) => references.extend(get_references_for_schema_anyschema(a)),
    }

    references
}

// Get references for `MediaType`
fn get_references_for_media_type(media_type: &MediaType) -> Vec<String> {
    let mut references = vec![];

    if media_type.schema.is_some() {
        references.extend(get_references_for_reference_or_schema(
            media_type.schema.as_ref().unwrap(),
        ))
    }
    references
}

// Get references for `RequestBody`
//
// Gets references from the `media_type` property of the `RequestBody`.
fn get_references_for_request_body(req_body: &RequestBody) -> Vec<String> {
    let mut references = vec![];

    for media_type in req_body.content.values() {
        references.extend(get_references_for_media_type(media_type));
    }

    references
}

// Get reference for `ReferenceOr<RequestBody>`
fn get_references_for_reference_or_request_body(value: &ReferenceOr<RequestBody>) -> Vec<String> {
    let mut references = vec![];

    match value {
        ReferenceOr::Reference { reference } => {
            println!("reference: {}", reference);
            references.push(reference.clone());
        }
        ReferenceOr::Item(req_body) => {
            references.extend(get_references_for_request_body(req_body));
        }
    }

    references
}

// Get references for `ReferenceOr<Parameter>`
//
// Gets references for the `parameters/` inside the `Operation`.
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
                        references.extend(get_references_for_reference_or_schema(&s))
                    }
                    ParameterSchemaOrContent::Content(c) => {
                        for (_, media_type) in c {
                            references.extend(get_references_for_media_type(&media_type));
                        }
                    }
                }
            }
        }
    }

    references
}

// Get references for the `Response` types
//
// Gets references from the `media_type` inside the Response.
fn get_references_for_response(response: &Response) -> Vec<String> {
    let mut references = vec![];
    for (_content_type, media_type) in &response.content {
        if media_type.schema.is_some() {
            references.extend(get_references_for_reference_or_schema(
                media_type.schema.as_ref().unwrap(),
            ));
        }
    }

    references
}

// Get `references` from `ReferenceOr<Response>` Object.
fn get_references_for_reference_or_response(value: &ReferenceOr<Response>) -> Vec<String> {
    let mut references = vec![];

    match value {
        ReferenceOr::Reference { reference } => {
            println!("reference: {}", reference);
            references.push(reference.clone());
        }
        ReferenceOr::Item(r) => references.extend(get_references_for_response(r)),
    }

    references
}

// Gets references for `responses/responses`. Also if a `default` response is presnt, get
// additional references from the `default` response.
fn get_references_for_responses(responses: &Responses) -> Vec<String> {
    let mut references = vec![];
    for (_status_code, response) in &responses.responses {
        references.extend(get_references_for_reference_or_response(response));
    }

    if responses.default.is_some() {
        references.extend(get_references_for_reference_or_response(
            responses.default.as_ref().unwrap(),
        ));
    }

    references
}

// Get References for `ReferenceOr<CallBack>` Enum.
//
// If the `Enum` is type `Reference`, append the reference to the list of known references.
// If the `Enum` is type `Item(Callback { ...} })` Gets the references from the `paths` inside the
// `Callback`.
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
                    references.extend(get_references_for_path_item(p));
                }
            }
        }
    }

    references
}

// Get References from `PathItem`.
//
// Gets references for each of the `Operation`s inside the `PathItem`.
fn get_references_for_path_item(path_item: &PathItem) -> Vec<String> {
    let mut references = vec![];

    if path_item.get.is_some() {
        references.extend(get_references_for_operation(
            path_item.get.as_ref().unwrap(),
        ));
    }
    if path_item.put.is_some() {
        references.extend(get_references_for_operation(
            path_item.put.as_ref().unwrap(),
        ));
    }
    if path_item.post.is_some() {
        references.extend(get_references_for_operation(
            path_item.post.as_ref().unwrap(),
        ));
    }
    if path_item.delete.is_some() {
        references.extend(get_references_for_operation(
            path_item.delete.as_ref().unwrap(),
        ));
    }
    if path_item.patch.is_some() {
        references.extend(get_references_for_operation(
            path_item.patch.as_ref().unwrap(),
        ));
    }

    references
}

// Get References from `Operation`
//
// Gets rererences from -
// 1. parameters: if Any to the `Operation`
// 2. responses: for the `Operation`
// 3. callbacks: for the `Operation`
// 4. request_body: for the `Operation`
fn get_references_for_operation(op: &Operation) -> Vec<String> {
    let mut references = vec![];
    references.extend(get_references_for_parameters(&op.parameters));
    references.extend(get_references_for_responses(&op.responses));
    references.extend(get_references_for_reference_or_callbacks(&op.callbacks));
    if op.request_body.is_some() {
        references.extend(get_references_for_reference_or_request_body(
            op.request_body.as_ref().unwrap(),
        ));
    }

    references
}

// Goes through all the `Object`s of the parsed `OpenAPI` spec and tries to collect references from
// those. This function will be called by the generator function.
//
// Gets references from:
// 1. `components/schemas` going deeper for each of the referred objects.
// 2. `components/request_bodies` (Only if `schema_only` flag is `false`.
// 3. `paths/` (Only if `schema_only` flag is `false`.)
pub(crate) fn get_dependent_refs_for_spec(spec: &OpenAPI, schema_only: bool) -> Vec<String> {
    let mut references = vec![];

    if spec.components.is_some() {
        let components = spec.components.as_ref().unwrap();
        for (_k, schema) in &components.schemas {
            references.extend(get_references_for_reference_or_schema(schema));
        }

        if !schema_only {
            for r in components.request_bodies.values() {
                references.extend(get_references_for_reference_or_request_body(r));
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
                    references.extend(get_references_for_path_item(p));
                }
            }
        }
    }

    references
}
