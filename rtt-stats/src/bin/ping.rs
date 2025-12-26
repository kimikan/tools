use std::time::Duration;

use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
  let args = std::env::args().collect::<Vec<_>>();
  if args.len() != 2 {
    return Err(anyhow!("invalid args: ping ip"));
  }

  use single_ping::ping;

  loop {
    let result = ping(&args[1], 1000, 1);
    std::thread::sleep(Duration::from_secs(1));

    match result {
      Ok(o) => {
        if o.dropped {
          println!("msg dropped");
        } else {
          println!("Ping successful rtt={}", o.latency_ms);
        }
      }
      Err(e) => {
        println!("error :{:?}", e);
        break;
      }
    };
  }

  Ok(())
}
