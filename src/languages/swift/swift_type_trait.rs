use oxc_ast::ast::{
  BindingPatternKind, Declaration, ExportNamedDeclaration, FormalParameters, PropertyKey,
  Statement, TSEnumDeclaration, TSEnumMember, TSEnumMemberName, TSInterfaceDeclaration,
  TSSignature, TSType, TSTypeReference,
};

use crate::languages::swift::{
  swift_fn_return_type_trait::SwiftFunctionReturnType, swift_is_async_trait::IsAsyncType,
};

const INDENT_SPACE: &str = "  ";

pub trait SwiftType {
  fn to_swift_type(&self) -> String;
}

impl SwiftType for PropertyKey<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      PropertyKey::StaticIdentifier(id_name) => id_name.name.to_string(),
      _ => "unknown-PropertyKey".to_owned(),
    }
  }
}

impl SwiftType for BindingPatternKind<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      BindingPatternKind::BindingIdentifier(val) => val.name.to_string(),
      _ => "unknown-BindingPatternKind".to_owned(),
    }
  }
}

impl SwiftType for TSTypeReference<'_> {
  fn to_swift_type(&self) -> String {
    let type_name = self.type_name.to_string();
    match type_name.as_str() {
      "Promise" => self
        .type_parameters
        .as_ref()
        .and_then(|x| x.params.first())
        .map(|x| x.to_swift_type())
        .unwrap_or_else(|| "Any".into()),

      "Array" => format!(
        "[{}]",
        self
          .type_parameters
          .as_ref()
          .and_then(|x| x.params.first())
          .map(|x| x.to_swift_type())
          .unwrap_or_else(|| "Any".into())
      ),

      "Record" | "Map" => {
        let key_str = self
          .type_parameters
          .as_ref()
          .and_then(|x| x.params.first())
          .map(|x| x.to_swift_type())
          .unwrap_or_else(|| "Any".into());

        let val_str = self
          .type_parameters
          .as_ref()
          .and_then(|x| x.params.get(1))
          .map(|x| x.to_swift_type())
          .unwrap_or_else(|| "Any".into());

        format!("[{}: {}]", key_str, val_str)
      }

      _ => type_name,
    }
  }
}

impl SwiftType for TSType<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      TSType::TSStringKeyword(_) => "String".to_string(),
      TSType::TSNumberKeyword(_) => "Double".to_string(),
      TSType::TSBooleanKeyword(_) => "Bool".to_string(),
      TSType::TSVoidKeyword(_) => "Void".to_string(),
      TSType::TSObjectKeyword(_) => "[String: Any]".to_string(),
      TSType::TSTypeReference(val) => val.to_swift_type(),
      _ => "Any".to_string(),
    }
  }
}

impl SwiftType for FormalParameters<'_> {
  fn to_swift_type(&self) -> String {
    self
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
      .join(", ")
  }
}

impl SwiftType for TSSignature<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      TSSignature::TSPropertySignature(prop_sig) => {
        let prop_name = prop_sig.key.to_swift_type();

        let is_arrow_function_property = match prop_sig.type_annotation.as_ref() {
          Some(val) => matches!(val.type_annotation, TSType::TSFunctionType(_)),
          None => false,
        };

        if is_arrow_function_property {
          let fn_return_type = prop_sig
            .type_annotation
            .as_ref()
            .map(|r| r.type_annotation.to_swift_fn_return_type())
            .unwrap_or_else(|| "".to_string());

          let params = match prop_sig.type_annotation.as_ref() {
            Some(val) => match &val.type_annotation {
              TSType::TSFunctionType(fn_type) => fn_type.params.to_swift_type(),
              _ => "".to_string(),
            },
            None => "".to_string(),
          };

          format!(
            "{}func {}({}){}",
            INDENT_SPACE, prop_name, params, fn_return_type
          )
        } else {
          let type_annotation = prop_sig
            .type_annotation
            .as_ref()
            .map(|annotation| annotation.type_annotation.to_swift_type())
            .unwrap_or_default();

          let optional = if prop_sig.optional { "?" } else { "" };
          let get_set_value = if prop_sig.readonly { "get" } else { "get set" };
          let async_values = if prop_sig.is_async_type() {
            " async throw"
          } else {
            ""
          };

          let accessor_parts = format!("{} {{ {}{} }}", optional, get_set_value, async_values);
          let swift_prop_sig = format!("{}{}", type_annotation, accessor_parts);

          format!("{}var {}: {}", INDENT_SPACE, prop_name, swift_prop_sig)
        }
      }
      TSSignature::TSMethodSignature(val) => {
        let params = val.params.to_swift_type();

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
      _ => "// unknown-signature".to_owned(),
    }
  }
}

impl SwiftType for Declaration<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      Declaration::TSInterfaceDeclaration(interface_decl) => interface_decl.to_swift_type(),
      _ => "// unknown-declartion".to_string(),
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
    format!("\nprotocol {} {{\n{}\n}}\n", protocol_name, body_data)
  }
}

impl SwiftType for ExportNamedDeclaration<'_> {
  fn to_swift_type(&self) -> String {
    self
      .declaration
      .as_ref()
      .map(|d| d.to_swift_type())
      .unwrap_or_else(|| "// unknown-export-named-declaration".to_string())
  }
}

impl SwiftType for TSEnumMember<'_> {
  fn to_swift_type(&self) -> String {
    match &self.id {
      TSEnumMemberName::Identifier(enum_id) => enum_id.to_string(),
      TSEnumMemberName::String(enum_string) => enum_string.to_string(),
    }
  }
}

impl SwiftType for TSEnumDeclaration<'_> {
  fn to_swift_type(&self) -> String {
    let enum_name = self.id.to_string();
    let enum_cases: String = self
      .members
      .iter()
      .map(|x| format!("{}case {}", INDENT_SPACE, x.to_swift_type()))
      .collect::<Vec<_>>()
      .join("\n");

    format!("\nenum {} {{ \n{}\n}}\n", enum_name, enum_cases)
  }
}

impl SwiftType for Statement<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      Statement::ExportNamedDeclaration(export_decl) => export_decl.to_swift_type(),
      Statement::TSInterfaceDeclaration(interface_decl) => interface_decl.to_swift_type(),
      Statement::TSEnumDeclaration(enum_decl) => enum_decl.to_swift_type(),
      _ => "// unknown-statement".to_string(),
    }
  }
}
