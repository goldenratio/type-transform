use chrono::Local;

pub fn get_content_banner_header() -> String {
  format!("// This Code is auto generated!\n// Time: {}", Local::now()).to_owned()
}
