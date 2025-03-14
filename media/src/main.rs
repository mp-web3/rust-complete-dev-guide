#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String, narrator: String },
    Unknown,
}

impl Media {
    fn new(media_type: &str, title: String, creator: String) -> Self {
        match media_type {
            "book" => Media::Book {
                title,
                author: creator,
            },
            "movie" => Media::Movie {
                title,
                director: creator,
            },
            "audiobook" => Media::Audiobook {
                title,
                narrator: creator,
            },
            _ => Media::Unknown,
        }
    }

    fn description(&self) -> String {
        if let Media::Book { title, author } = self {
            format!("Book: {}, Author: {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie: {}, Director {}", title, director)
        } else if let Media::Audiobook { title, narrator } = self {
            format!("Audiobook: {}, Narrator: {}", title, narrator)
        } else {
            format!("Unknown media type")
        }
    }

    fn print_description(&self) {
        println!("{}", self.description());
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let book: Media = Media::Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
    };

    let book_impl = Media::new("book", String::from("The Great Gatsby"), String::from("F. Scott Fitzgerald"));
    let movie_impl = Media::new("movie", String::from("The Godfather"), String::from("Francis Ford Coppola"));
    let audiobook_impl = Media::new("audiobook", String::from("Harry Potter"), String::from("Stephen Fry"));

    book_impl.print_description();
    movie_impl.print_description();
    audiobook_impl.print_description();
}
