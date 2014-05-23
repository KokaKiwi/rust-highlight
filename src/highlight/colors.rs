use collections::HashMap;

pub fn get_colors() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    map.insert("kw", "8959A8");
    map.insert("kw-2", "4271AE");
    map.insert("prelude-ty", "4271AE");
    map.insert("number", "718C00");
    map.insert("string", "718C00");
    map.insert("self", "C82829");
    map.insert("boolval", "C82829");
    map.insert("prelude-val", "C82829");
    map.insert("attribute", "C82829");
    // map.insert("ident", "C82829");
    map.insert("comment", "8E908C");
    map.insert("doccomment", "4D4D4C");
    map.insert("macro", "3E999F");
    map.insert("macro-nonterminal", "3E999F");
    map.insert("lifetime", "B76514");

    map
}

pub fn get_types() -> Vec<~str> {
    get_colors().keys().map(|k| k.into_owned()).collect()
}

pub fn get_color(ty: &str) -> Option<~str> {
    get_colors().find_equiv::<&str>(&ty).map(|k| k.to_owned())
}
