use oxc_ast::ast::{
  BindingPatternKind, Declaration, ExportNamedDeclaration, FormalParameters, PropertyKey,
  Statement, TSEnumDeclaration, TSEnumMember, TSEnumMemberName, TSFunctionType,
  TSInterfaceDeclaration, TSSignature, TSType, TSTypeReference,
};

use crate::languages::{
  kotlin::{kotlin_enum_display_type_trait::KotlinEnumDisplayType, kotlin_style},
  shared::{
    enum_trait::{GetEnumDisplayValue, IsEnumWithInitializerType},
    is_async_trait::IsAsyncType,
  },
};

use super::kotlin_is_interface_type_trait::KotlinIsInterfaceType;

pub trait KotlinType {
  fn to_kotlin_type(&self) -> String;
}

impl KotlinType for PropertyKey<'_> {
  fn to_kotlin_type(&self) -> String {
    match self {
      PropertyKey::StaticIdentifier(id_name) => id_name.name.to_string(),
      PropertyKey::Identifier(id_name) => id_name.to_string(),
      _ => "unknown-PropertyKey".to_owned(),
    }
  }
}

impl KotlinType for BindingPatternKind<'_> {
  fn to_kotlin_type(&self) -> String {
    match self {
      BindingPatternKind::BindingIdentifier(val) => val.name.to_string(),
      _ => "unknown-BindingPatternKind".to_owned(),
    }
  }
}

impl KotlinType for TSTypeReference<'_> {
  fn to_kotlin_type(&self) -> String {
    let type_name = self.type_name.to_string();
    match type_name.as_str() {
      "Promise" => self
        .type_parameters
        .as_ref()
        .and_then(|x| x.params.first())
        .map(|x| x.to_kotlin_type())
        .unwrap_or_else(|| "Any".into()),

      "Array" | "ReadonlyArray" => format!(
        "List<{}>",
        self
          .type_parameters
          .as_ref()
          .and_then(|x| x.params.first())
          .map(|x| x.to_kotlin_type())
          .unwrap_or_else(|| "Any".into())
      ),

      "Record" | "Map" | "ReadonlyMap" => {
        let key_str = self
          .type_parameters
          .as_ref()
          .and_then(|x| x.params.first())
          .map(|x| x.to_kotlin_type())
          .unwrap_or_else(|| "Any".into());

        let val_str = self
          .type_parameters
          .as_ref()
          .and_then(|x| x.params.get(1))
          .map(|x| x.to_kotlin_type())
          .unwrap_or_else(|| "Any".into());

        format!("Map<{key_str}, {val_str}>")
      }

      "Set" => {
        let val_str = self
          .type_parameters
          .as_ref()
          .and_then(|x| x.params.first())
          .map(|x| x.to_kotlin_type())
          .unwrap_or_else(|| "Any".into());
        format!("{type_name}<{val_str}>")
      }

      _ => type_name,
    }
  }
}

impl KotlinType for TSFunctionType<'_> {
  /// this is invoked from second level functions
  fn to_kotlin_type(&self) -> String {
    let type_name = self.return_type.type_annotation.to_kotlin_type();
    let fn_params = self.params.to_kotlin_type();

    format!("({fn_params}) -> {type_name}")
  }
}

impl KotlinType for TSType<'_> {
  fn to_kotlin_type(&self) -> String {
    match self {
      TSType::TSStringKeyword(_) => "String".to_string(),
      TSType::TSNumberKeyword(_) => "Double".to_string(),
      TSType::TSBooleanKeyword(_) => "Boolean".to_string(),
      TSType::TSVoidKeyword(_) => "Unit".to_string(),
      TSType::TSObjectKeyword(_) => "Map<String, Any>".to_string(),
      TSType::TSTypeReference(val) => val.to_kotlin_type(),
      TSType::TSFunctionType(fn_type) => fn_type.to_kotlin_type(),
      TSType::TSArrayType(array_type) => {
        let el_type = array_type.element_type.to_kotlin_type();
        format!("List<{el_type}>")
      }
      TSType::TSTypeOperatorType(op_type) => op_type.type_annotation.to_kotlin_type(),
      _ => "Any".to_string(),
    }
  }
}

impl KotlinType for FormalParameters<'_> {
  fn to_kotlin_type(&self) -> String {
    self
      .items
      .iter()
      .map(|param| {
        let type_annotation = param
          .pattern
          .type_annotation
          .as_ref()
          .map(|t| t.type_annotation.to_kotlin_type())
          .unwrap_or_else(|| "Any".to_string());

        format!(
          "{}: {}",
          param.pattern.kind.to_kotlin_type(),
          type_annotation
        )
      })
      .collect::<Vec<_>>()
      .join(", ")
  }
}

