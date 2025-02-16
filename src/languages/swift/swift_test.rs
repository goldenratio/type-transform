#[cfg(test)]
mod tests {

  use oxc_allocator::Allocator;
  use oxc_parser::{ParseOptions, Parser};
  use oxc_span::SourceType;

  use crate::languages::swift::swift_transformer::SwiftTransformer;

  #[test]
  fn it_converts_to_swift_type() {
    let source_text = r#"
    interface HelloWorld {
      readonly color: string;
      getUser(): User;
    }

    interface User {
      readonly name: string;
    }
    "#;
    let source_type = SourceType::ts();
    let allocator = Allocator::default();

    let parser_ret = Parser::new(&allocator, source_text, source_type)
      .with_options(ParseOptions {
        ..ParseOptions::default()
      })
      .parse();

    let program = parser_ret.program;
    let transformed_code = SwiftTransformer::transform(&program);
    println!("{}", transformed_code);

    let expected_code = r#"protocol HelloWorld {
  var color: String { get }
  func getUser() -> User
}

struct User {
  let name: String
}

"#;
    assert_eq!(transformed_code, expected_code);
  }
}
