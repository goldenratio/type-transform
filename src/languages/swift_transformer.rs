use oxc_ast::ast::{
  BindingPatternKind, Declaration, ExportNamedDeclaration, Program, PropertyKey, Statement,
  TSInterfaceDeclaration, TSPropertySignature, TSSignature, TSType, TSTypeReference,
};

pub struct SwiftTransformer;

const INDENT_SPACE: &str = "  ";

trait SwiftType {
  fn to_swift_type(&self) -> String;
}

trait SwiftFunctionReturnType {
  fn to_swift_fn_return_type(&self) -> String;
}

trait IsAsyncType {
  fn is_async_type(&self) -> bool;
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

impl IsAsyncType for TSTypeReference<'_> {
  fn is_async_type(&self) -> bool {
    let type_name = self.type_name.to_string();
    type_name == "Promise"
  }
}

impl SwiftType for TSTypeReference<'_> {
  fn to_swift_type(&self) -> String {
    let type_name = self.type_name.to_string();
    if type_name == "Promise" {
      let async_return_type = self
        .type_parameters
        .as_ref()
        .and_then(|x| x.params.first())
        .map(|x| x.to_swift_type())
        .unwrap_or_else(|| "Any".to_string());

      async_return_type
    } else {
      type_name
    }
  }
}

impl IsAsyncType for TSType<'_> {
  fn is_async_type(&self) -> bool {
    match self {
      TSType::TSTypeReference(val) => {
        let type_name = val.type_name.to_string();
        type_name == "Promise"
      }
      _ => false,
    }
  }
}
impl SwiftType for TSType<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      TSType::TSStringKeyword(_) => "String".to_string(),
      TSType::TSNumberKeyword(_) => "Double".to_string(),
      TSType::TSBooleanKeyword(_) => "Bool".to_string(),
      TSType::TSTypeReference(val) => val.to_swift_type(),
      TSType::TSVoidKeyword(_) => "Void".to_string(),
      _ => "Any".to_string(),
    }
  }
}

impl SwiftFunctionReturnType for TSType<'_> {
  fn to_swift_fn_return_type(&self) -> String {
    match self {
      TSType::TSStringKeyword(_) => " -> String".to_string(),
      TSType::TSNumberKeyword(_) => " -> Double".to_string(),
      TSType::TSBooleanKeyword(_) => " -> Bool".to_string(),
      TSType::TSTypeReference(val) => {
        let type_name = val.to_swift_type();
        if val.is_async_type() {
          format!(" async throws -> {}", type_name)
        } else {
          format!(" -> {}", type_name)
        }
      }
      TSType::TSVoidKeyword(_) => " -> Void".to_string(),
      _ => " -> Any".to_string(),
    }
  }
}

impl IsAsyncType for TSPropertySignature<'_> {
  fn is_async_type(&self) -> bool {
    self
      .type_annotation
      .as_ref()
      .map(|annotation| annotation.type_annotation.is_async_type())
      .unwrap_or_default()
  }
}

impl SwiftType for TSSignature<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      TSSignature::TSPropertySignature(prop_sig) => {
        let var = "var";
        let prop_name = prop_sig.key.to_swift_type();

        let type_annotation = prop_sig
          .type_annotation
          .as_ref()
          .map(|annotation| annotation.type_annotation.to_swift_type())
          .unwrap_or_default();

        let optional = if prop_sig.optional { "?" } else { "" };
        let get_set_value = if prop_sig.readonly { "get" } else { "get set" };
        let async_values = if prop_sig.is_async_type() {
          "async throw"
        } else {
          ""
        };

        let accessor_parts = [optional, "{", get_set_value, async_values, "}"].join(" ");
        let swift_prop_sig = format!("{}{}", type_annotation, accessor_parts);

        format!("{}{} {}: {}", INDENT_SPACE, var, prop_name, swift_prop_sig)
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
          .map(|r| r.type_annotation.to_swift_fn_return_type())
          .unwrap_or_else(|| "".to_string());

        let func_name = val.key.to_swift_type();
        format!(
          "{}func {}({}){}",
          INDENT_SPACE, func_name, params, return_type
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

    let protocol_name = self.id.name.to_string();
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
