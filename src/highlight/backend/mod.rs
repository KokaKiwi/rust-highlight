use std::io::IoResult;

use collections::HashMap;

pub mod html;
pub mod json;

pub enum BackendType {
    Html,
    Json,
}

pub trait Backend {
    fn configure(&mut self, vars: &HashMap<~str, ~str>) -> Result<(), ~str>;

    fn header(&mut self, w: &mut Writer) -> IoResult<()>;

    fn code_start(&mut self, w: &mut Writer) -> IoResult<()>;
    fn code_end(&mut self, w: &mut Writer) -> IoResult<()>;

    fn start(&mut self, w: &mut Writer, ty: &str) -> IoResult<()>;
    fn end(&mut self, w: &mut Writer, ty: &str) -> IoResult<()>;
    fn text(&mut self, w: &mut Writer, text: &str) -> IoResult<()>;
}

pub fn new_backend(ty: BackendType) -> ~Backend {
    match ty {
        Html => ~html::HtmlBackend::new() as ~Backend,
        Json => ~json::JsonBackend::new() as ~Backend,
    }
}
