rust-highlight
==============

Travis: [![Build Status](https://travis-ci.org/KokaKiwi/rust-highlight.svg?branch=master)](https://travis-ci.org/KokaKiwi/rust-highlight)

rust-highlight is small Rust tool you can use to get highlighted Rust code in HTML, LaTeX, or any other implemented backend.
Actually, the implemented backends are HTML, LaTeX and JSON.

Installation
------------

### Compile

~~~bash
git clone --recursive git://github.com/KokaKiwi/rust-highlight.git
cd rust-highlight
make
~~~

If you want to install to install dir (~/.rust):

~~~bash
make install
~~~

Usage
-----

~~~bash
$ ./rshighlight -h
Usage: Usage: ./rshighlight [-h] [-o FILENAME] [--header] [--html] [--json] [--latex] [-v KEY=VAL].. [filename]

    Small Rust tool to output highlighted Rust code.

Options:
    -h --help           Show this help and exit.
    -o --output FILENAME
                        Output filename.
    --header            Output head to put before highlighted code.
    --html              Output HTML code.
    --json              Output JSON code.
    --latex             Output LaTeX code.
    -v --var KEY=VAL    Set backend-specific variables.
~~~

~~~bash
./rshighlight --header code.rs >> code.html     # To generate used CSS classes
./rshighlight code.rs >> code.html              # To generate highlighted code.
~~~

Backends options
----------------

### HTML

- `use_classes` (boolean): Use CSS classes for highlighted code or not (use `class="..."` or `style="color: #...;`)

### JSON

- `pretty` (boolean): Output pretty JSON or not.

### LaTeX

There is no options actually.
