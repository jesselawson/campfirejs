// A document is a collection of cards

use super::Card;
pub struct Document<'doc> {
  pub cards_list: &'doc Vec<Card>,
}


