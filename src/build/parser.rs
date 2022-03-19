use pest::Parser;                                                                                                                                                                                    
use super::Card; 
use super::CampfireError;
use super::Document;
use std::path::Path;
use std::fs;

                                                                                                                                                                                                
#[derive(Parser)]                                                                                                                                                                                    
#[grammar = "campfire-file-grammar.pest"]                                                                                                                                                                            
struct CardParser; 

fn check_for_custom_footer_template(document:&mut Document) -> Result<(), CampfireError> {
    let footer_file = Path::new("footer.html");
    if footer_file.exists() {
        let content = match fs::read_to_string(footer_file) {
            Ok(file_as_string) => { file_as_string },
            Err(error) => {
                eprintln!("{}", error);
                return Err(CampfireError::UnableToReadFooterFile);
            }
        };
        
        if !content.is_empty() {
            document.footer_content = Some(content);
        }
    } else {
        document.use_default_footer();
    }

    Ok(())
}

/// Returns a Document that contains any config vars that were declared, as well 
/// as the content of the header, body, and footer
pub fn parse_campfire_file_as_string(filename: &String, file_string: &String, cardslist:&mut Vec<Card>) -> Result<Document, CampfireError>{
    
    let file = CardParser::parse(Rule::campfire_file, file_string.as_str())
        .expect("unsuccessful parse")
        .next().unwrap();

    //println!("{:#?}", file);

    let mut card = Card {
        name: None,
        source_filename: None,
        raw_body: None,
        html_body: None,
        compiled_body: None
    };

    let mut document = Document {
        filename: Some(String::from("index.html")),
        header_content: None,
        body_content: None,
        footer_content: None,
        title: None,
    };

    for line in file.into_inner() {

        card.set_source_filename( filename.to_string() );

        match line.as_rule() {
            // Predefined commands
            Rule::campfire_set_command => {

                let inner_pairs = line.into_inner();
                for pair in inner_pairs {
                    match pair.as_rule() {
                        // TODO -- continue getting $set details, and populate Document.
                        Rule::command_target => {
                            match pair.as_str() {
                                "title" => {},
                                _ => {
                                    return Err(CampfireError::UnknownCampfireSetCommand);
                                }
                            }

                        },
                        Rule::command_value => {

                        },
                        _ => {
                            return Err(CampfireError::MalformedCampfireSetCommand);
                        }
                    }
                }
            },

            Rule::card => { 
                let inner_pairs = line.into_inner();
                for pair in inner_pairs {
                    match pair.as_rule() {
                        Rule::card_name => {
                            card.set_name(pair.as_str().to_string());
                        },
                        Rule::card_body => {
                            card.set_raw_body(pair.as_str().to_string());
                        },
                        _ => { 
                            println!("Couldn't parse the following: {:?}", pair.as_rule());
                            return Err(CampfireError::UnknownExpressionType);

                        }
                    }
                }
            },
            Rule::EOI => { 
                break; // This prevents a duplicated last card in the file
            },
            _ => { println!("Couldn't match {:?}", line.as_rule()) }
        }

        cardslist.push(card.clone());
    }

    Ok(document)

}


