# tkeyboard

My first attempt at writing stuff in Rust.

 Basically a small tui app to
test the typing speed using the space-separated list of randomly picked
words from the file (words.txt). Highlights the inputted text with red
if the input is already wrong and as soon as user enters the whole string
correctly - shows the time and the typing speed.

## Options
- `-w WORDS` - set the number of words to generate on each attempt
- `-v` - show version 

## Installation

### Depenencies

All you really need is just rust compiler. Mostly you can install it via your package manager looking for `rust` or `rustc` package, or install it
from `rustup`.

### Installation method

Just run `make install`. You will be noted to enter your sudo password,
to move the binary to `/usr/local/bin` directory.

### Uninstallation

Run `make uninstall`.

## TODO

- Check how it works for other OS-es as was made only for Linux
- Find a dictionary and generation method that includes more short words, 
as on practice the words/minute counter may show much slower results than
usually, as the average length of words here is much longer.

