use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

use std::{thread::sleep, time::Duration};

/// Script to generate a fake progress bar.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// How long each message (step) is in milliseconds.
    #[arg(short)]
    step_length: u64,

    /// A list of messages to display next to the bar.
    #[arg(short, num_args(1..))]
    messages: Vec<String>,

    /// An optional initial message to be printed before the bar.
    #[arg(short)]
    inital_message: Option<String>,

    /// An optional final message to render after the bar is finished.
    #[arg(short)]
    final_message: Option<String>,

    /// An optional message to be printed one line after the bar.
    #[arg(short)]
    after_message: Option<String>,
}

fn main() {
    let Args { step_length, mut messages, inital_message, final_message, after_message } = Args::parse();

    let bar = ProgressBar::new(messages.len() as u64 * step_length);
    bar.set_style(
        ProgressStyle::with_template("  {spinner} [{bar:40.green/red}] {msg}")
            .unwrap()
            .tick_chars("/—\\|")
            .progress_chars("=>-"),
    );

    println!("\n");

    if let Some(initial_message) = inital_message {
        println!("  {initial_message}");
    }

    if messages.is_empty() {
        messages = vec![String::new()];
    }

    for message in messages.into_iter() {
        bar.set_message(message);
        for _ in 0..=step_length {
            sleep(Duration::from_millis(1));
            bar.inc(1);
        }
    }

    if let Some(final_message) = final_message {
        bar.set_message(final_message);
    }

    bar.finish();

    if let Some(after_message) = after_message {
        println!("  {after_message}");
    }
}
