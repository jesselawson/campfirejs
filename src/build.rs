

mod card;
use std::{process::exit, io::Write};

#[allow(unused_imports)]
use card::*;
mod document;
use document::*;
mod error;
use error::*;

mod compiler;
mod parser;

pub fn do_build() -> Result<(), error::CampfireError> {
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

  match parser::parse_campfire_file_as_string(&main_file_name, &main_file_as_string.as_ref().unwrap(), &mut cardslist) {
    Ok(()) => { },
    Err(some_error) => {
      return Err(some_error)
    }
  }

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
    if !&val.name.as_ref().is_none() { // If you don't check for this, you may get an error
                                       // while trying to .unwrap() a None (in the below param to markdown_to_html)
      val.set_html_body( comrak::markdown_to_html( &val.raw_body.as_ref().unwrap() , &comrak_options) );
    }
  }
  
  match compiler::compile_campfire_card_content(&mut cardslist) {
    Ok(()) => { println!("✅ {}", &main_file_name); },
    Err(some_error) => {
      println!("Compilation halted: {}", campfire_error(some_error));
    }
  }

  match compiler::build_campfire_project_dir() {
    Ok (()) => { println!("✅ Project directory");},
    Err(some_error) => {
      println("Compilation halted: {}", campfire_error(some_error));
    }
  }

  // Write compiled_body for all cards to file
  let path = std::path::Path::new("project/test.html");
  let prefix = path.parent().unwrap();
  
  match std::fs::create_dir_all(prefix) {
    Ok(_) => {},
    Err(err) => { eprintln!("Error creating project directory: {}", err); exit(1);}
  }
  
  let mut file_pointer= match std::fs::File::create(path) {
    Ok(file) => { file },
    _ => { println!("Unable to create output file!"); exit(1); }
  };
  
  for card in cardslist.iter_mut().enumerate() {
    let (_i,val):(usize, &mut Card) = card;
    if !&val.name.as_ref().is_none() {
      /*match std::fs::write(path, &val.compiled_body.as_ref().unwrap()) {
        Ok(_file) => {},
        Err(err) => {println!("Error writing to project: {}", err);}
      }*/
      match file_pointer.write(&val.compiled_body.as_ref().unwrap().as_bytes()) {
        Ok(_) => {},
        Err(err) => { eprintln!("Error writing to project file: {}", err); exit(1);}
      }
    }
  }



  Ok(())
}