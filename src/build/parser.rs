use pest::Parser;                                                                                                                                                                                    
use super::Card; 
use super::CampfireError;
use super::Document;
use super::document::LinkIndexItem;
use super::error::campfire_error;
use std::path::Path;
use std::fs;

                                                                                                                                                                                                
#[derive(Parser)]                                                                                                                                                                                    
#[grammar = "campfire-file-grammar.pest"]                                                                                                                                                                            
struct CardParser; 

// If a header.html file is detected, the contents are read in as the header.
fn set_default_or_custom_header(document:&mut Document) -> Result<bool, CampfireError> {
    let header_file = Path::new("header.html");
    if header_file.exists() {
        let content = match fs::read_to_string(header_file) {
            Ok(file_as_string) => { file_as_string },
            Err(error) => {
                eprintln!("{}", error);
                return Err(CampfireError::UnableToReadHeaderFile);
            }
        };
        
        if !content.is_empty() {
            document.header_content = content;
            Ok(true)
        } else {
            eprintln!("Header template found, but it's empty!");
            return Err(CampfireError::EmptyHeaderFileFound);
        }
    } else {
        document.use_default_header();
        Ok(false)
    }
}

// If a footer.html file is detected, the contents are read in as the footer.
fn set_default_or_custom_footer(document:&mut Document) -> Result<bool, CampfireError> {
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
            document.footer_content = content;
            Ok(true)
        } else {
            eprintln!("Footer template found, but it's empty!");
            return Err(CampfireError::EmptyFooterFileFound);
        }

    } else {
        document.use_default_footer();
        Ok(false)
    }
}

// If a style.css is found, it's loaded AFTER the default Campfire CSS (in case 
// you want to override it)
fn set_css_and_check_for_custom_css(document:&mut Document) -> Result<bool, CampfireError> {
    let css_file = Path::new("style.css");
    if css_file.exists() {
        let content = match fs::read_to_string(css_file) {
            Ok(file_as_string) => { file_as_string },
            Err(error) => {
                eprintln!("{}", error);
                return Err(CampfireError::UnableToReadCSSFile);
            }
        };
        
        if !content.is_empty() {
            document.css_content = content;
            Ok(true)
        } else {
            eprintln!("CSS template found, but it's empty!");
            return Err(CampfireError::EmptyCSSFileFound);
        }

    } else {
        
        Ok(false)
    }
}

/// Returns a Document that contains any config vars that were declared, as well 
/// as the content of the header, body, and footer
pub fn parse_campfire_file_as_string(filename: &String, file_string: &String) -> Result<Document, CampfireError>{
    
    let file = CardParser::parse(Rule::campfire_file, file_string.as_str())
        .expect("unsuccessful parse")
        .next().unwrap();

    //println!("{:#?}", file);

    let mut document = Document {
        // The resultant html file
        filename: String::from("index.html"),

        // Either default contents (in document.rs) or the contents of a header.html file
        header_content: String::new(),

        // Derived from the compiled_body of all cards
        body_content: String::new(),

        // Either default contents (in document.rs) or the contents of a footer.html file
        footer_content: String::new(),
        css_content: String::new(),
        title: String::new(),
        
        cards_list: Vec::<Card>::new(),

        // When a new Campfire link is found, they're stored here for the javascript generator
        link_index: Vec::<LinkIndexItem>::new(),

        // The generated javascript
        javascript: String::new(),

        card_html_tag: String::from("div")
    };

    for line in file.into_inner() {

        let mut card = Card {
            name: String::new(),
            source_filename: String::new(),
            raw_body: String::new(),
            html_body: String::new(),
            compiled_body: String::new()
        };

        card.set_source_filename( filename.to_string() );

        match line.as_rule() {
            // Predefined commands
            Rule::set_title_command => {
                let inner_pairs = line.into_inner();
                for pair in inner_pairs {
                    match pair.as_rule() {
                        // TODO -- continue getting $set details, and populate Document.
                        Rule::command_value => {
                            document.title = pair.as_str().to_string();
                        },
                        _ => {
                            return Err(CampfireError::MalformedCampfireSetCommand);
                        }
                    }
                }
            },
            Rule::set_card_html_tag_command => {
                let inner_pairs = line.into_inner();
                for pair in inner_pairs {
                    match pair.as_rule() {
                        // TODO -- continue getting $set details, and populate Document.
                        Rule::command_value => {
                            document.card_html_tag = pair.as_str().to_string();
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
                            card.add_raw_body(pair.as_str().to_string());
                        },    
                        // Reconstruct the code fences
                        Rule::code_block_lang => {
                            card.add_raw_body(pair.as_str().to_string());
                        },
                        Rule::code_block_value => {
                            card.add_raw_body("```".to_string());
                            card.add_raw_body(pair.as_str().to_string());
                            card.add_raw_body("```".to_string());
                        },
                        _ => { 
                            println!("Couldn't parse the following: {:?}", pair.as_rule());
                            return Err(CampfireError::UnknownExpressionType);
                        }
                    }
                    
                }
                document.cards_list.push(card);
            },
            Rule::EOI => { 
                break; // This prevents a duplicated last card in the file
            },
            _ => { println!("Couldn't match {:?}", line.as_rule()) }
        }
    }

    match set_css_and_check_for_custom_css(&mut document) {
        Ok(using) => { if using { println!("\t📄 Using custom css") } },
        Err(some_error) => { 
            campfire_error(some_error);
        }
    };

    match set_default_or_custom_header(&mut document) {
        Ok(using) => { if using { println!("\t📄 Using custom header template") } },
        Err(some_error) => { 
            campfire_error(some_error);
        }
    };

    match set_default_or_custom_footer(&mut document) {
    Ok(using) => { if using { println!("\t📄 Using custom footer template") } },
        Err(some_error) => { 
            campfire_error(some_error);
        }
    };

    Ok(document)

}


