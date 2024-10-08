use crate::{
    c_api::auxil::{char_pointer_to_string, string_to_char_pointer},
    errors::LoreCoreError,
    types::*,
};

#[repr(C)]
#[derive(Clone)]
pub struct CHistoryItem {
    pub timestamp: i64,
    pub year: i32,
    pub day: u32,
    pub content: *const libc::c_char,
    pub properties: *const libc::c_char,
}

fn to_c_history_item(item: &HistoryItem) -> Result<CHistoryItem, LoreCoreError> {
    Ok(CHistoryItem {
        timestamp: item.timestamp.to_int(),
        year: item.year.to_int(),
        day: item.day.to_int(),
        content: string_to_char_pointer(item.content.to_str()),
        properties: string_to_char_pointer(&item.properties.to_string()),
    })
}

impl TryFrom<HistoryItem> for CHistoryItem {
    type Error = LoreCoreError;

    fn try_from(value: HistoryItem) -> Result<Self, Self::Error> {
        to_c_history_item(&value)
    }
}

unsafe fn to_history_item(item: &CHistoryItem) -> Result<HistoryItem, LoreCoreError> {
    Ok(HistoryItem {
        timestamp: item.timestamp.into(),
        year: item.year.into(),
        day: item.day.into(),
        content: char_pointer_to_string(item.content)?.into(),
        properties: (&char_pointer_to_string(item.properties)?).into(),
    })
}

impl TryFrom<&CHistoryItem> for HistoryItem {
    type Error = LoreCoreError;

    fn try_from(value: &CHistoryItem) -> Result<Self, Self::Error> {
        unsafe { to_history_item(value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::timestamp::current_timestamp;

    #[test]
    fn history_item_roundtrips() {
        let years = vec![Year(-13), Year(2021)];
        let days = vec![Day(None), Day(Some(14))];
        let contents = vec!["", "Test content", "\\_\"'%$&!{[]}"];
        let properties = vec![
            HistoryItemProperties::none(),
            "{\"is_secret\":true}".into(),
            "{\"additional_concerns\":[\"\\entityref{some_label}\"]}".into(),
        ];
        for year in years {
            for day in &days {
                for content in &contents {
                    for property in &properties {
                        let item_before = HistoryItem {
                            timestamp: current_timestamp(),
                            year,
                            day: *day,
                            content: (*content).into(),
                            properties: property.clone(),
                        };
                        let c_item = to_c_history_item(&item_before).unwrap();
                        let item_after = unsafe { to_history_item(&c_item).unwrap() };
                        assert_eq!(item_before, item_after);
                    }
                }
            }
        }
    }
}
