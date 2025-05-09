use oxc_ast::ast::{
  BindingPatternKind, Declaration, ExportNamedDeclaration, FormalParameters, PropertyKey,
  Statement, TSEnumDeclaration, TSEnumMember, TSEnumMemberName, TSFunctionType,
  TSInterfaceDeclaration, TSSignature, TSType, TSTypeReference,
};

use crate::languages::{
  shared::{
    enum_trait::{GetEnumDisplayValue, IsEnumWithInitializerType},
    is_async_trait::IsAsyncType,
  },
  swift::{
    swift_enum_display_type_trait::SwiftEnumDisplayType,
    swift_fn_return_type_trait::SwiftFunctionReturnType, swift_struct_type_trait::SwiftStructType,
    swift_style,
  },
};

use super::swift_is_protocol_type_trait::SwiftIsProtoclType;

pub trait SwiftType {
  fn to_swift_type(&self) -> String;
}

impl SwiftType for PropertyKey<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      PropertyKey::StaticIdentifier(id_name) => id_name.name.to_string(),
      PropertyKey::Identifier(id_name) => id_name.to_string(),
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

      "Array" | "ReadonlyArray" => format!(
        "[{}]",
        self
          .type_parameters
          .as_ref()
          .and_then(|x| x.params.first())
          .map(|x| x.to_swift_type())
          .unwrap_or_else(|| "Any".into())
      ),

      "Record" | "Map" | "ReadonlyMap" => {
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

      "Set" => {
        let val_str = self
          .type_parameters
          .as_ref()
          .and_then(|x| x.params.first())
          .map(|x| x.to_swift_type())
          .unwrap_or_else(|| "Any".into());
        format!("{}<{}>", type_name, val_str)
      }

      _ => type_name,
    }
  }
}

impl SwiftType for TSFunctionType<'_> {
  /// this is invoked from second level functions
  fn to_swift_type(&self) -> String {
    let type_name = self.return_type.type_annotation.to_swift_type();
    let fn_params = self.params.to_swift_type();

    // Function types cannot have argument labels; use '_' before 'name'
    let ignore_arg_labels = if !fn_params.is_empty() { "_ " } else { "" };

    format!("({}{}) -> {}", ignore_arg_labels, fn_params, type_name)
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
      TSType::TSTypeReference(ref_type) => ref_type.to_swift_type(),
      TSType::TSFunctionType(fn_type) => fn_type.to_swift_type(),
      TSType::TSArrayType(array_type) => {
        let el_type = array_type.element_type.to_swift_type();
        format!("[{}]", el_type)
      }
      TSType::TSTypeOperatorType(op_type) => op_type.type_annotation.to_swift_type(),
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
        let optional = if prop_sig.optional { "?" } else { "" };

        // If property is a arrow function
        if let Some(annotation) = prop_sig.type_annotation.as_ref() {
          if let TSType::TSFunctionType(fn_type) = &annotation.type_annotation {
            let fn_return_type = fn_type
              .return_type
              .type_annotation
              .to_swift_fn_return_type();

            let fn_params = fn_type.params.to_swift_type();

            return format!(
              "{}func {}({}){}{}",
              swift_style::INDENT_SPACE,
              prop_name,
              fn_params,
              fn_return_type,
              optional
            );
          }
        }

        let type_annotation = prop_sig
          .type_annotation
          .as_ref()
          .map(|annotation| annotation.type_annotation.to_swift_type())
          .unwrap_or_default();

        let is_async = prop_sig.is_async_type();

        let get_set_value = if prop_sig.readonly || is_async {
          "get"
        } else {
          "get set"
        };
        let async_values = if is_async { " async throws" } else { "" };

        let accessor_parts = format!("{} {{ {}{} }}", optional, get_set_value, async_values);
        let swift_prop_sig = format!("{}{}", type_annotation, accessor_parts);

        format!(
          "{}var {}: {}",
          swift_style::INDENT_SPACE,
          prop_name,
          swift_prop_sig
        )
      }
      TSSignature::TSMethodSignature(method_sig) => {
        let params = method_sig.params.to_swift_type();

        let return_type = method_sig
          .return_type
          .as_ref()
          .map(|r| r.type_annotation.to_swift_fn_return_type())
          .unwrap_or_else(|| "".to_string());

        let func_name = method_sig.key.to_swift_type();
        format!(
          "{}func {}({}){}",
          swift_style::INDENT_SPACE,
          func_name,
          params,
          return_type
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
      Declaration::TSEnumDeclaration(enum_decl) => enum_decl.to_swift_type(),
      _ => "// unknown-declartion".to_string(),
    }
  }
}

impl SwiftType for TSInterfaceDeclaration<'_> {
  fn to_swift_type(&self) -> String {
    let is_protocol = self.is_swift_protocol_type();
    let interface_name = self.id.name.to_string();

    if is_protocol {
      let body_data = self
        .body
        .body
        .iter()
        .map(|signature| signature.to_swift_type())
        .collect::<Vec<_>>()
        .join("\n");

      format!("protocol {} {{\n{}\n}}\n\n", interface_name, body_data)
    } else {
      let body_data = self
        .body
        .body
        .iter()
        .map(|signature| signature.to_swift_struct_type())
        .collect::<Vec<_>>()
        .join("\n");

      format!("struct {} {{\n{}\n}}\n\n", interface_name, body_data)
    }
  }
}

impl SwiftType for ExportNamedDeclaration<'_> {
  fn to_swift_type(&self) -> String {
    self
      .declaration
      .as_ref()
      .map(|d| d.to_swift_type())
      .map(|d| {
        if d.starts_with("//") {
          d
        } else {
          format!("public {}", d)
        }
      })
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

    if self.is_enum_with_initializer_type() {
      let enum_cases: String = self
        .members
        .iter()
        .map(|x| {
          format!(
            "{}case {} = {}",
            swift_style::INDENT_SPACE,
            x.to_swift_type(),
            x.initializer
              .as_ref()
              .expect("Unable get initializer value from enum")
              .get_enum_display_value()
          )
        })
        .collect::<Vec<_>>()
        .join("\n");
      let enum_type = self.to_swift_enum_display_type();
      format!(
        "enum {}: {}, CaseIterable {{ \n{}\n}}\n",
        enum_name, enum_type, enum_cases
      )
    } else {
      let enum_cases: String = self
        .members
        .iter()
        .map(|x| format!("{}case {}", swift_style::INDENT_SPACE, x.to_swift_type()))
        .collect::<Vec<_>>()
        .join("\n");

      format!(
        "enum {}: Int, CaseIterable {{ \n{}\n}}\n",
        enum_name, enum_cases
      )
    }
  }
}

impl SwiftType for Statement<'_> {
  fn to_swift_type(&self) -> String {
    match self {
      Statement::ExportNamedDeclaration(export_decl) => export_decl.to_swift_type(),
      Statement::TSInterfaceDeclaration(interface_decl) => interface_decl.to_swift_type(),
      Statement::TSEnumDeclaration(enum_decl) => enum_decl.to_swift_type(),
      _ => "// unknown-statement\n\n".to_string(),
    }
  }
}
