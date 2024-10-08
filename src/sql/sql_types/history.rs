use diesel::{Insertable, Queryable};

use crate::{sql::schema::history_items, types::*};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Insertable, Queryable)]
#[diesel(table_name = history_items)]
pub(crate) struct SqlHistoryItem {
    pub timestamp: i64,
    pub year: i32,
    pub day: i32,
    pub content: String,
    pub properties: String,
}

impl PartialEq<&SqlHistoryItem> for SqlHistoryItem {
    fn eq(&self, other: &&SqlHistoryItem) -> bool {
        self.timestamp == other.timestamp
            && self.year == other.year
            && self.day == other.day
            && self.content == other.content
            && self.properties == other.properties
    }
}

impl HistoryItem {
    pub(crate) fn to_sql_history_item(&self) -> SqlHistoryItem {
        SqlHistoryItem {
            timestamp: self.timestamp.to_int(),
            year: self.year.to_int(),
            day: self.day.to_int() as i32,
            content: self.content.to_string(),
            properties: self.properties.to_string(),
        }
    }
}

impl SqlHistoryItem {
    pub(crate) fn to_history_item(&self) -> HistoryItem {
        HistoryItem {
            timestamp: self.timestamp.into(),
            year: self.year.into(),
            day: self.day.into(),
            content: self.content.as_str().into(),
            properties: (&self.properties).into(),
        }
    }
}
