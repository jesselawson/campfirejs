

mod card;
#[allow(unused_imports)]
use card::*;
mod boneserror;
use boneserror::*;

mod parser;
#[allow(unused_imports)]
use parser::*;


pub fn do_build() -> Result<(), boneserror::BonesError> {
  println!("Building bones project...");

  // Verify that there is a main.rs file
  /*let test = Block {
    filename: String::from("testfile.bn"),
    name: String::from("main"),
    content: String::from("Block main {\nHello, world!\n}")
  };*/

  // TODO: find all *.bn files in this and a content/ directory (if exists), 
  // and, for each, parse through them and build the blocks.

  // TODO: When unable to parse a file, generate a helpful error. 
  let main_file_name = String::from("start.campfire");

  let main_file_as_string:Option<String> = match std::fs::read_to_string(&main_file_name) {
    Ok(result) => { Some(result) },
    Err(_) => { None }
  };

  if main_file_as_string.is_none() {
    throw_general_bones_error(BONES_ERROR_MISSING_MAIN_FILE);
  }

  let mut cardslist:Vec<Card> = Vec::<Card>::new();

  parser::parse_campfire_file_as_string(&main_file_name, &main_file_as_string.unwrap(), &mut cardslist);
  
  // Convert to markdown
  for card in cardslist.iter_mut().enumerate() {
    let (_i,val):(usize, &mut Card) = card;
    if !&val.name.as_ref().is_none() { // If you don't check for this, you may get an error
                                       // while trying to .unwrap() a None (in the below param to markdown_to_html)
      val.set_compiled_body( comrak::markdown_to_html( &val.raw_body.as_ref().unwrap() , &comrak::ComrakOptions::default()) );
    }
  }
  
  // Handle campfire expressions
  for card in cardslist.iter_mut().enumerate() {
    let(_i,val):(usize, &mut Card) = card;
    if !&val.name.as_ref().is_none() {
      // TODO: Use pest to convert shortcodes to html via campfire-content-grammar.pest
    }
  }

  for card in cardslist.iter().enumerate() {
    let (_i,val):(usize, &Card) = card;

    println!("{:#?}", val);
  }

  Ok(())
}