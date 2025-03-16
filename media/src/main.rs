mod content;
use content::media::Media;
use content::catalog::Catalog;

fn main() {

    let book = Media::Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
    };
    let movie = Media::Movie {
        title: String::from("The Godfather"),
        director: String::from("Francis Ford Coppola"),
    };
    let audiobook = Media::Audiobook {
        title: String::from("Harry Potter"),
        narrator: String::from("Stephen Fry"),
    };
    let podcast = Media::Podcast(1);
    let mut catalog = Catalog::new();
    catalog.add_media(book);
    catalog.add_media(audiobook);
    catalog.add_media(movie);
    catalog.add_media(podcast);

    match catalog.get_by_index(3) {
        Some(item) => println!("Item: {:#?}", item ),
        None => println!("No items in catalog"),
    }


    // book_impl.print_description();
    // movie_impl.print_description();
    // audiobook_impl.print_description();
}
