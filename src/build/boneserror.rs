/*pub struct BonesError {
  id: u16,
  message: String
}*/

#[allow(unused_imports)]
use std;

pub static BONES_ERROR_MISSING_MAIN_FILE: &'static str = "Missing main.bn file.";

#[derive(Debug)]
pub enum CampfireError {
  MissingMainFile,
  ParseError,
  CampfireExpressionCompileError,
}

// TODO: When there's a specific error on a specific line, 
// we want an error emitter that will show us where the error is. 
#[allow(dead_code)]
pub fn campfire_error(err:CampfireError) -> &'static str {
  match err {
    CampfireError::MissingMainFile => "Cannot find start.campfire file!",
    CampfireError::ParseError => "Parse error",
    CampfireError::CampfireExpressionCompileError => "Error parsing Campfire expression",
    _ => unreachable!()
  }
}

pub fn throw_general_campfire_error(msg:&str) {
  println!("ERROR: {}",msg);
  
  std::process::exit(0);
}