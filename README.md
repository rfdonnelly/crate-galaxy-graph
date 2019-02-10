# Crates.io ngraph

Creates a dependency graph of all crates on crates.io for visualization in [Code
Galaxies].

[Code Galaxies]: https://anvaka.github.io/pm

## Steps

### Download crates.io registery and create graph

```sh
cargo run --release > out.dot
```

### Convert graph to ngraph format

```sh
(cd ngraph && npm install)
ngraph/index.js out.dot
```

## Attribution

The Rust program that generates a GraphViz digraph was forked from
[crates.io-graph].  It was then ported to Rust 2018 and trimmed for input to
ngraph.fromdot.

[crates.io-graph]: https://github.com/huonw/crates.io-graph
