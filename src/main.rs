/// You need to do this to use macro
extern crate pest;                                                                                                                                                                                   
#[macro_use]                                                                                                                                                                                         
extern crate pest_derive;

mod build; 
use build::do_build;
use build::error::campfire_error;
mod publish; use publish::*;

fn show_banner() {
  println!("Campfire, v{}", env!("CARGO_PKG_VERSION"));
  println!("https://campfirejs.org")
}


fn main() {
       
    let command = std::env::args().nth(1);

    let flag = match std::env::args().nth(2) {
      Some(flag_str) => flag_str,
      _ => {String::new()}
    };

    if !command.is_none() {
      match command.as_deref() {
         Some("build") => { 
            if !flag.is_empty() {
              if flag.eq("--html_files") {
                /*match do_html_files_build() {
                  Ok(()) => { println!("🪵🔥🪵 HTML Files Compilation successful!") },
                  Err(some_error) => {
                    campfire_error(some_error);
                  }
                }*/
              }
            } else {
              match do_build() {
                Ok(()) => { println!("🪵🔥🪵 HTML + JS Compilation successful!") },
                Err(some_error) => {
                  campfire_error(some_error);
              }
            } 
         }
        },
         Some("publish") => { do_publish() },
         _ => { show_banner(); }
      }
    } else {
      show_banner();
    }

    
}
