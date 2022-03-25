use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use regex;
use regex::Regex;

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

/// Returns the contents of the plugin file as a string if found, or an empty string if not
fn check_for_plugin_and_load_if_found(plugin_path:&str) -> Result<String, CampfireError> {
    let plugin_file = Path::new(plugin_path);
    
    if plugin_file.exists() {
        println!("\t📄 Using onclick.js plugin");
        let content = match fs::read_to_string(plugin_file) {
            Ok(file_as_string) => { file_as_string },
            Err(error) => {
                eprintln!("{}", error);
                return Err(CampfireError::UnableToReadPluginFile);
            }
        };
        
        Ok(content)

    } else {
        Ok(String::from(""))
    }
}

// Given a cardslist and a document, compiles all the cards from cardslist's 
// and then populates the document
pub fn compile_campfire_cards_into_document(document:&mut Document) -> Result<(), CampfireError>{
  let mut campfire_link_counter:u32 = 0;

  // Storing card names here so we have an array to search through for card_exists()
  let mut known_card_names:Vec<String> = Vec::<String>::new();

  for card in document.cards_list.iter_mut().enumerate() {
    let(_i,val):(usize,&mut Card) = card;
    //println!("{:#?}", &val);
    if !val.name.is_empty() {
        known_card_names.push(val.name.clone()); 
    }
                                                                
    println!("--> Compiling card {}...", &val.name);
  }

  // Populate compiled_body of each card
  for card in document.cards_list.iter_mut().enumerate() {
    let mut scratch = String::from("");
    let(_i,val):(usize, &mut Card) = card;
    
    scratch.push_str("<div class=\"campfire-card");
    if val.name.eq("start") {
        scratch.push_str(" start-card");
    }
    scratch.push_str("\" id=\"");
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

                    let name:String = String::from(&target_scratch);
                    
                    // Store the link details for the javascript generator
                    document.link_index.push(LinkIndexItem {
                        link_element_id: link_tag_scratch,
                        target_card_element_id: target_scratch,
                        target_card_name: name
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

    // End of card for-loop; only render the start card in index.html, since the other 
    // cards will be dynamically appended when a link targeting them is clicked.
    if val.name.eq("start") {
        document.body_content.push_str(&val.compiled_body);
    }
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

    // TODO: Create a CLI flag that lets you determine how this is done (e.g., 
    // something like campfire build --singlefile (to inject it in a script tag in index.html))
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

    // Build a cards_index
    javascript.push_str("const campfire_cards = new Map();");
    for card in document.cards_list.iter() {
        javascript.push_str("campfire_cards.set('");
        javascript.push_str(&card.name);
        javascript.push_str("', \"");
        // Need to remove everything between > and < so we have one long single string,
        // then, in order for the javascript to be valid, we need to replace all 
        // quotes with \" and then replace all newlines with literal slash-n ("\n").
        // These are rendered into the html string and then appended as html to #campfire-card-container.
        let re = Regex::new(r"(?m)>\s+?<").unwrap();
        let step1 = re.replace_all(&card.compiled_body, "><").to_string();
        let step2 = &str::replace(&step1, '"', "\\\"");
        let step3 = &str::replace(&step2, '\n', "\\n");
        javascript.push_str(step3);
        javascript.push_str("\");");
    }

    javascript.push_str("function campfire_init() {");

    let mut link_counter:u32 = 0;

    // Use this to give you a string of the link element
    let link_element = |link_counter:&u32| {
        let mut str = String::from("link");
        str.push_str(link_counter.to_string().as_str());
        str.push_str("_element");
        str
    };

    let onclick_plugin_contents = check_for_plugin_and_load_if_found(
        "plugins/onclick.js").unwrap();

    for link_item in document.link_index.iter() {
        
        // let link##_element = document.getElementById("link##_<link_element_id>");
        javascript.push_str("let link");
        let current_link_counter = &link_counter;
        
        javascript.push_str(&current_link_counter.to_string());
        javascript.push_str("_element = () => { return document.getElementById(\"");
        javascript.push_str(&link_item.link_element_id);
        javascript.push_str("\"); };");
        
        // TODO: Look for for a plugins/onclick.js file to load here instead!
        // if template, then javascript.push_str(the contents of the template file).
        // Hmm!
        // else {

        // All plugins/onclick will be given two variables to work with:
        // link_element
        // target_card_element
        //javascript.push_str(&link_element(&link_counter));
        
        javascript.push_str("document.body.addEventListener('click', function(event) { if( event.target.id == '");
        javascript.push_str(&link_item.link_element_id);
        javascript.push_str("') { let link_element = () => { return ");
        javascript.push_str(&link_element(&link_counter));
        javascript.push_str("(); };");
        javascript.push_str(";let target_card_element = () => { return document.getElementById(\"card_");
        javascript.push_str(&link_item.target_card_element_id);
        javascript.push_str("\");}; let target_card_html_content = () => {");
        javascript.push_str("return campfire_cards.get(\"");
        javascript.push_str(&link_item.target_card_name);
        javascript.push_str("\");}; let campfire_card_container = () => {return document.getElementById('campfire-card-container');};\n");

        // TODO: add let target_card_contents = "card.compiled_body" as html to add to innerHTML
        //  that way we can add it to the container in the plugin file. 
        
        
        if !onclick_plugin_contents.is_empty() {
            javascript.push_str(&onclick_plugin_contents);
            javascript.push_str("\n");
        } else {
            javascript.push_str(r##"
link_element().classList.add('cf-clicked');
campfire_card_container().insertAdjacentHTML('beforeend', target_card_html_content());
// Fades in the card; if you don't delay this a bit, the fade effect wont be visible.
window.setTimeout(function() {
    target_card_element().classList.add('cf-fade-in');
},50);      
            "##);
        }

        javascript.push_str("}});"); // Close the function for addEventListener
        
        // }
        link_counter+=1;
    }

    javascript.push_str("}"); // end capfire_init function
    javascript.push_str("document.addEventListener('DOMContentLoaded', campfire_init);");

    document.javascript.push_str(&javascript);

    Ok(())
}

