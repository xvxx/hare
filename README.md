# The March Hare

<img src="./img/hare1.gif" width="300" align="right" />

Hare is a simple HTTP server that turns [Hatter] templates into
HTML on-the-fly. It can be used for prototyping or developing static
sites quickly, with no compilation necessary.

Just start `hare` by pointing it at a directory of `*.hat` files, as
well as any static assets you're using (like `*.gif`), and have at!

## Installation

For now you can install hare using [cargo]:

    cargo install --git https://github.com/xvxx/hare

Then run it on a directory of `*.hat` files:

    hare .
    open http://0.0.0.0:8185/

That's it! You can also clone the repository and try it out using the
included `example` directory:

    git clone https://github.com/xvxx/hare
    cd hare
    cargo run example/
    open http://0.0.0.0:8185/

[cargo]: https://rustup.rs/
[hatter]: https://github.com/xvxx/hatter
