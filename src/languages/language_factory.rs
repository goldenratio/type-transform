use oxc_ast::ast::Program;

use super::{
  kotlin::kotlin_transformer::KotlinTransformer, swift::swift_transformer::SwiftTransformer,
};

pub enum LanguageType {
  Swift,
  Kotlin,
}

impl TryFrom<String> for LanguageType {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value.as_str() {
      "swift" => Ok(LanguageType::Swift),
      "kotlin" => Ok(LanguageType::Kotlin),
      "kt" => Ok(LanguageType::Kotlin),
      "kts" => Ok(LanguageType::Kotlin),
      _ => Err("unknown language!"),
    }
  }
}

pub struct LanguageFactory;

impl LanguageFactory {
  pub fn transform(language_type: LanguageType, ast_program: &Program) -> String {
    match language_type {
      LanguageType::Swift => SwiftTransformer::transform(ast_program),
      LanguageType::Kotlin => KotlinTransformer::transform(ast_program),
    }
  }
}
