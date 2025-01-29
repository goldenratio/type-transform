use oxc_ast::ast::{Program, Statement};

pub struct SwiftTransformer;

impl SwiftTransformer {
  pub fn transform(ast_program: &Program) -> String {
    let mut output = String::new();

    for statement in &ast_program.body {
      match statement {
        Statement::ExportNamedDeclaration(export_decl) => {
          // println!("Found a export declaration: {:?}", export_decl.specifiers);
        }
        Statement::TSInterfaceDeclaration(interface_decl) => {
          println!("Found a interface declaration: {:?}", interface_decl.id);
          println!(
            "Found a interface declaration: {:?}",
            interface_decl.body.body
          );
          output.push_str(&format!("protocol {} {{\n", interface_decl.id));
          output.push_str("}\n");
        }
        _ => {
          // ignore classes, functions, etc.. (we are only interesetd in types/interfaces)
        }
      }
    }

    output
  }
}
