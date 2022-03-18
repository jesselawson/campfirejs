// A document is a collection of cards

pub struct Document {
    pub filename: Option<String>,
    pub header_content: Option<String>,
    pub body_content: Option<String>,
    pub footer_content: Option<String>
}

impl Document {
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


