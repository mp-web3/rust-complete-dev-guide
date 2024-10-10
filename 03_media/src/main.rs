#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String }
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

    print_media(audiobook);
    print_media(movie);
    print_media(book);

}

