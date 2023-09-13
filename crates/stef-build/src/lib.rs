#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use std::path::{Path, PathBuf};

use stef_parser::Schema;
use thiserror::Error;

pub use self::definition::compile_schema;

mod decode;
mod definition;
mod encode;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed reading schema file at {file:?}")]
    Read {
        #[source]
        source: std::io::Error,
        file: PathBuf,
    },
}

pub fn compile(schemas: &[impl AsRef<Path>], _includes: &[impl AsRef<Path>]) -> Result<()> {
    for schema in schemas {
        let input = std::fs::read_to_string(schema).map_err(|e| Error::Read {
            source: e,
            file: schema.as_ref().to_owned(),
        })?;

        let schema = Schema::parse(&input).unwrap();
        let code = definition::compile_schema(&schema);

        println!("{code}");
    }

    Ok(())
}
