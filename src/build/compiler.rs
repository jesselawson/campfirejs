use std::io::Write;
use std::process::exit;

use pest::Parser;                                                                                                                                                                                    
use super::Card; 
use super::Document;
use super::document::LinkIndexItem;
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

// Given a cardslist and a document, compiles all the cards from cardslist's 
// and then populates the document
pub fn compile_campfire_cards_into_document(cardslist:&mut Vec<Card>, document:&mut Document) -> Result<(), CampfireError>{
  let mut campfire_link_counter:u32 = 0;
    let mut known_card_names:Vec<String> = Vec::<String>::new();

  for card in cardslist.iter_mut().enumerate() {
    let(_i,val):(usize,&mut Card) = card;
    //println!("{:#?}", &val);
    if !val.name.is_empty() {
        known_card_names.push(val.name.clone()); 
    }
                                              // Storing card names here so we have an 
    println!("--> Adding card {}...", &val.name);            // array to search through for card_exists()
  }

  // Populate compiled_body of each card
  for card in cardslist.iter_mut().enumerate() {
    let mut scratch = String::from("");
    let(_i,val):(usize, &mut Card) = card;
    
    scratch.push_str("<div class=\"campfire-card\" id=\"");
    scratch.push_str("card_");
    scratch.push_str(&val.name);
    scratch.push_str("\">");

    if !&val.name.is_empty() {
        //println!("Compiling card {}...", &val.name.as_ref().unwrap());
        //println!("html_body: {}", &val.html_body.as_ref().unwrap());
        
        let content = ContentParser::parse(
            Rule::content, 
            &val.html_body)
        .expect("failed to compile content for card")
        .next().unwrap();

        for expr in content.into_inner() {
            //println!("expr: {:#?}: {:#?}", expr.as_rule(), expr.as_str());
            match expr.as_rule() {
                Rule::EOI => {},
                Rule::text 
                |Rule::string
                |Rule ::mark_tag => {
                    scratch.push_str(expr.as_str());
                },
                
                Rule::campfire_link => {
                    //println!("-> Got campfire tag expression");

                    let mut link_tag_scratch = String::from("");
                    
                    scratch.push_str("<span class=\"campfire-card-label");
                    
                    if val.name.eq("start") {
                        scratch.push_str(" start-card");
                    }
                    scratch.push_str("\" id=\"");
                    
                    let mut label_scratch = String::from("");
                    let mut target_scratch = String::from("");

                    for pair in expr.into_inner() {
                        
                        match pair.as_rule() {
                            Rule::campfire_link_label => { 
                                // Strip the first two and last one character from the string (the %{ and }).
                                // This could likely be omitted if the grammar were rewritten to ignore the 
                                // %{ and } of a link.
                                let chars = &mut pair.as_str().chars();
                                chars.next();
                                chars.next();
                                chars.next_back();
                                label_scratch.push_str(&chars.as_str());
                            },
                            Rule::campfire_link_target => { 
                                // Strip the first and last character from the string; this could likely be 
                                // omitted if the grammar were rewritten to ignore the ( and ) of a link.
                                let chars = &mut pair.as_str().chars();
                                chars.next();
                                chars.next_back();
                                //println!("--> Found target: {}", &pair.as_str());
                                // Make sure card linked-to actually exists
                                if !card_exists(&chars.as_str(), &known_card_names) {
                                    eprintln!("Compiler error: found link to non-existent card '{}'!", &pair.as_str());
                                    return Err(CampfireError::CardDoesNotExist);
                            }
                            target_scratch.push_str(&chars.as_str())
                        },
                            _ => { 
                                eprintln!("Compiler error: unknown expression type found in card '{:?}': {:#?}", &val.name, pair.as_str());
                                return Err(CampfireError::UnknownExpressionType);
                            }
                        }
                    }

                    // campfire_link_expressions are always 
                    // link{#?}_cardname_targetcard
                    link_tag_scratch.push_str("link");
                    let current_campfire_link_counter = &campfire_link_counter;
                    link_tag_scratch.push_str(&current_campfire_link_counter.to_string());
                    campfire_link_counter+=1;
                    link_tag_scratch.push_str("_");
                    link_tag_scratch.push_str(&val.name);
                    link_tag_scratch.push_str("_");
                    link_tag_scratch.push_str(&target_scratch);
                    
                    scratch.push_str(&link_tag_scratch);
                    scratch.push_str("\">");
                    scratch.push_str(&label_scratch);
                    scratch.push_str("</span>");

                    println!("-> {}", &link_tag_scratch);
                    
                    // Store the link details for the javascript generator
                    document.link_index.push(LinkIndexItem {
                        link_element_id: link_tag_scratch,
                        target_card_element_id: target_scratch
                    });
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

    scratch.push_str("</div>\n");

    let _ = &val.set_compiled_body(scratch);

    // End of card for-loop
    document.body_content.push_str(&val.compiled_body);
  }

  return Ok(())
}

pub fn build_campfire_project_dir(document:&mut Document) -> Result<(),CampfireError> {
  
    {  // Write to index.html
        let path = std::path::Path::new("project/index.html");
        let prefix = path.parent().unwrap_or_else(|| std::path::Path::new("project"));

        match std::fs::create_dir_all(prefix) {
        Ok(_) => {  },
        Err(err) => { eprintln!("Error creating project directory: {}", err); exit(1);}
        }

        let mut file_pointer= match std::fs::File::create(path) {
        Ok(file) => { file },
        _ => { println!("Unable to create output file!"); exit(1); }
        };


        match file_pointer.write(document.get_final_file_contents().as_bytes()) {
        Ok(_) => {},
        Err(err) => { eprintln!("Error writing to project file: {}", err); exit(1);}
        }
    }

    {  // Write campfire.js
        let path = std::path::Path::new("project/campfire.js");
        let prefix = path.parent().unwrap_or_else(|| std::path::Path::new("project"));

        match std::fs::create_dir_all(prefix) {
            Ok(_) => {  },
            Err(err) => { eprintln!("Error creating project directory: {}", err); exit(1);}
        }

        let mut file_pointer= match std::fs::File::create(path) {
            Ok(file) => { file },
            _ => { println!("Unable to write to campfire.js file!"); exit(1); }
        };


        match file_pointer.write(document.get_final_javascript_contents().as_bytes()) {
            Ok(_) => {},
            Err(err) => { eprintln!("Error writing to campfire.js file: {}", err); exit(1);}
        }
    }

    return Ok(())
}


/// Goes through all Document.links_stack and generates javascript to attach to them
/// which handles onclick events.
pub fn generate_javascript_for_document(document:&mut Document) -> Result<(), CampfireError> {
    
    // TODO: check for on

    let mut javascript = String::new();

    javascript.push_str("function campfire_init() {");

    let mut link_counter:u32 = 0;

    let link_element = |link_counter:&u32| {
        let mut str = String::from("link");
        str.push_str(link_counter.to_string().as_str());
        str.push_str("_element");
        str
    };

    for link_item in document.link_index.iter() {
        
        // let link##_element = document.getElementById("link##_<link_element_id>");
        javascript.push_str("let link");
        let current_link_counter = &link_counter;
        javascript.push_str(&current_link_counter.to_string());
        javascript.push_str("_element = document.getElementById(\"");
        javascript.push_str(&link_item.link_element_id);
        javascript.push_str("\");");
        
        // TODO: Look for for a plugins/link.js file to load here instead!
        // if template, then javascript.push_str(the contents of the template file).
        // else {
            
        javascript.push_str(&link_element(&link_counter));
        javascript.push_str(".addEventListener('click', function() {");
        javascript.push_str(&link_element(&link_counter));
        javascript.push_str(".classList.add('cf-clicked');");
        javascript.push_str("document.getElementById(\"card_");
        javascript.push_str(&link_item.target_card_element_id);
        javascript.push_str("\").classList.add('cf-fade-in');});");
        

        // }
        link_counter+=1;
    }

    javascript.push_str("}"); // end capfire_init function
    javascript.push_str("document.addEventListener('DOMContentLoaded', campfire_init);");

    document.javascript.push_str(&javascript);

    Ok(())
}

