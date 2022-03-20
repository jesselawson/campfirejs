

mod card;
use std::{process::exit, io::Write};

#[allow(unused_imports)]
use card::*;
mod document;
use document::*;
pub(crate) mod error;
use error::*;

mod compiler;
mod parser;

pub fn do_build() -> Result<(), error::CampfireError> {
  println!("Building Campfire project...");

  let main_file_name = String::from("start.campfire");

  // TODO: For each .campfire file, parse as string
  let main_file_as_string:Option<String> = match std::fs::read_to_string(&main_file_name) {
    Ok(result) => { Some(result) },
    Err(_) => { None }
  };

  if main_file_as_string.is_none() {
    throw_general_campfire_error(BONES_ERROR_MISSING_MAIN_FILE);
  }

  let mut cardslist:Vec<Card> = Vec::<Card>::new();

  // First we create a Document based on campfire set vars and custom set vars in the 
  // campfire file. Then, we compile the cards. Finally (later in this file), we 
  // set the document's body_content based on the compiled content in the cards.
  let mut document = match parser::parse_campfire_file_as_string(&main_file_name, &main_file_as_string.as_ref().unwrap(), &mut cardslist) {
    Ok(doc) => { doc },
    Err(some_error) => {
      return Err(some_error)
    }
  };

  let comrak_render_options = comrak::ComrakRenderOptions {
    unsafe_: true,
    ..Default::default()
  };

  let comrak_options = comrak::ComrakOptions {
    render: comrak_render_options,
    ..Default::default()
  };
  
  // Convert to markdown
  for card in cardslist.iter_mut().enumerate() {
    let (_i,val):(usize, &mut Card) = card;
    if !&val.name.as_ref().is_none() && !&val.raw_body.as_ref().is_none() { // If you don't check for this, you may get an error
                                       // while trying to .unwrap() a None (in the below param to markdown_to_html)
      val.set_html_body( comrak::markdown_to_html( &val.raw_body.as_ref().unwrap(), &comrak_options) );
    }
  }
  
  match compiler::compile_campfire_cards_into_document(&mut cardslist, &mut document) {
    Ok(()) => { println!("✅ {}", &main_file_name); },
    Err(some_error) => {
      campfire_error(some_error);
    }
  }

  // TODO: compiler::generate_javascript_for_document(&mut document) { ... }

  match compiler::build_campfire_project_dir(&mut document) {
    Ok (()) => { println!("✅ Project directory");},
    Err(some_error) => {
      campfire_error(some_error);
    }
  }

  

  Ok(())
}