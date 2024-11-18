use clap::Parser;
use std::env;

#[derive(Parser)]
#[clap(name = "wasm-host", version = env!("CARGO_PKG_VERSION"))]
struct HostApp {
    end: u64,
}

fn main() -> anyhow::Result<()> {
    HostApp::parse().run()
}

fn fibonacci_iterative(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

fn integer_cube_root(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let mut low = 0;
    let mut high = n;

    while low <= high {
        let mid = low + (high - low) / 2;
        let cube = mid.saturating_mul(mid).saturating_mul(mid);

        if cube == n {
            return mid;
        } else if cube < n {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    high
}

impl HostApp {
    fn run(self) -> anyhow::Result<()> {
        for end in 1..=self.end {
            println!(
                "end = {} ;root result = {}",
                end,
                integer_cube_root(
                    (1..=end)
                        .map(|i| (i as f64).sin().cos().tan())
                        .sum::<f64>()
                        .round() as u64
                )
            );
        }

        Ok(())
    }
}