impl KotlinType for TSSignature<'_> {
  fn to_kotlin_type(&self) -> String {
    match self {
      TSSignature::TSPropertySignature(prop_sig) => {
        let prop_name = prop_sig.key.to_kotlin_type();
        let readonly = if prop_sig.readonly { "val" } else { "var" };

        // If property is a arrow function
        if let Some(annotation) = prop_sig.type_annotation.as_ref() {
          if let TSType::TSFunctionType(fn_type) = &annotation.type_annotation {
            let fn_return_type = fn_type.return_type.type_annotation.to_kotlin_type();
            let fn_params = fn_type.params.to_kotlin_type();
            let async_val = if fn_type.return_type.type_annotation.is_async_type() {
              "suspend "
            } else {
              ""
            };

            return format!(
              "{}{} {}: {}({}) -> {}",
              kotlin_style::INDENT_SPACE,
              readonly,
              prop_name,
              async_val,
              fn_params,
              fn_return_type
            );
          }
        }

        let type_annotation = prop_sig
          .type_annotation
          .as_ref()
          .map(|annotation| annotation.type_annotation.to_kotlin_type())
          .unwrap_or_default();

        let prop_return_type = if prop_sig.is_async_type() {
          // TODO: `import kotlinx.coroutines.Deferred` should included in banner
          format!("Deferred<{type_annotation}>")
        } else {
          type_annotation.to_string()
        };

        format!(
          "{}{} {}: {}",
          kotlin_style::INDENT_SPACE,
          readonly,
          prop_name,
          prop_return_type
        )
      }
      TSSignature::TSMethodSignature(method_sig) => {
        let params = method_sig.params.to_kotlin_type();

        let return_type = method_sig
          .return_type
          .as_ref()
          .map(|r| r.type_annotation.to_kotlin_type())
          .unwrap_or_else(|| "".to_string());

        let func_name = method_sig.key.to_kotlin_type();
        let async_val = if method_sig.is_async_type() {
          "suspend "
        } else {
          ""
        };
        format!(
          "{}{}fun {}({}): {}",
          kotlin_style::INDENT_SPACE,
          async_val,
          func_name,
          params,
          return_type
        )
      }
      _ => "// unknown-signature".to_string(),
    }
  }
}

impl KotlinType for TSInterfaceDeclaration<'_> {
  fn to_kotlin_type(&self) -> String {
    let interface_name = self.id.name.to_string();

    let is_interface = self.is_kotlin_interface_type();

    if is_interface {
      let body_data = self
        .body
        .body
        .iter()
        .map(|signature| signature.to_kotlin_type())
        .collect::<Vec<_>>()
        .join("\n");

      format!("interface {interface_name} {{\n{body_data}\n}}\n\n")
    } else {
      let body_data = self
        .body
        .body
        .iter()
        .map(|signature| signature.to_kotlin_type())
        .collect::<Vec<_>>()
        .join(",\n");
      format!("data class {interface_name} (\n{body_data}\n)\n\n")
    }
  }
}

impl KotlinType for Declaration<'_> {
  fn to_kotlin_type(&self) -> String {
    match self {
      Declaration::TSInterfaceDeclaration(interface_decl) => interface_decl.to_kotlin_type(),
      Declaration::TSEnumDeclaration(enum_decl) => enum_decl.to_kotlin_type(),
      _ => "// unknown-declaration".to_string(),
    }
  }
}

impl KotlinType for ExportNamedDeclaration<'_> {
  fn to_kotlin_type(&self) -> String {
    self
      .declaration
      .as_ref()
      .map(|d| d.to_kotlin_type())
      .unwrap_or_else(|| "// unknown-export-named-declaration".to_string())
  }
}

impl KotlinType for TSEnumMember<'_> {
  fn to_kotlin_type(&self) -> String {
    match &self.id {
      TSEnumMemberName::Identifier(enum_id) => enum_id.to_string(),
      TSEnumMemberName::String(enum_string) => enum_string.to_string(),
    }
  }
}

impl KotlinType for TSEnumDeclaration<'_> {
  fn to_kotlin_type(&self) -> String {
    let enum_name = self.id.to_string();
    if self.is_enum_with_initializer_type() {
      let enum_cases: String = self
        .members
        .iter()
        .map(|x| {
          format!(
            "{}{}({})",
            kotlin_style::INDENT_SPACE,
            x.to_kotlin_type(),
            x.initializer
              .as_ref()
              .expect("Unable get initializer value from enum")
              .get_enum_display_value()
          )
        })
        .collect::<Vec<_>>()
        .join("\n");
      let enum_type = self.to_kotlin_enum_display_type();
      format!("enum class {enum_name}(val value: {enum_type}) {{ \n{enum_cases}\n}}\n")
    } else {
      let enum_cases: String = self
        .members
        .iter()
        .map(|x| format!("{}{}", kotlin_style::INDENT_SPACE, x.to_kotlin_type()))
        .collect::<Vec<_>>()
        .join("\n");

      format!("enum class {enum_name} {{ \n{enum_cases}\n}}\n")
    }
  }
}

impl KotlinType for Statement<'_> {
  fn to_kotlin_type(&self) -> String {
    match self {
      Statement::ExportNamedDeclaration(export_decl) => export_decl.to_kotlin_type(),
      Statement::TSInterfaceDeclaration(interface_decl) => interface_decl.to_kotlin_type(),
      Statement::TSEnumDeclaration(enum_decl) => enum_decl.to_kotlin_type(),
      _ => "// unknown-statement\n\n".to_string(),
    }
  }
}
