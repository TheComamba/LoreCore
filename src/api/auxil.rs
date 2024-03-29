use crate::errors::LoreCoreError;
use std::ffi::{CStr, CString};

/// # Safety
///
/// `string` must be a valid C string.
pub unsafe fn char_pointer_to_string(string: *const libc::c_char) -> Result<String, LoreCoreError> {
    if string.is_null() {
        return Err(LoreCoreError::InputError(
            "Characterpointer is null.".to_string(),
        ));
    }
    let string: &str = CStr::from_ptr(string).to_str().map_err(|e| {
        LoreCoreError::InputError(
            "Could not convert characterpointer to string: ".to_string() + &e.to_string(),
        )
    })?;
    Ok(string.to_string())
}

pub(super) unsafe fn char_pointer_to_optional_string(
    string: *const libc::c_char,
) -> Result<Option<String>, LoreCoreError> {
    let string = char_pointer_to_string(string)?;
    Ok(if string.is_empty() {
        None
    } else {
        Some(string)
    })
}

pub fn string_to_char_pointer(string: &str) -> *const libc::c_char {
    CString::new(string).unwrap().into_raw()
}

pub(super) fn optional_string_to_char_pointer(string: &Option<String>) -> *const libc::c_char {
    match string {
        Some(string) => string_to_char_pointer(string),
        None => string_to_char_pointer(""),
    }
}

pub(super) fn char_ptr(message: &str) -> *const libc::c_char {
    CString::new(message).unwrap().into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_char_pointer_to_string() {
        let string = "Eyjafjallajökull!";
        let char_pointer = string_to_char_pointer(string);
        let result = unsafe { char_pointer_to_string(char_pointer) };
        assert_eq!(result.unwrap(), string);
    }

    #[test]
    fn test_null_pointer_to_string_error() {
        use std::ptr;
        let null_pointer = ptr::null();
        let result = unsafe { char_pointer_to_string(null_pointer) };
        assert!(
            result.is_err(),
            "Expected an error when converting a null pointer to a string"
        );
    }

    #[test]
    fn test_null_pointer_to_string() {
        let char_pointer = ptr::null();
        let result = unsafe { char_pointer_to_string(char_pointer) };
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_string_to_char_pointer() {
        let string = "";
        let char_pointer = string_to_char_pointer(string);
        let result = unsafe { char_pointer_to_string(char_pointer) };
        assert_eq!(result.unwrap(), string);
    }

    #[test]
    fn test_char_pointer_to_optional_string() {
        let string = "Eyjafjallajökull!";
        let char_pointer = string_to_char_pointer(string);
        let result = unsafe { char_pointer_to_optional_string(char_pointer) };
        assert_eq!(result.unwrap(), Some(string.to_string()));
    }

    #[test]
    fn test_null_pointer_to_optional_string() {
        let char_pointer = ptr::null();
        let result = unsafe { char_pointer_to_optional_string(char_pointer) };
        assert!(result.is_err());
    }

    #[test]
    fn test_string_to_char_pointer() {
        let string = "Eyjafjallajökull!";
        let char_pointer = string_to_char_pointer(string);
        let result = unsafe { char_pointer_to_string(char_pointer) };
        assert_eq!(result.unwrap(), string);
    }

    #[test]
    fn test_optional_string_to_char_pointer() {
        let string_opt = Some("Eyjafjallajökull!".to_string());
        let char_pointer = optional_string_to_char_pointer(&string_opt);
        let result = unsafe { char_pointer_to_optional_string(char_pointer) };
        assert_eq!(result.unwrap(), string_opt);
    }

    #[test]
    fn test_empty_optional_string_to_char_pointer() {
        let string_opt = Some("".to_string());
        let char_pointer = optional_string_to_char_pointer(&string_opt);
        let result = unsafe { char_pointer_to_optional_string(char_pointer) };
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_none_optional_string_to_char_pointer() {
        let string_opt = None;
        let char_pointer = optional_string_to_char_pointer(&string_opt);
        let result = unsafe { char_pointer_to_optional_string(char_pointer) };
        assert_eq!(result.unwrap(), None);
    }
}
