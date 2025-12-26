use std::{
  env, fs,
  io::{self, Write},
  net::IpAddr,
  time::Duration,
};

use chrono::Local;
use trippy_core::{Builder, Protocol};

async fn trace_route(target_ip: &str, file: &str) -> anyhow::Result<()> {
  let period: Duration = Duration::from_millis(100u64);
  let mut interval = tokio::time::interval(period);

  let dst_ip: IpAddr = IpAddr::V4(target_ip.parse()?);
  let mut is_first = true;

  let mut file = fs::File::create(file)?;
  let mut writer = io::BufWriter::new(&mut file);

  loop {
    interval.tick().await;

    println!("trace to target ip: {}", target_ip);
    let tracer = Builder::new(dst_ip)
      .protocol(Protocol::Icmp)
      .max_rounds(Some(1))
      .trace_identifier(1)
      .build()?;
    tracer.run()?;
    let result = tracer.snapshot();
    let hops = result.hops();

    if is_first {
      let ip_s = hops
        .iter()
        .map(|v| v.addrs().map(|v| v.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
      //std::fs::write("hops", format!("{:?}", ip_s))?;
      println!("ips: {:?}", ip_s);
      let vs = ip_s.iter().map(|v| v.first().cloned()).collect::<Vec<_>>();
      println!("{:?}", vs);
      let mut result = ",".to_string();
      for v in vs {
        if let Some(v) = v {
          result += &format!("{},", v);
        } else {
          result += ",";
        }
      }
      println!("{:?}", result);
      writer.write_fmt(format_args!("{}\n", result))?;
      is_first = false;
    }

    let avg_ms = hops
      .iter()
      .map(|v| v.avg_ms().to_string())
      .collect::<Vec<_>>();
    let line = avg_ms.join(",");
    writer.write_fmt(format_args!("{},{}\n", Local::now(), line))?;
    writer.flush()?;
  }
}

use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args: Vec<String> = env::args().collect();

  if args.len() == 2 {
    trace_route(&(args[1]), &format!("{}.csv", args[1])).await?;
  } else {
    println!("error command, traceroute ip");
  }
  Ok(())
}
