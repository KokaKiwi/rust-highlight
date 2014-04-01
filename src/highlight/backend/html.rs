// Taken and modified from https://github.com/mozilla/rust/blob/master/src/librustdoc/html/highlight.rs
use std::io::IoResult;
use std::str;

use collections::HashMap;

use colors;
use backend::Backend;

pub struct HtmlBackend {
    pub use_classes: bool,
}

impl HtmlBackend {
    pub fn new() -> HtmlBackend {
        HtmlBackend {
            use_classes: false,
        }
    }
}

fn escape_html(text: &str) -> ~str {
    let text = str::replace(text, "&", "&amp;");
    let text = str::replace(text, "<", "&lt;");
    let text = str::replace(text, ">", "&gt;");
    let text = str::replace(text, "\"", "&quot;");
    let text = str::replace(text, "'", "&#39;");

    text.to_owned()
}

impl Backend for HtmlBackend {
    fn configure(&mut self, vars: &HashMap<~str, ~str>) -> Result<(), ~str> {
        match vars.find(&~"use_classes") {
            Some(value) => {
                self.use_classes = match from_str(value.as_slice()) {
                    Some(v) => v,
                    None => {
                        return Err(format!("Bad `use_classes` value: {}", value));
                    }
                };
            }
            None => {}
        }

        Ok(())
    }

    fn header(&mut self, w: &mut Writer) -> IoResult<()> {
        if self.use_classes {
            try!(w.write_line("<style>"));
            for ty in colors::get_types().iter() {
                let color = colors::get_color(ty.as_slice()).unwrap();
                try!(writeln!(w, "    pre.rust .{} \\{ color: \\#{}; \\}", ty, color));
            }
            try!(w.write_line("</style>"));
        }

        Ok(())
    }

    fn code_start(&mut self, w: &mut Writer) -> IoResult<()> {
        try!(w.write_str("<pre class=\"rust\">"));

        Ok(())
    }

    fn code_end(&mut self, w: &mut Writer) -> IoResult<()> {
        try!(w.write_str("</pre>"));

        Ok(())
    }

    fn start(&mut self, w: &mut Writer, ty: &str) -> IoResult<()> {
        if ty != "" && ty != "normal" {
            try!(w.write_str("<span"));

            if self.use_classes {
                try!(write!(w, " class=\"{}\"", ty));
            } else {
                let color = colors::get_color(ty);
                match color {
                    Some(c) => {
                        try!(write!(w, " style=\"color: \\#{};\"", c));
                    }
                    None => {}
                }
            }

            try!(w.write_str(">"));
        }

        if ty == "attribute" {
            try!(w.write_str("#"));
        }

        Ok(())
    }

    fn end(&mut self, w: &mut Writer, ty: &str) -> IoResult<()> {
        if ty == "attribute" {
            try!(w.write_str("]"));
        }

        if ty != "" && ty != "normal" {
            try!(w.write_str("</span>"));
        }

        Ok(())
    }

    fn text(&mut self, w: &mut Writer, text: &str) -> IoResult<()> {
        try!(w.write_str(escape_html(text)));

        Ok(())
    }
}
