use oxc_ast::ast::{
  BindingPatternKind, Declaration, ExportNamedDeclaration, Program, PropertyKey, Statement,
  TSInterfaceDeclaration, TSSignature, TSType,
};

pub struct SwiftTransformer;

const INDENT_SPACE: &str = "  ";

trait SwiftType {
  fn to_swift_type(&self) -> String;
}

impl SwiftType for PropertyKey<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      PropertyKey::StaticIdentifier(id_name) => id_name.name.to_string(),
      _ => "unknown-id_name".to_owned(),
    }
  }
}

impl SwiftType for BindingPatternKind<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      BindingPatternKind::BindingIdentifier(val) => val.name.to_string(),
      _ => "uknown-BindingPatternKind".to_owned(),
    }
  }
}

impl SwiftType for TSType<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      TSType::TSStringKeyword(_) => "String".to_string(),
      TSType::TSNumberKeyword(_) => "Double".to_string(),
      TSType::TSBooleanKeyword(_) => "Bool".to_string(),
      TSType::TSArrayType(arr_type) => {
        format!("[{}]", arr_type.element_type.to_swift_type())
      }
      TSType::TSTypeReference(val) => val.type_name.to_string(),
      TSType::TSVoidKeyword(_) => "Void".to_string(),
      _ => "Any".to_string(),
    }
  }
}

impl SwiftType for TSSignature<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      TSSignature::TSPropertySignature(prop_sig) => {
        let key = prop_sig.key.to_swift_type();
        let var = "var";
        let type_annotation = prop_sig
          .type_annotation
          .as_ref()
          .map(|annotation| annotation.type_annotation.to_swift_type())
          .unwrap_or_else(|| "Any".to_string());

        let optional = if prop_sig.optional { "?" } else { "" };
        let get_set_value = if prop_sig.readonly { "get" } else { "get set" };

        format!(
          "{}{} {}: {}{} {{ {} }}",
          INDENT_SPACE, var, key, type_annotation, optional, get_set_value
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

        let return_type = val
          .return_type
          .as_ref()
          .map(|r| format!(" -> {}", r.type_annotation.to_swift_type()))
          .unwrap_or_else(|| "".to_string());

        format!(
          "{}func {}({}){}",
          INDENT_SPACE,
          val.key.to_swift_type(),
          params,
          return_type
        )
      }
      _ => "unknown-signature".to_owned(),
    }
  }
}

impl SwiftType for Declaration<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      Declaration::TSInterfaceDeclaration(interface_decl) => interface_decl.to_swift_type(),
      _ => "unknown-declartion".to_string(),
    }
  }
}

impl SwiftType for TSInterfaceDeclaration<'_> {
  fn to_swift_type(&self) -> String {
    let body_data = self
      .body
      .body
      .iter()
      .map(|signature| signature.to_swift_type())
      .collect::<Vec<_>>()
      .join("\n");

    let protocol_name: String = self.id.name.to_string();
    format!("protocol {} {{\n{}\n}}\n\n", protocol_name, body_data)
  }
}

impl SwiftType for ExportNamedDeclaration<'_> {
  fn to_swift_type(&self) -> String {
    self
      .declaration
      .as_ref()
      .map(|d| d.to_swift_type())
      .unwrap_or_else(|| "unknown-export-named-declaration".to_string())
  }
}

impl SwiftType for Statement<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      Statement::ExportNamedDeclaration(export_decl) => export_decl.to_swift_type(),
      Statement::TSInterfaceDeclaration(interface_decl) => interface_decl.to_swift_type(),
      _ => "uknown-statement".to_string(),
    }
  }
}

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
