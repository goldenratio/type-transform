use oxc_ast::ast::Program;

use super::kotlin_type_trait::KotlinType;

pub struct KotlinTransformer;

impl KotlinTransformer {
  pub fn transform(ast_program: &Program) -> String {
    let mut output = String::new();

    for statement in &ast_program.body {
      let statement_code = statement.to_kotlin_type();
      output.push_str(&statement_code);
    }

    output
  }
}
