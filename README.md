# rust-calc

[![pipeline status](https://gitlab.com/jrop/rust-calc/badges/master/pipeline.svg)](https://gitlab.com/jrop/rust-calc/commits/master)

A simple calculator CLI, written in Rust.

## Building

```sh
$ cargo build
# or
$ cargo build --release
```

## Running

When run, a REPL is started that parses math expressions and computes the result:

```sh
$ rust-calc
> 1 + 2 * 3
result=3.0
> (1+2)*3
result=9.0
> 1^2^3
result=1.0
> sin(pi / 6)
result=0.5
```

The following constants are supported:

- `e`
- `pi`

The following functions are supported:

- `abs`
- `acos`
- `asin`
- `atan`
- `ceil`
- `cos`
- `floor`
- `ln`
- `log`
- `log2`
- `sin`
- `tan`

## How It Works

Each line that is entered is lexed into tokens, which are then parsed using [Pratt-Parsing](http://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/) to produce an AST. The AST is then visited in the right order to calculate a result, which is printed to standard-out.

## License (MIT)

Copyright 2018 Jonathan Apodaca <jrapodaca@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
