/// A struct that represents an Anki card
pub struct Card {
    pub front: String,
    pub back: String,
    pub tags: String,
}
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "|{}|\n|{}|\ntags: {}",
            self.front, self.back, self.tags
        )
    }
}
impl Card {
    pub fn new(front: String, back: String, tags: String) -> Card {
        Card {
            front,
            back,
            tags,
        }
    }
    // Add an additional line of text to the back of the card
    pub fn add_to_back(&mut self, text: &str) {
        self.back += &(text.to_owned() + "<br>");
    }
    // Return a string that Anki can import
    pub fn export(&self) -> String {
        format!("{};{};{}\n",self.front,self.back,self.tags)
    }
}
