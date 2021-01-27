use std::{env, io, fs};

fn main() -> io::Result<()> {
    for filename in env::args().skip(1) {
        let file_contents = fs::read_to_string(filename)?;
        let words = file_contents.split_whitespace();
        for word in words {
            println!("word: {}", word);
        }
    }
    Ok(())
}
