use oxc_ast::ast::{Program, Statement};

use super::transformer::LanguageTransformer;

pub struct SwiftTransformer {}

impl LanguageTransformer for SwiftTransformer {
  fn transform(ast_program: &Program) -> String {
    let mut output = String::new();

    for stmt in &ast_program.body {
      match stmt {
        Statement::ExpressionStatement(expr_stmt) => {
          // println!("Found an expression: {:?}", expr_stmt.expression);
        }
        Statement::VariableDeclaration(var_decl) => {
          // println!("Found a variable declaration: {:?}", var_decl);
        }
        Statement::FunctionDeclaration(func_decl) => {
          println!("Found a function declaration: {:?}", func_decl.id);
        }
        Statement::ExportNamedDeclaration(export_decl) => {
          // println!("Found a export declaration: {:?}", export_decl.specifiers);
        }
        Statement::TSInterfaceDeclaration(interface_decl) => {
          println!("Found a interface declaration: {:?}", interface_decl.id);
          println!(
            "Found a interface declaration: {:?}",
            interface_decl.body.body
          );
        }
        _ => {
          // ignore classes, functions, etc.. (we are only interesetd in types/interfaces)
        }
      }
    }

    output
  }
}
