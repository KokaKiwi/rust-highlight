use std::io::IoResult;

use collections::HashMap;

use backend::Backend;
use colors;

pub struct LatexBackend;

impl LatexBackend {
    pub fn new() -> LatexBackend {
        LatexBackend
    }
}

static HEADER: &'static str = "\
\\usepackage{xcolor}
\\usepackage{fancyvrb}
\\newcommand{\\VerbBar}{|}
\\newcommand{\\VERB}{\\Verb[commandchars=\\\\\\{\\}]}
\\DefineVerbatimEnvironment{Highlighting}{Verbatim}{commandchars=\\\\\\{\\}}
% Add ',fontsize=\\small' for more characters per line
\\newenvironment{Shaded}{}{}
";

impl Backend for LatexBackend {
    fn configure(&mut self, _vars: &HashMap<~str, ~str>) -> Result<(), ~str> {
        Ok(())
    }

    fn header(&mut self, w: &mut Writer) -> IoResult<()> {
        try!(w.write_str(HEADER));

        for (ty, color) in colors::get_colors().iter() {
            try!(writeln!(w, "\\\\definecolor\\{{}\\}\\{HTML\\}\\{{}\\}", ty, color));
        }

        Ok(())
    }

    fn code_start(&mut self, w: &mut Writer) -> IoResult<()> {
        try!(w.write_line("\\begin{Shaded}"));
        try!(w.write_line("\\begin{Highlighting}[]"));

        Ok(())
    }

    fn code_end(&mut self, w: &mut Writer) -> IoResult<()> {
        try!(w.write_line("\\end{Highlighting}"));
        try!(w.write_line("\\end{Shaded}"));

        Ok(())
    }

    fn start(&mut self, w: &mut Writer, ty: &str) -> IoResult<()> {
        if colors::get_types().contains(&ty.to_owned()) {
            try!(write!(w, "\\\\textcolor\\{{}\\}\\{", ty));
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

        if colors::get_types().contains(&ty.to_owned()) {
            try!(w.write_str("}"));
        }

        Ok(())
    }

    fn text(&mut self, w: &mut Writer, text: &str) -> IoResult<()> {
        fn escape_latex(text: &str) -> ~str {
            let mut result = ~"";

            let mut escape = false;
            for c in text.chars() {
                if escape {
                    result.push_str("\\textbackslash{");
                }

                if c == '{' || c == '}' {
                    result.push_char('\\');
                }

                if c == '\\' {
                    escape = true;
                } else {
                    result.push_char(c);
                }

                if escape && c != '\\' {
                    result.push_char('}');
                    escape = false;
                }
            }

            result
        }
        let text = escape_latex(text);

        try!(w.write_str(text));

        Ok(())
    }
}
