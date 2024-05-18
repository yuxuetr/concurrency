use anyhow::Result;
use concurrency::Metrics;
use rand::{thread_rng, Rng};
use std::{thread, time::Duration};

const M: usize = 4;
const N: usize = 2;

fn main() -> Result<()> {
  let metrics = Metrics::new();
  println!("{:?}", metrics.snapshot());

  for idx in 0..N {
    task_worker(idx, metrics.clone());
  }

  for _ in 0..M {
    request_worker(metrics.clone());
  }

  loop {
    thread::sleep(Duration::from_secs(2));
    println!("{:?}", metrics.snapshot());
  }
}

fn task_worker(idx: usize, metrics: Metrics) {
  thread::spawn(move || loop {
    let mut rng = thread_rng();
    thread::sleep(Duration::from_millis(rng.gen_range(500..5000)));
    metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();
  });
}

fn request_worker(metrics: Metrics) {
  thread::spawn(move || loop {
    let mut rng = thread_rng();
    thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
    let page = rng.gen_range(1..5);
    metrics.inc(format!("req.page.{}", page)).unwrap();
  });
}
