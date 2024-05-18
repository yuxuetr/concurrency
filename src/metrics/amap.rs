use anyhow::{anyhow, Result};
use std::{
  collections::HashMap,
  fmt,
  sync::atomic::{AtomicI64, Ordering},
  sync::Arc,
};

#[derive(Debug)]
pub struct AmapMetrics {
  data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl Clone for AmapMetrics {
  fn clone(&self) -> Self {
    AmapMetrics {
      data: Arc::clone(&self.data),
    }
  }
}

impl AmapMetrics {
  pub fn new(metrics_names: &[&'static str]) -> Self {
    let map = metrics_names
      .iter()
      .map(|&name| (name, AtomicI64::new(0)))
      .collect();
    AmapMetrics {
      data: Arc::new(map),
    }
  }

  pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
    let key = key.as_ref();
    let counter = self
      .data
      .get(key)
      .ok_or_else(|| anyhow!("key {} not found", key))?;
    counter.fetch_add(1, Ordering::Relaxed);
    Ok(())
  }
}

impl fmt::Display for AmapMetrics {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (key, value) in self.data.iter() {
      writeln!(f, "{}: {}", key, value.load(Ordering::Relaxed))?;
    }
    Ok(())
  }
}
