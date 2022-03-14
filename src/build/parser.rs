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

    for campfire_file_pair in file.into_inner() {
        println!("New card");

        let mut card = Card {
            name: None,
            source_filename: None,
            raw_body: None,
            compiled_body: None
        };

        card.set_source_filename( filename.to_string() );

        println!("{:#?}", campfire_file_pair);

        match campfire_file_pair.as_rule() {
            Rule::card => {println!("---> Card!");}, 
            Rule::card_name => {
                card.set_name(campfire_file_pair.as_str().to_string());
            },
            Rule::card_body => {
                println!("---> Card body");
                card.set_raw_body(campfire_file_pair.as_str().to_string());
            },
            Rule::EOI => (),
            _ => unreachable!(),
        } 

        // For debug purposes, print out the Block elements
        println!("{:?}", card);

        cardslist.push(card);
    }
}


