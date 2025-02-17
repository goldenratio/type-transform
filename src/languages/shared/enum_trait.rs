use oxc_ast::ast::{Expression, TSEnumDeclaration};

pub trait IsEnumWithInitializerType {
  fn is_enum_with_initializer_type(&self) -> bool;
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
