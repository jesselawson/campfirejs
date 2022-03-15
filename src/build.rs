

mod card;
#[allow(unused_imports)]
use card::*;
mod boneserror;
use boneserror::*;

mod compiler;

mod parser;
#[allow(unused_imports)]
use parser::*;


pub fn do_build() -> Result<(), boneserror::CampfireError> {
  println!("Building Campfire project...");

  let main_file_name = String::from("start.campfire");

  let main_file_as_string:Option<String> = match std::fs::read_to_string(&main_file_name) {
    Ok(result) => { Some(result) },
    Err(_) => { None }
  };

  if main_file_as_string.is_none() {
    throw_general_campfire_error(BONES_ERROR_MISSING_MAIN_FILE);
  }

  let mut cardslist:Vec<Card> = Vec::<Card>::new();

  parser::parse_campfire_file_as_string(&main_file_name, &main_file_as_string.unwrap(), &mut cardslist);
  
  // Convert to markdown
  for card in cardslist.iter_mut().enumerate() {
    let (_i,val):(usize, &mut Card) = card;
    if !&val.name.as_ref().is_none() { // If you don't check for this, you may get an error
                                       // while trying to .unwrap() a None (in the below param to markdown_to_html)
      val.set_html_body( comrak::markdown_to_html( &val.raw_body.as_ref().unwrap() , &comrak::ComrakOptions::default()) );
    }
  }
  
  compiler::compile_campfire_card_content(&mut cardslist);

  for card in cardslist.iter_mut().enumerate() {
    let (_i,val):(usize, &mut Card) = card;
    if !&val.name.as_ref().is_none() {
      println!("{:#?}", &val);
    }
  }

  Ok(())
}