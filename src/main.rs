use std::io::Read;
use std::{env::args, fs::File};
use reqwest::blocking::Client;
use serde::Deserialize;
use clipboard::{ClipboardContext, ClipboardProvider};

const PASTES_URL: &str = "https://api.pastes.dev/post";

#[derive(Deserialize)]
struct PasteResult {
    key: String,
}

fn copy_to_clipboard(text: String) {
    let mut clipctx: ClipboardContext = ClipboardProvider::new().expect("Can't connect to clipboard");
    clipctx.set_contents(text).expect("Can't copy to clipboard");
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = args().collect();
    let str_args: Vec<&str> = args.iter().skip(1).map(|a| &a[..]).collect();
    match str_args[..] {
        [x, "-l", lang] => {

            let mut file = File::open(x).expect("Can't open file");
            let mut content: String = String::new();
            file.read_to_string(&mut content).unwrap();

            let req = Client::new()
                .post(PASTES_URL)
                .body(content)
                .header("Content-Type", format!("text/{}", lang))
                .send();

            let paste_url = format!("https://pastes.dev/{}",req.unwrap().json::<PasteResult>().unwrap().key);
            println!("{}", paste_url);
            copy_to_clipboard(paste_url);
            println!("Url copied to clipboard!");
        }
        _ => {
            println!("USAGE: share <FILE_PATH> -l <LANGUAGE>");
        }
    };
    Ok(())
}
