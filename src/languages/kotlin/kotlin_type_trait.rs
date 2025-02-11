use oxc_ast::ast::{
  PropertyKey, Statement, TSInterfaceDeclaration, TSSignature, TSType, TSTypeReference,
};

const INDENT_SPACE: &str = "  ";

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

      "Array" => format!(
        "[{}]",
        self
          .type_parameters
          .as_ref()
          .and_then(|x| x.params.first())
          .map(|x| x.to_kotlin_type())
          .unwrap_or_else(|| "Any".into())
      ),

      "Record" | "Map" => {
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

        format!("[{}: {}]", key_str, val_str)
      }

      "Set" => {
        let val_str = self
          .type_parameters
          .as_ref()
          .and_then(|x| x.params.first())
          .map(|x| x.to_kotlin_type())
          .unwrap_or_else(|| "Any".into());
        format!("{}<{}>", type_name, val_str)
      }

      _ => type_name,
    }
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
      _ => "Any".to_string(),
    }
  }
}

impl KotlinType for TSSignature<'_> {
  fn to_kotlin_type(&self) -> String {
    match self {
      TSSignature::TSPropertySignature(prop_sig) => {
        let prop_name = prop_sig.key.to_kotlin_type();
        let readonly = if prop_sig.readonly { "val" } else { "var" };
        let type_annotation = prop_sig
          .type_annotation
          .as_ref()
          .map(|annotation| annotation.type_annotation.to_kotlin_type())
          .unwrap_or_default();

        format!(
          "{}{} {}: {}",
          INDENT_SPACE, readonly, prop_name, type_annotation
        )
      }
      _ => "// unknown-signature".to_string(),
    }
  }
}

impl KotlinType for TSInterfaceDeclaration<'_> {
  fn to_kotlin_type(&self) -> String {
    let interface_name = self.id.name.to_string();

    let body_data = self
      .body
      .body
      .iter()
      .map(|signature| signature.to_kotlin_type())
      .collect::<Vec<_>>()
      .join("\n");

    format!("interface {} {{\n{}\n}}\n\n", interface_name, body_data)
  }
}

impl KotlinType for Statement<'_> {
  fn to_kotlin_type(&self) -> String {
    match self {
      Statement::TSInterfaceDeclaration(interface_decl) => interface_decl.to_kotlin_type(),
      _ => "// unknown-statement".to_string(),
    }
  }
}
