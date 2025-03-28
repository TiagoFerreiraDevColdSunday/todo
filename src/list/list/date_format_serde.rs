use chrono::NaiveDate;
use serde::{self, Deserialize, Deserializer, Serializer};

// Serialize NaiveDate as a string
pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date);
    serializer.serialize_str(&s)
}

// Deserialize a string as a NaiveDate
pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(serde::de::Error::custom)
}