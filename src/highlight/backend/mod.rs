use std::io::IoResult;

use collections::HashMap;

pub mod html;
pub mod json;
pub mod latex;

pub enum BackendType {
    Html,
    Json,
    Latex,
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

pub fn new_backend(ty: BackendType) -> Box<Backend> {
    match ty {
        Html => box html::HtmlBackend::new() as Box<Backend>,
        Json => box json::JsonBackend::new() as Box<Backend>,
        Latex => box latex::LatexBackend::new() as Box<Backend>,
    }
}
