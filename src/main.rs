use arboard::Clipboard;
use clap::{self, Parser};
use std::env;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use tempfile::Builder;

const DEFAULT_SUFFIX: &str = "txt";

/// nipe - vipe clone
/// nipe allows you to run your editor in the middle of a unix pipeline and edit the data that is being piped between programs. Your editor will have the full
/// data being piped from command1 loaded into it, and when you close it, that data will be piped into command2.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Suffix to use for the temporary file. Your editor might use this to determine the syntax highlighting to use.
    #[clap(short, long)]
    suffix: Option<String>,

    /// Instead of stdout and stdin use the system's clipboard. Useful for using your text editor
    /// as a clipboard editor.
    #[clap(short, long)]
    clipboard_editor: bool,

    /// Remove empty lines or whitespaces around the text.
    #[clap(short, long)]
    trim: bool,
}

impl Args {
    fn get_suffix(&self) -> &str {
        self.suffix.as_deref().unwrap_or(DEFAULT_SUFFIX)
    }

    fn get_output(&self, output: String) -> String {
        if self.trim {
            output.trim().to_string()
        } else {
            output
        }
    }
}

fn temp_editor(buffer: String, suffix: &str) -> String {
    let mut temp_file = Builder::new()
        .suffix(&format!(".{}", suffix))
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
        let mut buffer = String::new();
        output
            .read_to_string(&mut buffer)
            .expect("Failed to read from temporary file");
        buffer
    } else {
        panic!("Editor exited with non-zero status");
    }
}

fn vipe(args: Args) {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {
            let output = temp_editor(buffer, args.get_suffix());
            io::stdout()
                .write_all(args.get_output(output).as_bytes())
                .unwrap();
        }
        Err(err) => eprintln!("Error reading input: {}", err),
    }
}

fn clipboard_editor(args: Args) {
    let mut clipboard = Clipboard::new().unwrap();
    let buffer = clipboard.get_text().unwrap();
    let output = temp_editor(buffer, args.get_suffix());
    clipboard.set_text(args.get_output(output)).unwrap();
}

fn main() {
    let args = Args::parse();
    if args.clipboard_editor {
        clipboard_editor(args);
    } else {
        vipe(args);
    }
}
