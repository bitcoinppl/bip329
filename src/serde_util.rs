use serde::{Deserialize as _, Deserializer};
pub(crate) fn deserialize_string_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum StringOrBool {
        String(String),
        Bool(bool),
    }

    match StringOrBool::deserialize(deserializer)? {
        StringOrBool::Bool(b) => Ok(b),
        StringOrBool::String(s) => match s.to_ascii_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            string => {
                let msg = format!("Invalid boolean string: {string}");
                Err(serde::de::Error::custom(msg))
            }
        },
    }
}
