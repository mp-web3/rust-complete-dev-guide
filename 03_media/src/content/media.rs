#[derive(Debug)]
pub enum Media {
    // Book, Movie, and Audiobook are referred to as variants
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String }, 
    // Podcast {episode_number: u32},
    // You don't always need to have named fields for the variants, in fact you could
    // specify a variant with one raw value of type u32 like so:
    Podcast(u32),
    // And if you don't know yet what fields need to go in the variant you can omit
    // the curly braces like so:
    Placeholder
}

impl Media {
    pub fn description(&self) -> String {
        // Until we figure out what type `self` is, Rust won't allow us to access any properties on `self`
        // even if they are common to all three different types (e.g. `title`)
        
        // if let Media::Book { title, author} = self {
        //     format!("Book title: {}, Book author: {}.", title, author)
        // } else if let Media::Movie { title, director} = self {
        //     format!("Movie title: {}, Movie director: {}.", title, director)
        // } else if let Media::Audiobook {title} = self {
        //     format!("Audiobook title: {}.", title)            
        // } else {
        //     String::from("Media description")
        // }

        // We will use instead a pattern match statement
        // Remember that match statements need to handle every possible case
        match self {
            // If self is of Media Type Book, give me access to title and author...
            Media::Book { title, author} => {
                // Inside here is like being in a If else statement
                format!("Book title: {}, Book author: {}.", title, author)
            },
            Media::Movie { title, director } => {
                format!("Movie title: {}, Movie director: {}.", title, director)
            },
            Media::Audiobook { title } => {
                format!("Audiobook title: {}.", title)
            },
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            },
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}