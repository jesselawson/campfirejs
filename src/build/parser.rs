use pest::Parser;                                                                                                                                                                                    
use super::Card; 
                                                                                                                                                                                                
#[derive(Parser)]                                                                                                                                                                                    
#[grammar = "campfire-file-grammar.pest"]                                                                                                                                                                            
struct CardParser; 

// Reads the *.campfire file and populates cardslist
pub fn parse_campfire_file_as_string(filename: &String, file_string: &String, cardslist:&mut Vec<Card>) {
    
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

    for line in file.into_inner() {
        //println!("--> {:?}", line.as_rule());

        card.set_source_filename( filename.to_string() );

        match line.as_rule() {
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
                        _ => { println!("Couldn't match {:?}", pair.as_rule()) }
                    }
                }
            },
            Rule::EOI => { },
            _ => { println!("Couldn't match {:?}", line.as_rule()) }
        }

        cardslist.push(card.clone());
    }

}


