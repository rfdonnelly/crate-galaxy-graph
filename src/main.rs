#![feature(io, path, path_ext, std_misc)]
extern crate glob;
extern crate "rustc-serialize" as rustc_serialize;

use std::collections::{HashMap, HashSet, hash_map};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::process::Command;

const MAX_REV_DEP_COUNT: usize = 100;

#[derive(RustcDecodable)]
#[allow(dead_code)]
struct CrateInfo {
    name: String,
    vers: String,
    deps: Vec<DepInfo>,
    cksum: String,
    features: HashMap<String, Vec<String>>,
    yanked: bool,
}

#[derive(RustcDecodable)]
#[allow(dead_code)]
struct DepInfo {
    name: String,
    req: String,
    features: Vec<String>,
    optional: bool,
    default_features: bool,
    target: Option<String>,
    kind: Option<String>
}

// shallowly download the index, if necessary
fn fetch_index() {
    if std::path::PathBuf::new("crates.io-index").exists() {
        return
    }

    Command::new("git")
        .arg("clone")
        .arg("--depth").arg("1")
        .arg("https://github.com/rust-lang/crates.io-index")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn main() {
    fetch_index();

    let mut opts = glob::MatchOptions::new();
    opts.require_literal_leading_dot = true;

    let mut crates = vec![];
    let mut edges = vec![];
    let mut interacts = HashSet::new();
    let mut rev_dep_count = HashMap::new();

    let index_paths1 = glob::glob_with("crates.io-index/*/*/*", &opts).unwrap();

    let index_paths2 = glob::glob_with("crates.io-index/[12]/*", &opts).unwrap();

    for path in index_paths1.chain(index_paths2) {
        let path = path.unwrap();

        let file = File::open(&path).unwrap();
        let last_line = BufReader::new(file).lines().last().unwrap().unwrap();
        let crate_info: CrateInfo = rustc_serialize::json::decode(&*last_line).unwrap();

        crates.push(crate_info.name.clone());

        let mut deps = crate_info.deps.iter()
            // remove dev dependencies
            .filter(|d| d.kind.as_ref().map_or(true, |s| &**s != "dev"))
            // we only need the name
            .map(|d| &d.name)
            .collect::<Vec<_>>();

        // it has dependencies, it links into the package ecosystem, yay!
        if !deps.is_empty() {
            interacts.insert(crate_info.name.clone());
        }

        // remove any duplicates
        deps.sort();
        deps.dedup();

        // register all the dependencies
        for &dep_name in deps.iter() {
            interacts.insert(dep_name.clone());
            edges.push((crate_info.name.clone(), dep_name.clone()));

            let count = match rev_dep_count.entry(dep_name.clone()) {
                hash_map::Entry::Occupied(o) => o.into_mut(),
                hash_map::Entry::Vacant(v) => v.insert(0)
            };
            *count += 1;
        }
    }

    crates.retain(|name| // interacts.contains(name) && // done externally
                  rev_dep_count.get(name).map_or(true, |n| *n <= MAX_REV_DEP_COUNT));
    edges.retain(|&(ref source, ref target)|
                 rev_dep_count.get(source).map_or(true, |n| *n <= MAX_REV_DEP_COUNT) &&
                 rev_dep_count.get(target).map_or(true, |n| *n <= MAX_REV_DEP_COUNT));

    // it would be nice to use the `graphviz` crate, but that doesn't
    // seem to allow attaching arbitrary attributes at the moment.
    println!("digraph cratesio {{");
    for krate in crates.iter() {
        let count = rev_dep_count.get(krate).map_or(0, |n| *n);
        println!("  {ident}[label=\"{name}\" href=\"https://crates.io/crates/{name}\" fontsize={size}]",
                 ident = krate.replace("-", "_"),
                 name = krate,
                 size = 14.0 + count as f64 / 2.0);
    }
    for &(ref source, ref target) in edges.iter() {
        println!("  {} -> {}", source.replace("-", "_"), target.replace("-", "_"))
    }

    println!("}}");
}
