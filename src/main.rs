/// You need to do this to use macro
extern crate pest;                                                                                                                                                                                   
#[macro_use]                                                                                                                                                                                         
extern crate pest_derive;

mod build; 
use build::do_build;
mod publish; use publish::*;

fn show_banner() {
  println!("Campfire, v{}", env!("CARGO_PKG_VERSION"));
  println!("https://campfirejs.org")
}


fn main() {
       
    let command = std::env::args().nth(1);

    if !command.is_none() {
      match command.as_deref() {
         Some("build") => { do_build(); },
         Some("publish") => { do_publish() },
         _ => { show_banner(); }
      }
    } else {
      show_banner();
    }

    
}
