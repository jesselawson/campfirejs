#[allow(dead_code)]
#[derive(Debug,Clone)]

pub struct Card {
  pub source_filename: String,
  pub name: String,     // The Blocks name, or "unique identifier"
  pub raw_body: String, // Read straight from the file
  pub html_body: String,// raw_body passed through comrak's Markdown compiler
  pub compiled_body: String, // html_body passed through our campfire expression compiler
}

impl Card {
  pub fn set_source_filename(&mut self, file:String) {
    self.source_filename = file;
  }

  pub fn set_name(&mut self, name:String) {
    self.name = name;
  }

  pub fn set_raw_body(&mut self, raw_content:String) {
    self.raw_body = raw_content;
  }

  pub fn add_raw_body(&mut self, raw_content:String) {
    self.raw_body.push_str(raw_content.as_str());
  }

  pub fn set_html_body(&mut self, html_content:String) {
    self.html_body = html_content;
  }

  pub fn set_compiled_body(&mut self, html_content:String) {
    self.compiled_body = html_content;
  }

}


mod tests {

}