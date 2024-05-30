#![allow(unused_imports)]
#![allow(unused_mut)]

use crate::tokio;
use prost::Message;
use rinf::debug_print;
use rinf::DartSignal;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;
use tokio::sync::mpsc::channel;

type SignalHandlers =
    OnceLock<Mutex<HashMap<i32, Box<dyn Fn(Vec<u8>, Vec<u8>) + Send>>>>;
static SIGNAL_HANDLERS: SignalHandlers = OnceLock::new();

pub fn handle_dart_signal(
    message_id: i32,
    message_bytes: Vec<u8>,
    binary: Vec<u8>
) {    
    let mutex = SIGNAL_HANDLERS.get_or_init(|| {
        let mut hash_map =
            HashMap
            ::<i32, Box<dyn Fn(Vec<u8>, Vec<u8>) + Send + 'static>>
            ::new();
hash_map.insert(
    0,
    Box::new(|message_bytes: Vec<u8>, binary: Vec<u8>| {
        use super::basic::*;
        let message = SmallText::decode(
            message_bytes.as_slice()
        ).unwrap();
        let dart_signal = DartSignal {
            message,
            binary,
        };
        let cell = SMALL_TEXT_CHANNEL
            .get_or_init(|| {
                let (sender, receiver) = channel(1024);
                Mutex::new(RefCell::new(Some((Some(sender), Some(receiver)))))
            })
            .lock()
            .unwrap();
        #[cfg(debug_assertions)]
        {
            // After Dart's hot restart,
            // a sender from the previous run already exists
            // which is now closed.
            let borrowed = cell.borrow();
            let pair = borrowed.as_ref().unwrap();
            let is_closed = pair.0.as_ref().unwrap().is_closed();
            drop(borrowed);
            if is_closed {
                let (sender, receiver) = channel(1024);
                cell.replace(Some((Some(sender), Some(receiver))));
            }
        }
        let borrowed = cell.borrow();
        let pair = borrowed.as_ref().unwrap();
        let sender = pair.0.as_ref().unwrap();
        let _ = sender.try_send(dart_signal);
    }),
);
hash_map.insert(
    2,
    Box::new(|message_bytes: Vec<u8>, binary: Vec<u8>| {
        use super::frames::*;
        let message = FramesReq::decode(
            message_bytes.as_slice()
        ).unwrap();
        let dart_signal = DartSignal {
            message,
            binary,
        };
        let cell = FRAMES_REQ_CHANNEL
            .get_or_init(|| {
                let (sender, receiver) = channel(1024);
                Mutex::new(RefCell::new(Some((Some(sender), Some(receiver)))))
            })
            .lock()
            .unwrap();
        #[cfg(debug_assertions)]
        {
            // After Dart's hot restart,
            // a sender from the previous run already exists
            // which is now closed.
            let borrowed = cell.borrow();
            let pair = borrowed.as_ref().unwrap();
            let is_closed = pair.0.as_ref().unwrap().is_closed();
            drop(borrowed);
            if is_closed {
                let (sender, receiver) = channel(1024);
                cell.replace(Some((Some(sender), Some(receiver))));
            }
        }
        let borrowed = cell.borrow();
        let pair = borrowed.as_ref().unwrap();
        let sender = pair.0.as_ref().unwrap();
        let _ = sender.try_send(dart_signal);
    }),
);
        Mutex::new(hash_map)
    });

    let guard = mutex.lock().unwrap();
    let signal_handler = guard.get(&message_id).unwrap();
    signal_handler(message_bytes, binary);
}
