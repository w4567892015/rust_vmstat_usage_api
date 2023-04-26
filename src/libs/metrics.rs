use std::{process::Command};
use chrono::{Utc, DateTime};

pub fn get_metrics() -> String {
	let output = Command::new("vmstat")
		.args(["1", "1"])
		.output()
		.expect("failed to execute process");

	let out = String::from_utf8_lossy(&output.stdout);
	let parts: Vec<&str> = out.split("\n").collect();
	let part: Vec<&str> = parts[2].trim().split_whitespace().collect();
	let data = part.join(",");

	let now: DateTime<Utc> = Utc::now();
	let formatted_time = now.format("%H:%M:%S%.3f").to_string();
	let perf_counters = format!("{},{}", formatted_time, data);

	return perf_counters;
}
