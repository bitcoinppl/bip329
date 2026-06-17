use crate::SpendableFieldValue;
use serde::Deserializer;

pub(crate) fn deserialize_string_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    // keep normal output parsing aligned with metadata-aware parsing
    let value = <SpendableFieldValue as serde::Deserialize>::deserialize(deserializer)?;

    value
        .explicit_value()
        .ok_or_else(|| serde::de::Error::custom("missing spendable value"))
}
