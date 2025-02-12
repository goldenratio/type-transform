use oxc_ast::ast::{TSMethodSignature, TSPropertySignature, TSType, TSTypeReference};

pub trait IsAsyncType {
  fn is_async_type(&self) -> bool;
}

impl IsAsyncType for TSTypeReference<'_> {
  fn is_async_type(&self) -> bool {
    let type_name = self.type_name.to_string();
    type_name == "Promise"
  }
}

impl IsAsyncType for TSType<'_> {
  fn is_async_type(&self) -> bool {
    match self {
      TSType::TSTypeReference(val) => val.is_async_type(),
      _ => false,
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

impl IsAsyncType for TSMethodSignature<'_> {
  fn is_async_type(&self) -> bool {
    self
      .return_type
      .as_ref()
      .map(|r| r.type_annotation.is_async_type())
      .unwrap_or_default()
  }
}
