use oxc_ast::ast::Program;

use super::kotlin_type_trait::KotlinType;

pub struct KotlinTransformer;

impl KotlinTransformer {
  pub fn transform(ast_program: &Program) -> String {
    let mut import_banners = String::new();
    let mut output = String::new();

    for statement in &ast_program.body {
      let statement_code = statement.to_kotlin_type();
      output.push_str(&statement_code);
    }

    // TODO: for now a naive aprroach, think about better solution
    if output.contains("suspend ") || output.contains("Deferred") {
      import_banners.push_str("import kotlinx.coroutines.Deferred\n");
    }

    format!("{}\n{}", import_banners, output)
  }
}
