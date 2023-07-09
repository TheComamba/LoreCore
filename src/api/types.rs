use super::auxil::{char_pointer_to_optional_string, char_pointer_to_string};
use crate::{
    errors::LoreCoreError,
    sql::{entity::EntityColumn, history::HistoryItem, relationships::EntityRelationship},
};

#[repr(C)]
#[derive(Clone)]
pub struct CEntityColumn {
    pub label: *const libc::c_char,
    pub descriptor: *const libc::c_char,
    pub description: *const libc::c_char,
}

#[repr(C)]
#[derive(Clone)]
pub struct CEntityRelationship {
    pub parent: *const libc::c_char,
    pub child: *const libc::c_char,
    pub role: *const libc::c_char,
}

#[repr(C)]
#[derive(Clone)]
pub struct CHistoryItem {
    pub label: *const libc::c_char,
    pub content: *const libc::c_char,
    pub is_concerns_others: u8,
    pub is_secret: u8,
    pub year: i32,
    pub day: i32,
    pub originator: *const libc::c_char,
}

pub(super) fn to_entity_column(column: &CEntityColumn) -> Result<EntityColumn, LoreCoreError> {
    Ok(EntityColumn {
        label: char_pointer_to_string(column.label)?,
        descriptor: char_pointer_to_string(column.descriptor)?,
        description: char_pointer_to_string(column.description)?,
    })
}

pub(super) fn to_history_item(item: &CHistoryItem) -> Result<HistoryItem, LoreCoreError> {
    Ok(HistoryItem {
        label: char_pointer_to_string(item.label)?,
        content: char_pointer_to_string(item.content)?,
        is_concerns_others: item.is_concerns_others != 0,
        is_secret: item.is_secret != 0,
        year: item.year,
        day: if item.day > 0 { Some(item.day) } else { None },
        originator: char_pointer_to_optional_string(item.originator)?,
    })
}

pub(super) fn to_relationship(
    rel: &CEntityRelationship,
) -> Result<EntityRelationship, LoreCoreError> {
    Ok(EntityRelationship {
        parent: char_pointer_to_string(rel.parent)?,
        child: char_pointer_to_string(rel.child)?,
        role: char_pointer_to_optional_string(rel.role)?,
    })
}