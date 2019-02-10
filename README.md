# Crates.io ngraph

Creates a dependency graph of all crates on crates.io for visualization in [Code Galaxies].

[![Build Status](https://travis-ci.org/rfdonnelly/crate-galaxy-graph.svg?branch=master)](https://travis-ci.org/rfdonnelly/crate-galaxy-graph)

[Code Galaxies]: https://anvaka.github.io/pm

## Dependencies

For creating a ngraph for crates.io:

* Rust
* NodeJS

For deploy:

* Ruby

## Steps

1. Create a GraphViz digraph for crates.io

   ```sh
   cargo run --release > out.dot
   ```

2. Convert GraphViz to ngraph

   ```sh
   (cd ngraph && npm install)
   ngraph/index.js out.dot
   ```

3. Deploy

   Runs in Travis CI only.

   ```sh
   ./deploy
   ```

## Attribution

The Rust program that generates a GraphViz digraph was forked from [crates.io-graph].
It was then ported to Rust 2018 and modified for input to ngraph.fromdot.

[crates.io-graph]: https://github.com/huonw/crates.io-graph
