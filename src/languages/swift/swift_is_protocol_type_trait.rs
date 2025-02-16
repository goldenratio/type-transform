use oxc_ast::ast::{TSInterfaceDeclaration, TSSignature, TSType};

pub trait SwiftIsProtoclType {
  fn is_swift_protocol_type(&self) -> bool;
}

impl SwiftIsProtoclType for TSInterfaceDeclaration<'_> {
  fn is_swift_protocol_type(&self) -> bool {
    let is_protocol = self.body.body.iter().any(|x| match x {
      TSSignature::TSMethodSignature(_) => true,
      TSSignature::TSPropertySignature(prop_sig) => {
        if let Some(type_annotation) = &prop_sig.type_annotation {
          matches!(type_annotation.type_annotation, TSType::TSFunctionType(_))
        } else {
          false
        }
      }
      _ => false,
    });
    is_protocol
  }
}
