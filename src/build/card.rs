#[allow(dead_code)]
#[derive(Debug)]
pub struct Card {
  pub source_filename: Option<String>,
  pub name: Option<String>,     // The Blocks name, or "unique identifier"
  pub raw_body: Option<String>,
  pub compiled_body: Option<String>,
}

impl Card {
  pub fn set_source_filename(&mut self, file:String) {
    self.source_filename = Some(file);
  }

  pub fn set_name(&mut self, name:String) {
    self.name = Some(name);
  }

  pub fn set_raw_body(&mut self, raw_content:String) {
    self.raw_body = Some(raw_content);
  }

  pub fn set_compiled_body(&mut self, html_content:String) {
    self.compiled_body = Some(html_content);
  }


}


mod tests {

}