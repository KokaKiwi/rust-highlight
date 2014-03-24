rust-highlight
==============

Travis: [![Build Status](https://travis-ci.org/KokaKiwi/rust-highlight.svg?branch=master)](https://travis-ci.org/KokaKiwi/rust-highlight)

rust-highlight is small Rust tool you can use to get highlighted Rust code in HTML, JSON, or any other implemented backend.
Actually, the implemented backends are HTML and JSON.

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
Usage: ./rshighlight [options] [filename]

  -h --help          Show this help and exit.
  -o --output        Output filename.
  --header           Output head to put before highlighted code.
  --html             Output HTML code.
  --latex            Output LaTeX code.
  -v --var KEY=VAL   Set backend-specific variables.
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
