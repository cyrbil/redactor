/*
 * Copyright (c) 2024. http://github.com/cyrbil - All rights reserved.
 * This file is part of Redactor which is released under MIT LICENSE.
 * See file LICENSE.txt for full license details.
 */

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Cli::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
