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

    if !command.is_none() {
      match command.as_deref() {
         Some("build") => { 
            match do_build() {
              Ok(()) => { println!("🪵🔥🪵 Compilation successful!") },
              Err(some_error) => {
                campfire_error(some_error);
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
