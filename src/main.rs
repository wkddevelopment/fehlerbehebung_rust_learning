// Link: https://rust-lang-de.github.io/rustbook-de/ch09-00-error-handling.html

use std::fs;
use std::io;
use text_io::read;

fn main() {

    fn readfile() -> Result<String, io::Error> {
        let test = fs::read_to_string("text.txt");
        test
    }

    fn write_in_new_file(content: &str, old: Result<String, io::Error>) -> Result<(), io::Error> {
        let concstr = format!("{:?} {}", old, content);
        let writer = fs::write("text.txt", &concstr);
        writer
    }   

    loop {
        let read = readfile();
        println!("Inhalt der Textdatei:");
        println!("{:?}", read);

        println!("Neuen Text eingeben oder mit 'q' abbrechen:");
        let neuer_text: String = read!("{}\n"); 
        match neuer_text.as_str() {
            "q" => break,
            _ => {
                let _writing = write_in_new_file(&neuer_text.to_string(), read);
                //println!("{:#?}", writing);
            }}
    }

    println!("Ich ho0ffe es hat Spaß gemacht!");


}

