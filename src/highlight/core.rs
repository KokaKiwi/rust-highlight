use syntax::parse;
use syntax::parse::lexer;
use syntax::codemap::{BytePos, Span};

use t = syntax::parse::token;

#[deriving(Encodable, Show)]
pub enum Part {
    Start(~str),
    End(~str),
    Text(~str, ~str),
}

pub fn highlight(src: &str) -> Vec<Part> {

    let sess = parse::new_parse_sess();
    let fm = parse::string_to_filemap(&sess, src.to_owned(), ~"<stdin>");

    let mut parts = Vec::new();
    do_highlight(&sess,
                 lexer::new_string_reader(&sess.span_diagnostic, fm),
                 &mut parts);

    parts
}

fn do_highlight(sess: &parse::ParseSess, mut lexer: lexer::StringReader, parts: &mut Vec<Part>) {
    use syntax::parse::lexer::Reader;

    let mut last = BytePos(0);
    let mut is_attribute = false;
    let mut is_macro = false;
    let mut is_macro_nonterminal = false;

    loop {
        let next = lexer.next_token();
        let test = if next.tok == t::EOF {
            lexer.pos
        } else {
            next.sp.lo
        };

        if test > last {
            let snip = sess.span_diagnostic.cm.span_to_snippet(Span {
                lo: last,
                hi: test,
                expn_info: None,
            }).unwrap();
            let ty = if snip.contains("/") {
                ~"comment"
            } else {
                ~"normal"
            };
            let part = Text(ty, snip);
            parts.push(part);
        }
        last = next.sp.hi;
        if next.tok == t::EOF {
            break
        }

        let ty = match next.tok {
            // If this '&' token is directly adjacent to another token, assume
            // that it's the address-of operator instead of the and-operator.
            // This allows us to give all pointers their own class (~ and @ are
            // below).
            t::BINOP(t::AND) if lexer.peek().sp.lo == next.sp.hi => "kw-2",
            t::AT | t::TILDE => "kw-2",

            // consider this as part of a macro invocation if there was a
            // leading identifier
            t::NOT if is_macro => { is_macro = false; "macro" }

            // operators
            t::EQ | t::LT | t::LE | t::EQEQ | t::NE | t::GE | t::GT |
                t::ANDAND | t::OROR | t::NOT | t::BINOP(..) | t::RARROW |
                t::BINOPEQ(..) | t::FAT_ARROW => "op",

            // miscellaneous, no highlighting
            t::DOT | t::DOTDOT | t::DOTDOTDOT | t::COMMA | t::SEMI |
                t::COLON | t::MOD_SEP | t::LARROW | t::DARROW | t::LPAREN |
                t::RPAREN | t::LBRACKET | t::LBRACE | t::RBRACE => "",
            t::DOLLAR => {
                if t::is_ident(&lexer.peek().tok) {
                    is_macro_nonterminal = true;
                    "macro-nonterminal"
                } else {
                    ""
                }
            }

            // This is the start of an attribute. We're going to want to
            // continue highlighting it as an attribute until the ending ']' is
            // seen, so skip out early. Down below we terminate the attribute
            // span when we see the ']'.
            t::POUND => {
                is_attribute = true;
                parts.push(Start(~"attribute"));
                continue
            }
            t::RBRACKET => {
                if is_attribute {
                    is_attribute = false;
                    parts.push(End(~"attribute"));
                    continue
                } else {
                    ""
                }
            }

            // text literals
            t::LIT_CHAR(..) | t::LIT_STR(..) | t::LIT_STR_RAW(..) => "string",

            // number literals
            t::LIT_INT(..) | t::LIT_UINT(..) | t::LIT_INT_UNSUFFIXED(..) |
                t::LIT_FLOAT(..) | t::LIT_FLOAT_UNSUFFIXED(..) => "number",

            // keywords are also included in the identifier set
            t::IDENT(ident, _is_mod_sep) => {
                match t::get_ident(ident).get() {
                    "ref" | "mut" => "kw-2",

                    "self" => "self",
                    "false" | "true" => "boolval",

                    "Option" | "Result" => "prelude-ty",
                    "Some" | "None" | "Ok" | "Err" => "prelude-val",

                    _ if t::is_any_keyword(&next.tok) => "kw",
                    _ => {
                        if is_macro_nonterminal {
                            is_macro_nonterminal = false;
                            "macro-nonterminal"
                        } else if lexer.peek().tok == t::NOT {
                            is_macro = true;
                            "macro"
                        } else {
                            "ident"
                        }
                    }
                }
            }

            t::LIFETIME(..) => "lifetime",
            t::DOC_COMMENT(..) => "doccomment",
            t::UNDERSCORE | t::EOF | t::INTERPOLATED(..) => "",
        };
        let ty = ty.to_owned();

        let snip = sess.span_diagnostic.cm.span_to_snippet(next.sp).unwrap();
        let part = Text(ty, snip);
        parts.push(part);
    }
}
