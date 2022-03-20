// A document is a collection of cards

pub struct CampfireCustomVar {
    pub key: Option<String>,
    pub value: Option<String>
}

pub struct Document {
    pub filename: String,
    pub header_content: String,
    pub body_content: String,
    pub footer_content: String,
    pub title: String
}

impl Document {

    /// Sets the header to be a pre-loaded default
    pub fn use_default_header(&mut self) {
        self.header_content = String::from(r##"
        <html>
            <head>
                <title>My Campfire App</title>
                <link rel="stylesheet" src="style.css"/>
            </head>
            <body>
                $$cards_container_contents$$
        "##);
    }

    pub fn use_default_footer(&mut self) {
        self.footer_content =  String::from(r##"
                <script src="campfire.js"/>
            </body>
        </html>
        "##);
    }

    pub fn get_final_file_contents(&mut self) -> String {
        let mut output = String::new();

        output.push_str(&self.header_content);
        output.push_str(&self.body_content);
        output.push_str(&self.footer_content);

        output
    }
}


