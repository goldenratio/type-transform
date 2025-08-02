use crate::languages::language_factory::LanguageType;

pub fn get_language_from_file_name(file_name: &str) -> Option<LanguageType> {
  let ext = file_name.split(".").last();
  match ext {
    Some(val) => LanguageType::try_from(val.to_owned()).ok(),
    None => Option::None,
  }
}

pub fn parse_banner(val: &Option<String>) -> String {
  let banner = val
    .as_ref()
    .map(|s| s.split("\\n").collect::<Vec<_>>().join("\n"))
    .unwrap_or_default();

  if !banner.is_empty() {
    format!("{banner}\n")
  } else {
    banner
  }
}

pub fn parse_footer(val: &Option<String>) -> String {
  let footer = val
    .as_ref()
    .map(|s| s.split("\\n").collect::<Vec<_>>().join("\n"))
    .unwrap_or_default();

  if !footer.is_empty() {
    format!("\n{footer}")
  } else {
    footer
  }
}
