use crate::languages::transformer::{LanguageType, DEFAULT_LANGUAGE};

pub fn get_language_from_file_name(file_name: &str) -> LanguageType {
  let ext = file_name.split(".").last();
  match ext {
    Some(val) => LanguageType::from(val),
    None => DEFAULT_LANGUAGE,
  }
}
