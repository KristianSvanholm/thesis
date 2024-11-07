mod linux;
mod macos;

#[cfg(target_os = "linux")]
use linux::benchmark::benchmark;
#[cfg(target_os = "macos")]
use macos::benchmark::benchmark;

fn main() {
  println!("{} µj", benchmark("pwd"));
  println!("{} µj", benchmark(""));
}
