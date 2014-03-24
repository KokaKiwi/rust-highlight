use std::io::IoResult;

use collections::HashMap;
use serialize::{json, Encodable, Encoder};

use backend::Backend;

pub struct JsonBackend {
    pretty: bool,

    entries: Vec<Entry>,
}

#[deriving(Encodable)]
struct Entry {
    ty: ~str,
    value: ~str,
}

impl JsonBackend {
    pub fn new() -> JsonBackend {
        JsonBackend {
            pretty: false,

            entries: Vec::new(),
        }
    }
}

impl Backend for JsonBackend {
    fn configure(&mut self, vars: &HashMap<~str, ~str>) -> Result<(), ~str> {
        match vars.find(&~"pretty") {
            Some(ref value) => {
                self.pretty = match from_str(value.as_slice()) {
                    Some(b) => b,
                    None => {
                        return Err(format!("Bad value for pretty: {}", value));
                    }
                }
            }
            None => {}
        }

        Ok(())
    }

    fn header(&mut self, _w: &mut Writer) -> IoResult<()> {
        Ok(())
    }

    fn code_start(&mut self, _w: &mut Writer) -> IoResult<()> {
        Ok(())
    }

    fn code_end(&mut self, w: &mut Writer) -> IoResult<()> {
        if self.pretty {
            let mut encoder = json::PrettyEncoder::new(w);
            self.entries.encode(&mut encoder);
        } else {
            let mut encoder = json::Encoder::new(w);
            self.entries.encode(&mut encoder);
        }

        Ok(())
    }

    fn start(&mut self, _w: &mut Writer, ty: &str) -> IoResult<()> {
        self.entries.push(Entry {
            ty: ~"start",
            value: ty.to_owned(),
        });

        Ok(())
    }

    fn end(&mut self, _w: &mut Writer, ty: &str) -> IoResult<()> {
        self.entries.push(Entry {
            ty: ~"end",
            value: ty.to_owned(),
        });

        Ok(())
    }

    fn text(&mut self, _w: &mut Writer, text: &str) -> IoResult<()> {
        self.entries.push(Entry {
            ty: ~"text",
            value: text.to_owned(),
        });

        Ok(())
    }
}