use clap::Parser;
use flameview::add_one;  // will be swapped for real API in Milestone M6

#[derive(Parser)]
struct Opt { #[arg(long, default_value_t = 0)] value: i32 }

fn main() { println!("{}", add_one(Opt::parse().value)); }
