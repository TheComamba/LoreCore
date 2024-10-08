use ::diesel::prelude::*;

use crate::{
    errors::{sql_loading_error, LoreCoreError},
    types::*,
};

use super::{
    lore_database::LoreDatabase, schema::history_items, search_params::HistoryItemSearchParams,
    sql_types::*,
};

impl LoreDatabase {
    pub fn write_history_items(&self, cols: Vec<HistoryItem>) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        for col in cols.into_iter() {
            let col = col.to_sql_history_item();
            diesel::insert_into(history_items::table)
                .values(&col)
                .execute(&mut connection)
                .map_err(|e| {
                    LoreCoreError::SqlError(
                        "Writing history item to database failed: ".to_string() + &e.to_string(),
                    )
                })?;
        }
        Ok(())
    }

    pub fn redate_history_item(
        &self,
        timestamp: Timestamp,
        year: Year,
        day: Day,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(
            history_items::table.filter(history_items::timestamp.eq(timestamp.to_int())),
        )
        .set((
            history_items::year.eq(year.to_int()),
            history_items::day.eq(day.to_int() as i32),
        ))
        .execute(&mut connection)
        .map_err(|e| {
            LoreCoreError::SqlError(
                "Redating history item in database failed: ".to_string() + &e.to_string(),
            )
        })?;
        Ok(())
    }

    pub fn delete_history_item(&self, timestamp: Timestamp) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::delete(
            history_items::table.filter(history_items::timestamp.eq(timestamp.to_int())),
        )
        .execute(&mut connection)
        .map_err(|e| {
            LoreCoreError::SqlError(
                "Deleting history item from database failed: ".to_string() + &e.to_string(),
            )
        })?;
        Ok(())
    }

    pub fn change_history_item_content(
        &self,
        timestamp: Timestamp,
        content: &HistoryItemContent,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(
            history_items::table.filter(history_items::timestamp.eq(timestamp.to_int())),
        )
        .set(history_items::content.eq(content.to_str()))
        .execute(&mut connection)
        .map_err(|e| {
            LoreCoreError::SqlError(
                "Changing history item content in database failed: ".to_string() + &e.to_string(),
            )
        })?;
        Ok(())
    }

    pub fn change_history_item_properties(
        &self,
        timestamp: Timestamp,
        properties: &HistoryItemProperties,
    ) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        diesel::update(
            history_items::table.filter(history_items::timestamp.eq(timestamp.to_int())),
        )
        .set(history_items::properties.eq(properties.to_string()))
        .execute(&mut connection)
        .map_err(|e| {
            LoreCoreError::SqlError(
                "Changing history item properties in database failed: ".to_string()
                    + &e.to_string(),
            )
        })?;
        Ok(())
    }

    pub fn read_history_items(
        &self,
        search_params: HistoryItemSearchParams,
    ) -> Result<Vec<HistoryItem>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let mut query = history_items::table.into_boxed();
        let year = search_params.year;
        if let Some(year) = year {
            query = query.filter(history_items::year.eq(year.to_int()));
        }
        let day = search_params.day;
        if let Some(day) = day {
            query = query.filter(history_items::day.eq(day.to_int() as i32));
        }
        let timestamp = search_params.timestamp;
        if let Some(timestamp) = timestamp {
            query = query.filter(history_items::timestamp.eq(timestamp.to_int()));
        }
        let content = search_params.content;
        if content.is_some() {
            if content.is_exact {
                query = query.filter(history_items::content.eq(content.exact_text()));
            } else {
                query = query.filter(history_items::content.like(content.search_pattern()));
            }
        }
        let mut items: Vec<_> = query
            .load::<SqlHistoryItem>(&mut connection)
            .map_err(|e| {
                sql_loading_error("history items", vec![("year", &year), ("day", &day)], e)
            })?
            .into_iter()
            .map(|item| item.to_history_item())
            .collect();
        items.sort();
        Ok(items)
    }
}
