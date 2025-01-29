use oxc_ast::ast::Program;

pub trait LanguageTransformer {
  fn transform(ast_program: &Program) -> String;
}
