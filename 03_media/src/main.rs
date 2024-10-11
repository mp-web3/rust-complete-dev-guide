#[derive(Debug)]
enum Media {
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

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
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

    let podcast = Media::Podcast(1);
    let placeholder = Media::Placeholder;

    // println!( "{:#?}", book.description());
    // println!( "{:#?}", movie.description());
    // println!( "{:#?}", audiobook.description());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // println!("{:#?}", catalog);
    // To access a single media in the catalog
    // println!("{:#?}", catalog.items.get(100));

    match catalog.items.get(10) {
        // Option:: is implicit
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => println!("Nothing at that index!")
    }

}

