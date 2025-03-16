#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String, narrator: String },
    Podcast(u32),
    Unknown,
}

impl Media {

    pub fn description(&self) -> String {
        match self {
            Media::Book { title, author} => format!("Book: {}, Author: {}", title, author),
            Media::Movie { title, director } => format!("Movie: {}, Director {}", title, director),
            Media::Audiobook { title, narrator } => format!("Audiobook: {}, Narrator: {}", title, narrator),
            Media::Podcast(episode_number) => format!("Podcast: Episode {}", episode_number),
            Media::Unknown => format!("Unknown media type"),
        }

    }

    pub fn print_description(&self) {
        println!("{}", self.description());
    }
}