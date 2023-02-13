use arboard::Clipboard;
use bincode;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::clone::Clone;
use std::thread;
use std::time::Duration;
mod socket;
use socket::{recv_data, send_data};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct ImageData<'a> {
    width: usize,
    height: usize,
    bytes: Cow<'a, [u8]>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct TextData {
    text: String,
}

fn main() {
    let mut clipboard = Clipboard::new().unwrap();

    let mut last_text_data = TextData {
        text: "".to_string(),
    };

    thread::spawn(|| loop {
        let data = recv_data();
        let decoded: TextData = match bincode::deserialize(&data) {
            Ok(data) => data,
            Err(_) => continue
        };
        if let Ok(mut clipboard) = Clipboard::new() {
            match clipboard.set_text(decoded.text) {
                Ok(_) => {}
                Err(_) => {}
            };
        }
        thread::sleep(Duration::from_secs(1));
    });

    loop {
        let text_data = clipboard.get().text();
        if let Ok(text_data) = text_data {
            let text_data = TextData { text: text_data };
            if text_data != last_text_data {
                last_text_data = text_data.clone();
                let encoded: Vec<u8> = bincode::serialize(&text_data).unwrap();
                send_data(encoded.clone());
                // let decoded: TextData = bincode::deserialize(&encoded).unwrap();
                // println!("Text: {:?}", decoded);
            }
        }
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
