use super::{lore_database::LoreDatabase, search_text::EntityColumnSearchParams};
use crate::{
    errors::{sql_loading_error, LoreCoreError},
    sql::schema::entities,
};
use ::diesel::prelude::*;
use diesel::{Insertable, RunQueryDsl};

#[derive(Insertable, Queryable, PartialEq, Clone, Debug)]
#[diesel(table_name = entities)]
#[repr(C)]
pub struct EntityColumn {
    pub label: String,
    pub descriptor: String,
    pub description: Option<String>,
}

impl LoreDatabase {
    pub fn write_entity_columns(&self, cols: Vec<EntityColumn>) -> Result<(), LoreCoreError> {
        let mut connection = self.db_connection()?;
        for col in cols.into_iter() {
            diesel::insert_into(entities::table)
                .values(&col)
                .execute(&mut connection)
                .map_err(|e| {
                    LoreCoreError::SqlError(
                        "Writing column to database failed: ".to_string() + &e.to_string(),
                    )
                })?;
        }
        Ok(())
    }

    pub fn get_entity_columns(
        &self,
        search_params: EntityColumnSearchParams,
    ) -> Result<Vec<EntityColumn>, LoreCoreError> {
        let mut connection = self.db_connection()?;
        let mut query = entities::table.into_boxed();
        let label = search_params.label;
        if label.is_some() {
            if label.is_exact {
                query = query.filter(entities::label.eq(label.to_string()));
            } else {
                query = query.filter(entities::label.like(label.to_string()));
            }
        }
        let descriptor = search_params.descriptor;
        if descriptor.is_some() {
            if descriptor.is_exact {
                query = query.filter(entities::descriptor.eq(descriptor.to_string()));
            } else {
                query = query.filter(entities::descriptor.like(descriptor.to_string()));
            }
        }
        let cols = query.load::<EntityColumn>(&mut connection).map_err(|e| {
            sql_loading_error(
                "entities",
                "columns",
                vec![("label", &label), ("descriptor", &descriptor)],
                e,
            )
        })?;
        Ok(cols)
    }
}

pub fn get_labels(entity_columns: &Vec<EntityColumn>) -> Vec<String> {
    let mut cols = entity_columns
        .iter()
        .map(|c| c.label.clone())
        .collect::<Vec<String>>();
    cols.sort();
    cols.dedup();
    cols
}

pub fn get_descriptors(entity_columns: &Vec<EntityColumn>) -> Vec<String> {
    let mut cols = entity_columns
        .iter()
        .map(|c| c.descriptor.clone())
        .collect::<Vec<String>>();
    cols.sort();
    cols.dedup();
    cols
}
