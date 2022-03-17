use pest::Parser;                                                                                                                                                                                    
use super::Card; 
use super::Document;
                                                                                                                                                                                                
#[derive(Parser)]                                                                                                                                                                                    
#[grammar = "campfire-content-grammar.pest"]                                                                                                                                                                            
struct ContentParser; 

// Reads the *.campfire file and populates cardslist
pub fn compile_campfire_card_content(cardslist:&mut Vec<Card>) {
    let mut campfire_link_counter = 0;
    
  for card in cardslist.iter_mut().enumerate() {
    let mut scratch = String::from("");
    let(_i,val):(usize, &mut Card) = card;
    
    scratch.push_str("<div class=\"campfire-card\" id=\"");
    scratch.push_str("card_");
    scratch.push_str(&val.name.as_ref().unwrap());
    scratch.push_str("\">");

    if !&val.name.as_ref().is_none() {
        println!("Compiling {}...", &val.name.as_ref().unwrap());
        //println!("html_body: {}", &val.html_body.as_ref().unwrap());
        
        let content = ContentParser::parse(Rule::content, &val.html_body.as_ref().unwrap())
        .expect("failed to compile content for card")
        .next().unwrap();

        for expr in content.into_inner() {
            println!("expr: {:#?}: {:#?}", expr.as_rule(), expr.as_str());
            match expr.as_rule() {
                Rule::markdown_expression => { 
                    println!("-> Got markdown expression");
                    scratch.push_str(expr.as_str());
                },
                Rule::campfire_link_expression => {
                    println!("-> Got campfire tag expression");
                    let mut link_tag_scratch = String::from("<span class=\"campfire-card-label\" id=\"");
                    let mut label_scratch = String::from("");
                    let mut target_scratch = String::from("");

                    for pair in expr.into_inner() {
                        
                        match pair.as_rule() {
                            Rule::label => { println!("--> Found label: {}", &pair.as_str());
                            label_scratch.push_str(&pair.as_str());
                        },
                            Rule::target => { println!("--> Found target: {}", &pair.as_str());
                            target_scratch.push_str(&pair.as_str())
                        },
                            _ => { println!("Error compiling Campfire tag expression:\n--> Card: {:?}\n--> {:#?}", &val.name.as_ref().unwrap(), pair.as_str()) }            
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
                _ => { println!("-----> Couldn't match {:?}", expr.as_rule()) }
            }
        }
        //val.set_compiled_body(compile_content(&val.compiled_body.as_ref().unwrap()).unwrap());
    }

    scratch.push_str("</div>");

    let _ = &val.set_compiled_body(scratch);

    // End of card for-loop
  }
}


