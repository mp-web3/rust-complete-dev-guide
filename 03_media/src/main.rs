mod content;

use content::catalog::Catalog;
use content::media::Media;

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

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // println!("{:#?}", catalog);
    // To access a single media in the catalog
    // println!("{:#?}", catalog.items.get(100));

    let mut item = catalog.get_by_index(0);
    
    // Other ways of handling options

    // item.unwap()
    // Use for quick debugging or examples
    println!("{:#?}", item.unwrap());
    // item.expect()
    // Use when we want the program **to crash** if there is no value
    item = catalog.get_by_index(1);
    println!("{:#?}", item.expect("expected there to ba a value here"));
    // item.unwrap_or()
    // Use when it makes sense to provide a fallback value
    item = catalog.get_by_index(40);
    let placeholder = Media::Placeholder;
    println!("{:#?}", item.unwrap_or(&placeholder));

}

