use std::io::Write;

use crate::errors::BomError;
use once_cell::sync::Lazy;
use serde::{
    ser::{Serialize, SerializeStruct},
    Deserialize,
};
use yaserde::ser::Config;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Debug, Deserialize, YaDeserialize, YaSerialize)]
#[serde(rename_all = "camelCase")]
#[yaserde(
    prefix = "ns",
    default_namespace = "ns",
    namespace = "ns:http://cyclonedx.org/schema/bom/1.3"
)]
struct Bom {
    #[yaserde(attribute)]
    version: Option<u32>,

    #[yaserde(attribute, rename = "serialNumber")]
    serial_number: Option<String>,
}

static YASERDE_CONFIG: Lazy<Config> = Lazy::new(|| Config {
    perform_indent: true,
    write_document_declaration: true,
    indent_string: Some(String::from("  ")),
});

impl Bom {
    pub fn write_json(&self, writer: impl Write) -> Result<(), BomError> {
        serde_json::to_writer_pretty(writer, self).map_err(BomError::JsonSerializationError)
    }

    pub fn to_json(&self) -> Result<String, BomError> {
        serde_json::to_string_pretty(self).map_err(BomError::JsonSerializationError)
    }

    pub fn write_xml<W: Write>(&self, writer: W) -> Result<W, BomError> {
        yaserde::ser::serialize_with_writer(self, writer, &YASERDE_CONFIG)
            .map_err(BomError::XmlSerializationError)
    }

    pub fn to_xml(&self) -> Result<String, BomError> {
        yaserde::ser::to_string_with_config(
            self,
            &yaserde::ser::Config {
                perform_indent: true,
                write_document_declaration: true,
                indent_string: Some("  ".to_string()),
            },
        )
        .map_err(BomError::XmlSerializationError)
    }
}

// TODO: Remove this manual step if/when https://github.com/media-io/yaserde/issues/131 is implemented and replace with JSON-specific fields
impl Serialize for Bom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Bom", 4)?;
        state.serialize_field("bomFormat", "CycloneDX")?;
        state.serialize_field("specVersion", "1.3")?;
        state.serialize_field("version", &self.version)?;
        state.serialize_field("serialNumber", &self.serial_number)?;
        state.end()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_serialize_to_json() {
        let actual = Bom {
            version: Some(1),
            serial_number: Some("fake-uuid".to_string()),
        };

        insta::assert_json_snapshot!(actual);
    }

    #[test]
    fn it_should_serialize_to_xml() {
        let actual = Bom {
            version: Some(1),
            serial_number: Some("fake-uuid".to_string()),
        };

        let actual_xml = actual.to_xml().expect("Failed to output BOM as XML");
        insta::assert_snapshot!(actual_xml);
    }
}
