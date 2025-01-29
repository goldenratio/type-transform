mod languages;

use std::{fs, path::Path};

use clap::Parser;

use languages::{swift_transformer::SwiftTransformer, transformer::LanguageTransformer};
use oxc_allocator::Allocator;
use oxc_parser::{ParseOptions, Parser as OxcParser};
use oxc_span::SourceType;

/// Convert TypeScript types to swift,kotlin, etc..
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Input file name
  src: String,

  /// The output file. Determines --lang and --top-level.
  #[arg(short, long)]
  out: String,
}

fn main() {
  let args = Args::parse();

  let path = Path::new(&args.src);
  let source_text = fs::read_to_string(path)
    .map_err(|_| format!("Unable to read source file: {}", &args.src))
    .unwrap();

  let source_type = SourceType::from_path(path).unwrap();

  let allocator = Allocator::default();
  let ret = OxcParser::new(&allocator, &source_text, source_type)
    .with_options(ParseOptions {
      ..ParseOptions::default()
    })
    .parse();

  let program = ret.program;

  // println!("Comments:");
  // for comment in &program.comments {
  //   let s = comment.content_span().source_text(&source_text);
  //   println!("{s}");
  // }

  println!("AST:");
  println!("{}", serde_json::to_string_pretty(&program).unwrap());

  if ret.errors.is_empty() {
    let transformed_code = SwiftTransformer::transform(&program);
    println!("transformed code,\n{:?}", transformed_code);
  } else {
    for error in ret.errors {
      let error = error.with_source_code(source_text.clone());
      println!("{error:?}");
      println!("Parsed with Errors.");
    }
  }
}
