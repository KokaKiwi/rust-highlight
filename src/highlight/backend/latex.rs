use std::io::IoResult;

use collections::HashMap;

use backend::Backend;
use colors;

pub struct LatexBackend {
    contexts: Vec<~str>,
}

impl LatexBackend {
    pub fn new() -> LatexBackend {
        LatexBackend {
            contexts: Vec::new(),
        }
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
        if ty != "comment" {
            if colors::get_types().contains(&ty.to_owned()) {
                try!(write!(w, "\\\\textcolor\\{{}\\}\\{", ty));
            }

            if ty == "attribute" {
                try!(w.write_str("#"));
            }
        }

        self.contexts.push(ty.to_owned());

        Ok(())
    }

    fn end(&mut self, w: &mut Writer, ty: &str) -> IoResult<()> {
        if ty != "comment" {
            if ty == "attribute" {
                try!(w.write_str("]"));
            }

            if colors::get_types().contains(&ty.to_owned()) {
                try!(w.write_str("}"));
            }
        }

        self.contexts.pop();

        Ok(())
    }

    fn text(&mut self, w: &mut Writer, text: &str) -> IoResult<()> {
        fn escape_latex(text: &str) -> ~str {
            let mut result = StrBuf::new();

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

            result.into_owned()
        }

        fn escape_comment(text: &str, has_color: bool) -> ~str {
            let mut result = StrBuf::new();

            let mut first = true;
            for line in text.lines() {
                if !first {
                    result.push_str("\n");
                }

                if line.len() > 0 && has_color {
                    result.push_str("\\textcolor{comment}{");
                }

                result.push_str(line);

                if line.len() > 0 && has_color {
                    result.push_str("}");
                }

                first = false;
            }

            let old_len = text.len();
            let text = text.trim_right_chars('\n').to_owned();
            let new_len = text.len();

            range(0, old_len - new_len).advance(|_| {
                result.push_char('\n');

                true
            });

            result.into_owned()
        }

        let context = self.contexts.last().unwrap();
        let has_color = colors::get_types().contains(context);
        let context = context.as_slice();

        let text = if context == "comment" {
            escape_comment(text, has_color)
        } else {
            escape_latex(text)
        };
        try!(w.write_str(text));

        Ok(())
    }
}
