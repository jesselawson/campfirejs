use std::ptr::eq;

use pest::Parser;                                                                                                                                                                                    
use super::Card; 
use super::Document;
use super::error::CampfireError;
                                                                                                                                                                                                
#[derive(Parser)]                                                                                                                                                                                    
#[grammar = "campfire-content-grammar.pest"]                                                                                                                                                                            
struct ContentParser; 

fn card_exists(name:&str, known_card_names:&Vec<String>) -> bool {
    for card in known_card_names {
        // println!("----------> Checking if {} == {}...", name, card.as_str());
        if name == card.as_str() {
            return true;
        }
    }
    return false;
}

// Reads the *.campfire file and populates cardslist
pub fn compile_campfire_card_content(cardslist:&mut Vec<Card>) -> Result<(), CampfireError>{
  let mut campfire_link_counter = 0;
    let mut known_card_names:Vec<String> = Vec::<String>::new();

  for card in cardslist.iter_mut().enumerate() {
    let(_i,val):(usize,&mut Card) = card;
    known_card_names.push(val.name.as_ref().unwrap().to_string());              // Storing card names here so we have an 
    //println!("--> Adding card {}...", val.name.as_ref().unwrap());            // array to search through for card_exists()
  }

  // Populate compiled_body of each card
  for card in cardslist.iter_mut().enumerate() {
    let mut scratch = String::from("");
    let(_i,val):(usize, &mut Card) = card;
    
    scratch.push_str("<div class=\"campfire-card\" id=\"");
    scratch.push_str("card_");
    scratch.push_str(&val.name.as_ref().unwrap());
    scratch.push_str("\">");

    if !&val.name.as_ref().is_none() {
        //println!("Compiling card {}...", &val.name.as_ref().unwrap());
        //println!("html_body: {}", &val.html_body.as_ref().unwrap());
        
        let content = ContentParser::parse(
            Rule::content, 
            &val.html_body.as_ref().unwrap())
        .expect("failed to compile content for card")
        .next().unwrap();

        for expr in content.into_inner() {
            //println!("expr: {:#?}: {:#?}", expr.as_rule(), expr.as_str());
            match expr.as_rule() {
                Rule::markdown_expression => { 
                    //println!("-> Got markdown expression");
                    scratch.push_str(expr.as_str());
                },
                Rule::campfire_link_expression => {
                    //println!("-> Got campfire tag expression");

                    let mut link_tag_scratch = String::from("<span class=\"campfire-card-label\" id=\"");
                    let mut label_scratch = String::from("");
                    let mut target_scratch = String::from("");

                    for pair in expr.into_inner() {
                        
                        match pair.as_rule() {
                            Rule::label => { 
                                //println!("--> Found label: {}", &pair.as_str());
                                label_scratch.push_str(&pair.as_str());
                            },
                            Rule::target => { 
                                //println!("--> Found target: {}", &pair.as_str());
                                // Make sure card linked-to actually exists
                                if !card_exists(&pair.as_str(), &known_card_names) {
                                    println!("Compiler error: found link to non-existent card '{}'!", &pair.as_str());
                                    return Err(CampfireError::CardDoesNotExist);
                            }
                            target_scratch.push_str(&pair.as_str())
                        },
                            _ => { 
                                println!("Compiler error: unknown expression type found in card '{:?}': {:#?}", &val.name.as_ref().unwrap(), pair.as_str());
                                return Err(CampfireError::UnknownExpressionType);
                            }
                        }
                    }

                    // campfire_link_expressions are always 
                    // link{#?}_cardname_targetcard
                    link_tag_scratch.push_str("link");
                    let current_campfire_link_counter = &campfire_link_counter.to_string();
                    link_tag_scratch.push_str(&current_campfire_link_counter);
                    campfire_link_counter+=1;
                    link_tag_scratch.push_str("_");
                    link_tag_scratch.push_str(&val.name.as_ref().unwrap());
                    link_tag_scratch.push_str("_");
                    link_tag_scratch.push_str(&target_scratch);
                    link_tag_scratch.push_str("\">");
                    link_tag_scratch.push_str(&label_scratch);
                    link_tag_scratch.push_str("</span>");
                    
                    scratch.push_str(&link_tag_scratch);
                    //link_tag_scratch.push_str()
                },
                //Rule::campfire_cmd_expression => {},
                _ => { 
                    println!("Couldn't match {:?}", expr.as_rule());
                    return Err(CampfireError::UnknownExpressionType);
                }
            }
        }
        //val.set_compiled_body(compile_content(&val.compiled_body.as_ref().unwrap()).unwrap());
    }

    scratch.push_str("</div>");

    let _ = &val.set_compiled_body(scratch);

    // End of card for-loop
  }

  return Ok(())
}

pub fn build_campfire_project_dir() {

}

