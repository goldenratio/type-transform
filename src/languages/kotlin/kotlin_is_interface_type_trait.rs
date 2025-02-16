use oxc_ast::ast::{TSInterfaceDeclaration, TSSignature, TSType};

pub trait KotlinIsInterfaceType {
  fn is_kotlin_interface_type(&self) -> bool;
}

impl KotlinIsInterfaceType for TSInterfaceDeclaration<'_> {
  fn is_kotlin_interface_type(&self) -> bool {
    let is_interface = self.body.body.iter().any(|x| match x {
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
    is_interface
  }
}
