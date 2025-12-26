use std::sync::Mutex;

use speedtest_rs::{
  distance::EarthLocation,
  error::SpeedTestError,
  speedtest::{self, SpeedTestServer},
  speedtest_config::SpeedTestConfig,
};

fn default_server() -> SpeedTestServer {
  SpeedTestServer {
    country: "China".into(),
    host: "4gsuzhou1.speedtest.jsinfo.net:8080".into(),
    id: 5396,
    location: EarthLocation::default(),
    distance: Some(1029.4774),
    name: "Suzhou".into(),
    sponsor: "China Telecom JiangSu 5G".into(),
    url: "http://4gsuzhou1.speedtest.jsinfo.net:8080/speedtest/upload.php".into(),
  }
}

fn auto_server(config: &SpeedTestConfig) -> Result<SpeedTestServer, SpeedTestError> {
  let servers = speedtest::get_server_list_with_config(&config)?;
  println!("get server list");
  let servers = servers
    .servers
    .into_iter()
    .filter(|v| v.country == "China")
    .collect::<Vec<_>>();

  println!("servers: {:?}", servers);

  let best = speedtest::get_best_server_based_on_latency(&servers)?;
  println!("found best server {:?}", best);
  Ok(best.server.clone())
}

fn main() -> Result<(), SpeedTestError> {
  //let config = speedtest::get_configuration()?;
  let config_xml = include_str!("speedtest.xml");
  let config = SpeedTestConfig::parse(config_xml)?;

  println!("got config");

  let args = std::env::args().collect::<Vec<_>>();
  let server = if args.len() >= 2 {
    auto_server(&config)?
  } else {
    default_server()
  };

  let completed = Mutex::new(0);

  let r = speedtest::test_upload_with_progress_and_config(
    &server,
    move || {
      let mut lock = completed.lock().unwrap();
      *lock += 1;
      println!("----------------------{} completed", *lock);
    },
    &config,
  )?;

  println!("Upload: {:.2} Kbps", r.kbps());

  Ok(())
}
