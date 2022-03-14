/*pub struct BonesError {
  id: u16,
  message: String
}*/

#[allow(unused_imports)]
use std;

pub static BONES_ERROR_MISSING_MAIN_FILE: &'static str = "Missing main.bn file.";

pub enum BonesError {
  MissingMainFile,
  UnableToParseBonesFile,
}

// TODO: When there's a specific error on a specific line, 
// we want an error emitter that will show us where the error is. 
#[allow(dead_code)]
pub fn emit_bones_error() -> String {
  // Lookup error and display
  let result:String = String::from("TODO");
  return result as String;
  
}

pub fn throw_general_bones_error(msg:&str) {
  println!("ERROR: {}",msg);
  
  std::process::exit(0);
}