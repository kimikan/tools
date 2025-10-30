use bincode::{Decode, Encode};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::net::UdpSocket;

#[repr(usize)]
#[derive(Deserialize, Serialize, Encode, Decode, Debug)]
enum Phase {
  Ping(i64),
  Pong(i64, i64),
}

impl Phase {
  fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
    let encoded: Vec<u8> = bincode::encode_to_vec(self, bincode::config::standard())?;
    Ok(encoded)
  }

  fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
    let v: (Phase, usize) = bincode::decode_from_slice(bytes, bincode::config::standard())?;
    Ok(v.0)
  }
}

fn now() -> i64 {
  let now: DateTime<Utc> = Utc::now();
  now.timestamp_micros()
}

use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::PathElement;
use plotters::prelude::{BLACK, BitMapBackend, Color, IntoFont, LineSeries, RED, WHITE};
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

struct Statistics {
  ping_num: AtomicUsize,
  rtt_s: Mutex<Vec<(DateTime<Utc>, f64)>>,
}

impl Statistics {
  fn new() -> Self {
    Self {
      ping_num: 0.into(),
      rtt_s: Default::default(),
    }
  }

  fn add_ping(&self) {
    self
      .ping_num
      .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
  }

  fn add_rtt(&self, v: i64) {
    let mut m = self.rtt_s.lock().unwrap();
    (*m).push((Utc::now(), (v as f64 / 1000.0)));
  }
}

async fn udp_server(socket: Arc<UdpSocket>, stats: Arc<Statistics>) -> anyhow::Result<()> {
  let mut buf = [0; 1024];

  loop {
    let (len, src_addr) = socket.recv_from(&mut buf).await?;
    let phase = Phase::from_bytes(&buf[..len])?;
    match phase {
      Phase::Ping(v1) => {
        let p2 = Phase::Pong(v1, now());
        let bytes = p2.to_bytes()?;
        socket.send_to(&bytes[..], src_addr).await?;
      }
      Phase::Pong(v1, _v2) => {
        let v3 = now();
        let rtt = v3 - v1;
        stats.add_rtt(rtt);
      }
    };
  }
}

async fn udp_client(
  socket: Arc<UdpSocket>,
  stats: Arc<Statistics>,
  remote: String,
) -> anyhow::Result<()> {
  const PERIOD: Duration = Duration::from_millis(100);
  let mut interval = tokio::time::interval(PERIOD);

  loop {
    interval.tick().await;

    let ping = Phase::Ping(now());
    let data = ping.to_bytes()?;

    match socket.send_to(&data[..], &remote).await {
      Ok(_) => {
        stats.add_ping();
      }
      Err(e) => {
        println!("send ping failed: {:?}", e);
        continue;
      }
    }
  }
}

fn print_stats(stats: Arc<Statistics>) {
  let lock = stats.rtt_s.lock().unwrap();
  println!("Test {:?} times", stats.ping_num);
  println!("Success {:?} times", lock.len());

  lock.iter().for_each(|&(t, v)| {
    println!("{:?} {}", t, v);
  });
}

fn help() {
  println!("written by kan");
  println!("exe client 127.0.0.1:9002");
  println!("exe server 9000");
}

fn plotter(stats: Arc<Statistics>) -> anyhow::Result<()> {
  let lock = stats.rtt_s.lock().unwrap();
  if lock.is_empty() {
    return Ok(());
  }

  let sum = stats.ping_num.load(std::sync::atomic::Ordering::SeqCst);
  let percentage = (sum - lock.len()) as f64 / sum as f64 * 100.0;

  let file: &str = &format!("line-chart{}.png", Local::now());
  let x_min = lock.first().unwrap().0;
  let x_max = lock.last().unwrap().0;

  let items = lock.iter().copied().map(|v| v.1).collect::<Vec<_>>();
  let y_min = items
    .clone()
    .into_iter()
    .filter(|&x| !x.is_nan())
    .fold(f64::INFINITY, f64::min);

  let y_max = items.into_iter().fold(f64::NEG_INFINITY, f64::max);

  let root = BitMapBackend::new(file, (1920, 1024)).into_drawing_area();
  root.fill(&WHITE)?;
  let mut chart = ChartBuilder::on(&root)
    .caption(
      format!(
        "x-axis: finish-time,  y-axis: (ms), test_rounds: {}, packet_loss: {}%",
        sum, percentage
      ),
      ("sans-serif", 50).into_font(),
    )
    .margin(5)
    .x_label_area_size(30)
    .y_label_area_size(30)
    .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

  chart.configure_mesh().draw()?;

  chart
    .draw_series(LineSeries::new(lock.clone(), &RED))?
    .label("Time-Delay")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

  chart
    .configure_series_labels()
    .background_style(&WHITE.mix(0.8))
    .border_style(&BLACK)
    .draw()?;

  root.present()?;
  Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = std::env::args().collect::<Vec<_>>();
  // exe client 127.0.0.1:9002
  //exe server 9000
  println!("args: {:?}", args);
  if args.len() != 3 {
    help();
    return Ok(());
  }

  if args[1] == "client" {
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
      Ok(s) => s,
      Err(e) => {
        println!("bind failed: {:?}", e);
        return Err(anyhow::Error::new(e));
      }
    };

    println!("UDP server listening on :{}", socket.local_addr()?.port());
    let socket_arc = Arc::new(socket);

    let stats = Arc::new(Statistics::new());

    tokio::spawn(udp_server(socket_arc.clone(), stats.clone()));
    tokio::time::sleep(Duration::from_millis(100)).await;

    let client_handle = tokio::spawn(udp_client(socket_arc, stats.clone(), args[2].clone()));
    tokio::signal::ctrl_c().await?;
    client_handle.abort();

    plotter(stats.clone())?;
    print_stats(stats);
  } else if args[1] == "server" {
    let socket = match UdpSocket::bind(format!("0.0.0.0:{}", args[2])).await {
      Ok(s) => s,
      Err(e) => {
        println!("bind failed: {:?}", e);
        return Err(anyhow::Error::new(e));
      }
    };
    println!("UDP Server listening on :{:?}", args[2]);
    let socket_arc = Arc::new(socket);
    let stats = Arc::new(Statistics::new());
    udp_server(socket_arc.clone(), stats.clone()).await?;
  } else {
    help();
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::Phase;

  #[tokio::test]
  async fn test_send() {
    let p = Phase::Ping(123);

    let bytes = p.to_bytes().unwrap();
    println!("{:?}", Phase::from_bytes(&bytes[..]).unwrap());
  }
}
