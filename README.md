# Lamb
A lambda calculus evaluator in Rust, with multiple beta reduction strategies.

## Example
Boolean logic:
    let true = \x.\y.x
    let false = \x.\y.y

    let and = \p.\q.p true q

    let main = and true false

Recursivity:
    \\todo
## Syntax
Rules:
* variables must be a lowercase letter
* identifiers
 * are declared with 'let {name} = {term}'
 * consists of letters and digits
 * must start with an uppercase or lowercase letter.
  * length must be at least 2 if it starts with a lowercase letter
* abstraction extends as far right as possible
* application is left associative

## Usage
    lamb [FLAGS] [OPTIONS] [INPUT]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Sets the level of verbosity

OPTIONS:
    -s, --strategy <STRATEGY>    Sets the beta reduction strategy [possible values: normal, applicative, call_by_name,
                                 call_by_value]

ARGS:
    <INPUT>    Sets the input file to use. If no file is given, then start the REPL
## Building
    $ cargo build
