use hotkey_register::{keys, modifiers, HotkeyListener, Listener, ListenerHotkey};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
// use std::rc::Rc;

enum Message {
    TEXT(String),
}

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(0.5);
    let (sender, receiver) = mpsc::channel();
    //    let mut hotkey: Listener = HotkeyListener::new();
    let mut hotkey = Listener::new();
    let result = hotkey.register_hotkey(ListenerHotkey::new(modifiers::ALT, keys::E), move || {
        sender.send(Message::TEXT("handle hotkey".to_string()));
    });

    match result {
        Ok(_) => println!("listener success"),
        Err(err) => println!("error {:?}", err),
    }
    let file =
        BufReader::new(File::open("/Users/jerry/Music/local/化身孤岛的鲸-周深.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);

    loop {
        match receiver.recv() {
            Ok(Message::TEXT(info)) => {
                println!("info is {:?}", info);
            }
            _ => {
                println!("match failed");
            }
        }
        //        sink.
        //        let item = source.clone();
        //        BufReader::new(File::open("/Users/jerry/Music/local/飞鸟和蝉-任然.mp3").unwrap());
        // Decode that sound file into a source
        //        sink.
        //    sink.sleep_until_end();
        //        sink.s
    }
}
