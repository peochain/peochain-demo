// Serde helper for bounded strings
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if value.len() > 42 {
        return Err(serde::ser::Error::custom("String exceeds maximum allowed length (42)"));
    }
    serializer.serialize_str(value)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.len() > 42 {
        return Err(serde::de::Error::custom("String exceeds maximum allowed length (42)"));
    }
    Ok(s)
}
