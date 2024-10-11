#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String }
}

impl Media {
    fn description(&self) -> String {
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
            }
        }

    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("When Plants Dream")
    };

    let movie = Media::Movie {
        title: String::from("The Suicide Squad"),
        director: String::from("James Gunn")
    };

    let book = Media::Book {
        title: String::from("Permanent Record"),
        author: String::from("Edward Snowden")
    };

    println!( "{:#?}", book.description());
    println!( "{:#?}", movie.description());
    println!( "{:#?}", audiobook.description());

}

