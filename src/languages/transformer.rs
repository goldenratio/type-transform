use oxc_ast::ast::Program;

use super::{kotlin_transformer::KotlinTransformer, swift_transformer::SwiftTransformer};

pub enum LanguageType {
  Swift,
  Kotlin,
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
