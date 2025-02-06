use oxc_ast::ast::Program;

use super::swift_type_trait::SwiftType;

pub struct SwiftTransformer;

impl SwiftTransformer {
  pub fn transform(ast_program: &Program) -> String {
    let mut output = String::new();

    for statement in &ast_program.body {
      let statement_code = statement.to_swift_type();
      output.push_str(&statement_code);
    }

    output
  }
}
