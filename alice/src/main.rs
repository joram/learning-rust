extern crate stem;
extern crate reqwest;

fn main() {
    println!("hello world");

    let resp = reqwest::get("https://www.rust-lang.org");
//        .text()?;

//    use stem;
    let word = "pencils";
    let s = stem::get(word);
    match s {
       Ok(stemmed) => println!("{} => {}", word, stemmed),
       Err(e) => println!("could not stem! reason: {}", e),
    }
}