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
  UnknownExpressionType,
  CardDoesNotExist,
  MalformedCampfireSetCommand,
  UnknownCampfireSetCommand,
  UnableToReadFooterFile
}

// TODO: When there's a specific error on a specific line, 
// we want an error emitter that will show us where the error is. 
#[allow(dead_code)]
pub fn campfire_error(err:CampfireError) -> &'static str {
  match err {
    CampfireError::MissingMainFile => "Cannot find start.campfire file!",
    CampfireError::UnknownExpressionType => "Unknown expression type found in file",
    CampfireError::CardDoesNotExist => "Found link to non-existent card",
    CampfireError::MalformedCampfireSetCommand => "The $set command does not adhere to the $set syntax: $set @<var_target> <value>",
    CampfireError::UnknownCampfireSetCommand => "That $set target does not exist! Omit the @ and you can set a custom var instead.",
    CampfireError::UnableToReadFooterFile => "A 'footer.html' template was found, but it could not be read.",
    _ => unreachable!()
  }
}

pub fn throw_general_campfire_error(msg:&str) {
  println!("ERROR: {}",msg);
  
  std::process::exit(0);
}