use oxc_ast::ast::TSSignature;

use crate::languages::swift::swift_type_trait::SwiftType;

const INDENT_SPACE: &str = "  ";

pub trait SwiftStructType {
  fn to_swift_struct_type(&self) -> String;
}

impl SwiftStructType for TSSignature<'_> {
  fn to_swift_struct_type(&self) -> String {
    match self {
      TSSignature::TSPropertySignature(prop_sig) => {
        let type_annotation = prop_sig
          .type_annotation
          .as_ref()
          .map(|annotation| annotation.type_annotation.to_swift_type())
          .unwrap_or_default();

        let prop_name = prop_sig.key.to_swift_type();
        let optional = if prop_sig.optional { "?" } else { "" };
        let swift_prop_sig = format!("{}{}", type_annotation, optional);

        // by default all struct properties are `public`
        format!(
          "{}public let {}: {}",
          INDENT_SPACE, prop_name, swift_prop_sig
        )
      }
      _ => "".to_string(),
    }
  }
}
