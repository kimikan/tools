//! This `hub` crate is the
//! entry point of the Rust logic.

mod messages;
// use tokio_with_wasm::tokio; // Uncomment this line to target the web
mod api;

use tokio;

rinf::write_interface!();

// Use `tokio::spawn` to run concurrent tasks.
// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    //use messages::basic::*;
    // Send signals to Dart like below.
    //SmallNumber { number: 7 }.send_signal_to_dart();
    // Get receivers that listen to Dart signals like below.
    //let _ = SmallText::get_dart_signal_receiver();

    use messages::frames::*;
    let mut recv = FramesReq::get_dart_signal_receiver();
    while let Some(_) = recv.recv().await {
        let v = api::invoke::asc_to_frame_strings();

        if let Ok(v) = v {
            FramesResp{frames:v}.send_signal_to_dart();    
        } else {
            FramesResp{frames:vec![]}.send_signal_to_dart();
        }
    }
}
