use oxc_ast::ast::Program;

use super::{kotlin_transformer::KotlinTransformer, swift_transformer::SwiftTransformer};

pub const DEFAULT_LANGUAGE: LanguageType = LanguageType::Swift;

pub enum LanguageType {
  Swift,
  Kotlin,
}

impl From<&str> for LanguageType {
  fn from(value: &str) -> Self {
    match value {
      "swift" => LanguageType::Swift,
      "kt" => LanguageType::Kotlin,
      "kts" => LanguageType::Kotlin,
      _ => DEFAULT_LANGUAGE,
    }
  }
}

pub struct LanguageTransformFactory;

impl LanguageTransformFactory {
  pub fn transform(language_type: LanguageType, ast_program: &Program) -> String {
    match language_type {
      LanguageType::Swift => SwiftTransformer::transform(ast_program),
      LanguageType::Kotlin => KotlinTransformer::transform(ast_program),
    }
  }
}
