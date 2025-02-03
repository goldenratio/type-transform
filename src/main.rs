mod languages;
mod utils;

use std::{fs, path::Path};

use clap::Parser;

use languages::language_factory::LanguageFactory;
use oxc_allocator::Allocator;
use oxc_parser::{ParseOptions, Parser as OxcParser};
use oxc_span::SourceType;
use utils::{content_utils::get_content_banner_header, file_utils::get_language_from_file_name};

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
  let destination_language =
    get_language_from_file_name(&args.out).expect("unable to detect target language from fileName");

  // println!("destionation_language: {:?}", destination_language);

  // for comment in &program.comments {
  //   let s = comment.content_span().source_text(&source_text);
  //   println!("{s}");
  // }

  println!("AST:");
  println!("{}", serde_json::to_string_pretty(&program).unwrap());

  if ret.errors.is_empty() {
    let transformed_code = LanguageFactory::transform(destination_language, &program);
    let out_path = Path::new(&args.out);
    let banner_header = get_content_banner_header();
    let updated_content = format!("{}\n{}", banner_header, transformed_code);
    let res = fs::write(out_path, updated_content);
    match res {
      Ok(_) => println!("success"),
      Err(_) => println!("failed to write to file!!"),
    }
  } else {
    for error in ret.errors {
      let error = error.with_source_code(source_text.clone());
      println!("{error:?}");
      println!("Parsed with Errors.");
    }
  }
}
