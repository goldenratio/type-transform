use crate::languages::language_factory::LanguageType;

pub fn get_language_from_file_name(file_name: &str) -> Option<LanguageType> {
  let ext = file_name.split(".").last();
  match ext {
    Some(val) => LanguageType::try_from(val.to_owned()).ok(),
    None => Option::None,
  }
}
