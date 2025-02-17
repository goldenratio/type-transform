use oxc_ast::ast::{Expression, TSEnumDeclaration};

use crate::languages::shared::enum_trait::GetEnumDisplayValue;

pub trait KotlinEnumDisplayType {
  fn to_kotlin_enum_display_type(&self) -> String;
}

impl KotlinEnumDisplayType for TSEnumDeclaration<'_> {
  fn to_kotlin_enum_display_type(&self) -> String {
    let all_enum_string = self
      .members
      .iter()
      .all(|x| matches!(x.initializer, Some(Expression::StringLiteral(_))));

    if all_enum_string {
      return "String".to_string();
    }

    let all_enum_double = self.members.iter().all(|x| {
      matches!(x.initializer, Some(Expression::NumericLiteral(_)))
        && x
          .initializer
          .as_ref()
          .expect("Unable to get enum initializer value for get_enum_display_type")
          .get_enum_display_value()
          .contains(".")
    });

    if all_enum_double {
      return "Double".to_string();
    }

    "Int".to_string()
  }
}
