use oxc_ast::ast::{Expression, TSEnumDeclaration};

pub trait IsEnumWithInitializerType {
  fn is_enum_with_initializer_type(&self) -> bool;
  fn get_enum_display_type(&self) -> String;
}

pub trait GetEnumDisplayValue {
  fn get_enum_display_value(&self) -> String;
}

impl IsEnumWithInitializerType for TSEnumDeclaration<'_> {
  fn is_enum_with_initializer_type(&self) -> bool {
    let all_enum_string = self
      .members
      .iter()
      .all(|x| matches!(x.initializer, Some(Expression::StringLiteral(_))));

    if all_enum_string {
      return true;
    }

    let all_enum_numeric = self
      .members
      .iter()
      .all(|x| matches!(x.initializer, Some(Expression::NumericLiteral(_))));

    if all_enum_numeric {
      return true;
    }

    false
  }

  fn get_enum_display_type(&self) -> String {
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

impl GetEnumDisplayValue for Expression<'_> {
  fn get_enum_display_value(&self) -> String {
    match self {
      Expression::StringLiteral(string_enum) => format!("\"{}\"", string_enum.value),
      Expression::NumericLiteral(number_enum) => number_enum.value.to_string(),
      _ => "UNKNOWN".to_string(),
    }
  }
}
