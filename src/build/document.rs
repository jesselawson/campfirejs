// A document is a collection of cards

pub struct CampfireConfigVars {
    pub header_template_file: Option<String>,                                   // You can supply a path to a header file
    pub footer_template_file: Option<String>,                                   // Supply a path to a footer file
}

pub struct CampfireCustomVar {
    pub key: Option<String>,
    pub value: Option<String>
}

pub struct Document {
    pub filename: Option<String>,
    pub header_content: Option<String>,
    pub body_content: Option<String>,
    pub footer_content: Option<String>,
    pub title: Option<String>
}

impl Document {

    pub fn new(&mut self, filename:String) -> Document {
        let doc = Document { 
            filename: Some(filename), header_content: None, body_content: None, 
            footer_content: None, title: None };
        doc
    }

    /// Sets the header to be a pre-loaded default
    pub fn use_default_header(&mut self) {
        self.header_content = Some(String::from(r##"
        <html>
            <head>
                <title>My Campfire App</title>
                <link rel="stylesheet" src="style.css"/>
            </head>
            <body>
                $$cards_container_contents$$
        "##));
    }

    pub fn use_custom_header(&mut self, custom_header:String) {
        self.header_content = Some(custom_header);
    }

    pub fn use_default_footer(&mut self) {
        self.footer_content =  Some(String::from(r##"
                <script src="campfire.js"/>
            </body>
        </html>
        "##));
    }

    pub fn get_final_file_contents(&mut self) -> String {
        let mut output = String::new();

        output.push_str(&self.header_content.as_ref().unwrap());
        output.push_str(&self.body_content.as_ref().unwrap());
        output.push_str(&self.footer_content.as_ref().unwrap());

        output
    }
}


