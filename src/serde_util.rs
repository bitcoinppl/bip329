use serde::{Deserialize as _, Deserializer};
pub(crate) fn deserialize_string_or_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum StringOrBool {
        String(String),
        Bool(bool),
    }

    let opt = Option::deserialize(deserializer)?;

    match opt {
        None => Ok(None),
        Some(StringOrBool::Bool(b)) => Ok(Some(b)),
        Some(StringOrBool::String(s)) => match s.to_ascii_lowercase().as_str() {
            "true" => Ok(Some(true)),
            "false" => Ok(Some(false)),
            string => {
                let msg = format!("Invalid boolean string: {string}");
                Err(serde::de::Error::custom(msg))
            }
        },
    }
}
