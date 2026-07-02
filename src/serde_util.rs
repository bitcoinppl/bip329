use crate::SpendableFieldValue;
use serde::Deserializer;

pub(crate) fn deserialize_optional_string_or_bool<'de, D>(
    deserializer: D,
) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    // keep normal output parsing aligned with metadata-aware parsing
    let value = <SpendableFieldValue as serde::Deserialize>::deserialize(deserializer)?;

    Ok(value.explicit_value())
}
