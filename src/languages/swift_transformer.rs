use oxc_ast::ast::{BindingPatternKind, Program, PropertyKey, Statement, TSSignature, TSType};

pub struct SwiftTransformer;

const INDENT_VALUE: &str = "  ";

trait SwiftType {
  fn to_swift_type(&self) -> String;
}

impl SwiftType for PropertyKey<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      PropertyKey::StaticIdentifier(id_name) => id_name.name.to_owned().into(),
      _ => "unknown id_name".to_owned(),
    }
  }
}

impl SwiftType for BindingPatternKind<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      BindingPatternKind::BindingIdentifier(val) => val.name.to_string(),
      _ => "uknown BindingPatternKind".to_owned(),
    }
  }
}

impl SwiftType for TSType<'_> {
  fn to_swift_type(&self) -> String {
    // type mapping from TypeScript to Swift
    match self {
      TSType::TSStringKeyword(_) => "String".to_string(),
      TSType::TSNumberKeyword(_) => "Double".to_string(),
      TSType::TSBooleanKeyword(_) => "Bool".to_string(),
      TSType::TSArrayType(arr_type) => {
        format!("[{}]", arr_type.element_type.to_swift_type())
      }
      TSType::TSTypeReference(val) => val.type_name.to_string(),
      // Fallback (types, that we are not interested)
      _ => "Any".to_string(),
    }
  }
}

impl SwiftType for TSSignature<'_> {
  fn to_swift_type(&self) -> String {
    // map typescript signature to swift
    match self {
      TSSignature::TSPropertySignature(prop_sig) => {
        let key = prop_sig.key.to_swift_type();
        let var_or_let = "let";
        let type_annotation = prop_sig
          .type_annotation
          .as_ref()
          .map(|annotation| annotation.type_annotation.to_swift_type())
          .unwrap_or_else(|| "Any".to_string());

        format!(
          "{}{} {}: {}",
          INDENT_VALUE, var_or_let, key, type_annotation
        )
      }
      TSSignature::TSMethodSignature(val) => {
        let params = val
          .params
          .items
          .iter()
          .map(|param| {
            let type_annotation = param
              .pattern
              .type_annotation
              .as_ref()
              .map(|t| t.type_annotation.to_swift_type())
              .unwrap_or_else(|| "Any".to_string());

            format!(
              "{}: {}",
              param.pattern.kind.to_swift_type(),
              type_annotation
            )
          })
          .collect::<Vec<_>>()
          .join(", ");

        format!(
          "{}func {}({})",
          INDENT_VALUE,
          val.key.to_swift_type(),
          params
        )
      }
      _ => "unknown signature".to_owned(),
    }
  }
}

impl SwiftTransformer {
  pub fn transform(ast_program: &Program) -> String {
    let mut output = String::new();

    for statement in &ast_program.body {
      match statement {
        Statement::ExportNamedDeclaration(_export_decl) => {
          // println!("Found a export declaration: {:?}", export_decl.specifiers);
        }
        Statement::TSInterfaceDeclaration(interface_decl) => {
          let body_data = interface_decl
            .body
            .body
            .iter()
            .map(|signature| signature.to_swift_type())
            .collect::<Vec<_>>()
            .join("\n");

          let protocol_name: String = interface_decl.id.name.to_string();
          let code = format!("protocol {} {{\n{}\n}}\n", protocol_name, body_data);

          output.push_str(&code);
        }
        _ => {
          // ignore classes, functions, etc.. (we are only interesetd in types/interfaces)
        }
      }
    }

    output
  }
}
