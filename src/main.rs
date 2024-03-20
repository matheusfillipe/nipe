use clap::{self, Parser};
use std::env;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use tempfile::Builder;

/// nipe - vipe clone
/// nipe allows you to run your editor in the middle of a unix pipeline and edit the data that is being piped between programs. Your editor will have the full
/// data being piped from command1 loaded into it, and when you close it, that data will be piped into command2.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Suffix to use for the temporary file. Your editor might use this to determine the syntax highlighting to use.
    #[clap(short, long)]
    suffix: Option<String>,
}

fn vipe(suffix: &str) {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {
            let mut temp_file = Builder::new()
                .suffix(suffix)
                .tempfile()
                .expect("Failed to create temporary file");
            writeln!(temp_file, "{}", buffer).expect("Failed to write to temporary file");

            let editor = env::var("EDITOR").unwrap_or_else(|_| String::from("vi"));

            let status = Command::new(editor)
                .arg(temp_file.path())
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .expect("Failed to launch editor");

            if status.success() {
                let mut output = temp_file.reopen().expect("Failed to reopen temporary file");
                io::copy(&mut output, &mut io::stdout()).expect("Failed to pipe output");
            }
        }
        Err(err) => eprintln!("Error reading input: {}", err),
    }
}

fn main() {
    let args = Args::parse();
    vipe(args.suffix.as_deref().unwrap_or("txt"));
}
