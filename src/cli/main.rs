#![crate_id = "rshighlight#0.1.0"]
#![crate_type = "bin"]
#![license = "MIT"]

extern crate collections;
extern crate getopts;
extern crate highlight;

use std::os;
use std::io;
use std::str;

use collections::HashMap;

use highlight::{backend, core};
use highlight::backend::Backend;
use highlight::core::{Start,End,Text};

struct Args {
    show_help: bool,

    backend: backend::BackendType,
    backend_vars: HashMap<~str, ~str>,

    header: bool,
    output_filename: Option<~str>,
    filename: Option<~str>,
}

fn parse_args(argv: &[~str], opts: &[getopts::OptGroup]) -> Result<Args, ~str> {
    fn select_backend(matches: &getopts::Matches) -> Result<backend::BackendType, ~str> {
        if matches.opt_present("html") {
            Ok(backend::Html)
        } else if matches.opt_present("json") {
            Ok(backend::Json)
        } else if matches.opt_present("latex") {
            Ok(backend::Latex)
        } else {
            Ok(backend::Html)
        }
    }

    let argv: Vec<StrBuf> = argv.iter().map(|s| s.to_strbuf()).collect();
    let matches = match getopts::getopts(argv.as_slice(), opts) {
        Ok(m) => m,
        Err(e) => {
            return Err(e.to_err_msg().into_owned());
        }
    };

    let mut args = Args {
        show_help: matches.opt_present("help"),

        backend: match select_backend(&matches) {
            Ok(ty) => ty,
            Err(e) => {
                return Err(e);
            }
        },
        backend_vars: HashMap::new(),

        header: matches.opt_present("header"),
        output_filename: matches.opt_str("output").map(|s| s.into_owned()),
        filename: match matches.free.len() {
            0 => None,
            _ => Some(matches.free.get(0).to_owned()),
        }
    };

    for var in matches.opt_strs("var").iter() {
        let value = var.as_slice();
        let parts: Vec<&str> = value.splitn('=', 1).collect();
        if parts.len() != 2 {
            return Err(format!("Bad backend variable format: {}", value));
        }
        let (name, value) = (parts.get(0), parts.get(1));
        args.backend_vars.insert(name.to_owned(), value.to_owned());
    }

    Ok(args)
}

static BRIEF: &'static str = "Small Rust tool to output highlighted Rust code.";

fn print_usage(program: &str, opts: &[getopts::OptGroup]) {
    println!("Usage: {} [filename]", getopts::short_usage(program, opts));
    println!("");
    println!("    {}", getopts::usage(BRIEF, opts));
}

#[allow(unused_must_use)]
fn main() {
    let argv = os::args();
    let (program, argv) = (argv.get(0).as_slice(), argv.tail());

    let opts = ~[
        getopts::optflag("h", "help", "Show this help and exit."),
        getopts::optopt("o", "output", "Output filename.", "FILENAME"),
        getopts::optflag("", "header", "Output head to put before highlighted code."),
        getopts::optflag("", "html", "Output HTML code."),
        getopts::optflag("", "json", "Output JSON code."),
        getopts::optflag("", "latex", "Output LaTeX code."),
        getopts::optmulti("v", "var", "Set backend-specific variables.", "KEY=VAL"),
    ];

    let args = match parse_args(argv, opts) {
        Ok(args) => args,
        Err(msg) => {
            print_usage(program, opts);
            fail!("{}", msg);
        }
    };

    if args.show_help {
        print_usage(program, opts);
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
            box io::File::create(&path) as Box<Writer>
        }
        None => {
            box io::stdout() as Box<Writer>
        }
    };

    if args.header {
        backend.header(output);
        return;
    }

    let mut input = match args.filename {
        Some(ref filename) => {
            let path = Path::new(filename.as_slice());
            box io::File::open(&path) as Box<Reader>
        }
        None => {
            box io::stdin() as Box<Reader>
        }
    };

    let src = match input.read_to_end() {
        Ok(s) => s,
        Err(f) => {
            fail!("Read error: {} ({})", f, args.filename.unwrap_or("stdin".into_owned()));
        }
    };
    let src = str::from_utf8(src.as_slice()).unwrap();

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
