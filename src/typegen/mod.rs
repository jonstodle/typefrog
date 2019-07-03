use crate::typegen::languages::Rust;
use crate::typegen::schema_types::Schema;
use std::fmt::{self, Display};

pub mod languages;
mod schema_types;

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {
}

pub fn generate_types(schema: &str) -> Result<String, Error> {
    let schema = serde_json::from_str::<Schema>(schema)
        .map_err(|e| Error(e.to_string()))?;

    Ok(Rust::generate_types(&schema.types))
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, Write};
    use crate::typegen::generate_types;

    #[test]
    fn generate_types_should_output_a_string() {
        let mut input = String::new();
        let _ = File::open("/Users/jon/Downloads/gqltest/new-papi-schema.json").unwrap()
            .read_to_string(&mut input);

        let types = generate_types(&input).unwrap();

        let _ = File::create("/Users/jon/Dev/typefrog/src/papi.rs").unwrap()
            .write_all(types.as_ref());
    }
}
