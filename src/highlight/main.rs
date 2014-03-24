#[crate_id = "rshighlight#0.1.0"];
#[crate_type = "bin"];
#[license = "MIT"];

extern crate syntax;
extern crate serialize;
extern crate collections;

use std::os;
use std::io;
use std::str;

use collections::HashMap;

use backend::Backend;
use core::{Start,End,Text};

mod core;
mod backend;
mod colors;

struct Args {
    show_help: bool,

    backend: backend::BackendType,
    backend_vars: HashMap<~str, ~str>,

    header: bool,
    output_filename: Option<~str>,
    filename: Option<~str>,
}

fn parse_args(argv: &[~str]) -> Result<Args, ~str> {
    let mut args = Args {
        show_help: false,

        backend: backend::Html,
        backend_vars: HashMap::new(),

        header: false,
        output_filename: None,
        filename: None,
    };

    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_slice() {
            "--help" | "-h" => {
                args.show_help = true;
            }

            "--output" | "-o" => {
                args.output_filename = Some(argv[i + 1].to_owned());
                i += 1;
            }

            "--html" => {
                args.backend = backend::Html;
            }

            "--json" => {
                args.backend = backend::Json;
            }

            "--header" => {
                args.header = true;
            }

            "--var" | "-v" => {
                let value = argv[i + 1].as_slice();
                let parts: ~[&str] = value.splitn('=', 1).collect();
                if parts.len() != 2 {
                    return Err(format!("Bad backend variable format: {}", value));
                }
                let (name, value) = (parts[0], parts[1]);
                args.backend_vars.insert(name.to_owned(), value.to_owned());
                i += 1;
            }

            filename => {
                if args.filename.is_some() {
                    return Err(format!("Too much arguments: {}", filename));
                }

                args.filename = Some(filename.to_owned());
            }
        }

        i += 1;
    }

    Ok(args)
}

fn print_usage(program: &str) {
    println!("Usage: {} [options] [filename]", program);
    println!("");
    println!("  -h --help          Show this help and exit.");
    println!("  -o --output        Output filename.");
    println!("  --header           Output head to put before highlighted code.");
    println!("  --html             Output HTML code.");
    println!("  --latex            Output LaTeX code.");
    println!("  -v --var KEY=VAL   Set backend-specific variables.");
}

#[allow(unused_must_use)]
fn main() {
    let mut argv = os::args();
    let program = argv.shift().unwrap();

    let args = match parse_args(argv) {
        Ok(args) => args,
        Err(msg) => {
            print_usage(program);
            fail!("{}", msg);
        }
    };

    if args.show_help {
        print_usage(program);
        return;
    }

    let mut backend = backend::new_backend(args.backend);
    match backend.configure(&args.backend_vars) {
        Ok(_) => {}
        Err(ref msg) => {
            fail!("Backend error: {}", msg);
        }
    }

    let mut output = match args.output_filename {
        Some(ref filename) => {
            let path = Path::new(filename.as_slice());
            ~io::File::create(&path) as ~Writer
        }
        None => {
            ~io::stdout() as ~Writer
        }
    };

    if args.header {
        backend.header(output);
        return;
    }

    let mut input = match args.filename {
        Some(ref filename) => {
            let path = Path::new(filename.as_slice());
            ~io::File::open(&path) as ~Reader
        }
        None => {
            ~io::stdin() as ~Reader
        }
    };
    let src = input.read_to_end().unwrap();
    let src = str::from_utf8(src).unwrap();

    let parts = core::highlight(src);

    backend.code_start(output);
    for part in parts.iter() {
        match *part {
            Start(ref ty) => {
                backend.start(output, ty.as_slice());
            }
            End(ref ty) => {
                backend.end(output, ty.as_slice());
            }
            Text(ref ty, ref text) => {
                backend.start(output, ty.as_slice());
                backend.text(output, text.as_slice());
                backend.end(output, ty.as_slice());
            }
        }
    }
    backend.code_end(output);
    output.write_char('\n');
}
