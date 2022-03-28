// A document is a collection of cards

use super::card::Card;

pub struct LinkIndexItem {
    pub link_element_id: String,            // The element ID of the link span itself
    pub target_card_element_id: String,     // The element ID of target card's div
    pub target_card_name: String            // The name of the card to be rendered
}

pub struct Document {
    pub filename: String,
    pub header_content: String,
    pub body_content: String,
    pub footer_content: String,
    pub css_content: String,
    pub title: String,
    pub link_index: Vec<LinkIndexItem>,
    pub cards_list: Vec<Card>,
    pub javascript: String
}

impl Document {

    /// Sets the header to be a pre-loaded default
    /// NOTE: Ensure self.css_content is set, because this uses it!
    pub fn use_default_header(&mut self) {
        self.header_content = String::from(r##"
        <html>
            <head>
                <title>"##);
        self.header_content.push_str(&self.title);
        self.header_content.push_str(r##"</title>
                
        <style>"##);
        self.header_content.push_str(&self.css_content);
        self.use_default_css();
        self.header_content.push_str(r##"
        
                </style>

                <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.classless.min.css">
            </head>
            <body>
            <div id="campfire-card-container">
        "##);
    }

    pub fn use_default_footer(&mut self) {
        self.footer_content =  String::from(r##"
            </div><!-- /campfire-card-container !-->
            <script src="campfire.js"></script>
            </body>
        </html>
        "##);
    }

    pub fn use_default_css(&mut self) {
        self.header_content.push_str(r##"
        /*#campfire-card-container {
            width: 600px;
            margin: auto;
        }   

        .__oldcampfirecard {
            border: 1px solid #333fff;
            box-shadow: 1px 1px 3px rgba(0,0,0,.25);
            margin-bottom: 1.31em;
            padding: 1.11em;
        }*/

        .campfire-card {
            
            visibility: hidden;
            transition: opacity .71s;
            border-radius: 8px;
            
            opacity: 0;
        }

        .campfire-card-label {
            text-color: blue;
            text-decoration: underline;
            transition: text-color .5s;
            transition: height .5s;
            cursor: pointer;
        }

        .start-card {
            opacity: 1;
            visibility: visible;
        }

        .cf-clicked {
           text-color: inherit;
           text-decoration: inherit;
           cursor: inherit;
        }

        .cf-fade-in {
            visibility: visible;
            opacity: 1;
        }

        .cf-fade-out {
            visibility: hidden;
            opacity: 0;
        }
        "##);
    }

    pub fn get_final_file_contents(&mut self) -> String {
        let mut output = String::new();

        output.push_str(&self.header_content);
        output.push_str(&self.body_content);
        output.push_str(&self.footer_content);

        output
    }

    pub fn get_final_javascript_contents(&mut self) -> String {
        let mut output = String::new();

        output.push_str(&self.javascript);
        
        output
    }
}


