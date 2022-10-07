use clipboard::{ClipboardContext, ClipboardProvider};
use curl::easy::{Easy, List};
use std::io::Read;
use std::str;
use std::{env::args, fs::File};

const PASTES_URL: &str = "https://api.pastes.dev/post";

fn send_request(data: &str, res: &mut Vec<u8>, lang: &str) {
    let mut easy = Easy::new();
    easy.url(PASTES_URL).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut headers = List::new();
    headers
        .append(format!("Content-Type: text/{}", lang).as_str())
        .unwrap();

    easy.http_headers(headers).unwrap();

    let mut transfer = easy.transfer();

    transfer
        .write_function(|val: &[u8]| {
            res.extend_from_slice(val);
            Ok(val.len())
        })
        .unwrap();

    transfer
        .read_function(|buf| Ok(data.as_bytes().read(buf).unwrap_or(0)))
        .unwrap();

    transfer.perform().unwrap();
}

fn copy_to_clipboard(text: String) {
    let mut clipctx: ClipboardContext =
        ClipboardProvider::new().expect("Can't connect to clipboard");

    clipctx.set_contents(text).expect("Can't copy to clipboard");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = args().collect();
    let str_args: Vec<&str> = args.iter().skip(1).map(|a| &a[..]).collect();
    let mut res: Vec<u8> = Vec::new();
    match str_args[..] {
        [x, "-l", lang] => {
            let mut file = File::open(x).expect("Can't open file");
            let mut buf: String = String::new();
            file.read_to_string(&mut buf).unwrap();
            send_request(buf.as_str(), &mut res, lang);

            let res_as_str = str::from_utf8(res.as_slice()).unwrap();

            let key = res_as_str
                .split(':')
                .last()
                .unwrap()
                .split('\"')
                .nth(1)
                .unwrap();

            let paste_url = format!("https://pastes.dev/{}", key);
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
