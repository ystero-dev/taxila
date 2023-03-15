//! 5G Service Based Interface Data Types and Stubs

mod generator;
pub use generator::Generator;

mod utils;

mod schema;
pub use schema::{sanitize_str_for_ident, AnyOfHandler};

mod anyof_handlers;
pub use anyof_handlers::default_anyof_handler;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate url;

mod models;
