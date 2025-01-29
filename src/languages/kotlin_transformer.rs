use oxc_ast::ast::Program;

use super::transformer::LanguageTransformer;

pub struct KotlinTransformer {}

impl LanguageTransformer for KotlinTransformer {
  fn transform(ast_program: &Program) -> String {
    todo!()
  }
}
