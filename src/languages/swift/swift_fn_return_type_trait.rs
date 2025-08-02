use oxc_ast::ast::TSType;

use crate::languages::shared::is_async_trait::IsAsyncType;

use super::swift_type_trait::SwiftType;

pub trait SwiftFunctionReturnType {
  fn to_swift_fn_return_type(&self) -> String;
}

impl SwiftFunctionReturnType for TSType<'_> {
  fn to_swift_fn_return_type(&self) -> String {
    match self {
      TSType::TSStringKeyword(_) => " -> String".to_string(),
      TSType::TSNumberKeyword(_) => " -> Double".to_string(),
      TSType::TSBooleanKeyword(_) => " -> Bool".to_string(),
      TSType::TSVoidKeyword(_) => " -> Void".to_string(),
      TSType::TSTypeReference(val) => {
        let type_name = val.to_swift_type();
        if val.is_async_type() {
          format!(" async throws -> {type_name}")
        } else {
          format!(" -> {type_name}")
        }
      }
      TSType::TSFunctionType(val) => {
        format!(" -> {}", val.to_swift_type())
      }
      _ => " -> Any".to_string(),
    }
  }
}
