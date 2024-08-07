use manganis::*;

fn main() {
    // Generate a unqiue file name for the txt file after it's been bundled
    let txt_file = asset!("/assets/file.txt");
    println!("{txt_file}");

    // Generate a unqiue file name for the txt file after it's been bundled
    let image_file = asset!("/assets/logo.png");
    println!("{image_file}");

    // // Include folders!
    // let folder = asset!("/assets/somefolder");
    // println!("{folder}");

    // // Urls too
    // let url = asset!("https://raw.githubusercontent.com/TheZoq2/ferris/2c58ca0909375fcf8a21ce0296fb320e5bdf5bea/book_cover/space.png");
    // println!("{url}");

    // dbg!(&txt_file.resolve());
}
