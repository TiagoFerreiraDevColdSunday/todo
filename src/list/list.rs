use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

mod date_format_serde;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List {
    pub title: String,
    pub description: String,
    #[serde(with = "crate::list::list::date_format_serde")]
    pub deadline: NaiveDate,
}
