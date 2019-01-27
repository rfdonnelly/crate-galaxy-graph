use serde::Deserialize;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::{self, File};
use std::process::{Command, Stdio};
use std::error::Error;

#[derive(Deserialize)]
#[allow(dead_code)]
struct CrateInfo {
    name: String,
    vers: String,
    deps: Vec<DepInfo>,
    cksum: String,
    features: HashMap<String, Vec<String>>,
    yanked: bool,
}

#[derive(Deserialize)]
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

fn fetch_index() -> Result<(), Box<dyn Error>> {
    match fs::metadata("crates.io-index") {
        Err(_) => {
            eprintln!("Cloning crates.io-index...");
            // Ideally, we'd direct stdout to stderr but there is not a convenient way to do
            // this.  See https://www.reddit.com/r/rust/comments/adaj2f/how_to_pipe_child_process_stdout_to_stderr/
            // for alternatives.  Ignore stdout instead.
            Command::new("git")
                .arg("clone")
                .arg("--depth").arg("1")
                .arg("https://github.com/rust-lang/crates.io-index")
                .stdout(Stdio::null())
                .spawn()?.wait()?;
        }
        Ok(_) => {
            eprintln!("Pulling crates.io-index...");
            Command::new("git")
                .arg("pull")
                .stdout(Stdio::null())
                .spawn()?.wait()?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    fetch_index()?;

    let mut opts = glob::MatchOptions::new();
    opts.require_literal_leading_dot = true;

    let mut edges = vec![];

    let index_paths1 = glob::glob_with("crates.io-index/*/*/*", &opts).unwrap();
    let index_paths2 = glob::glob_with("crates.io-index/[12]/*", &opts).unwrap();

    for path in index_paths1.chain(index_paths2) {
        let path = path.unwrap();

        let file = File::open(&path)?;
        let last_line = BufReader::new(file).lines().last().unwrap()?;
        let crate_info: CrateInfo = serde_json::from_str(&last_line)?;

        let mut deps = crate_info.deps.iter()
            // remove dev dependencies
            .filter(|d| d.kind.as_ref().map_or(true, |s| &**s != "dev"))
            // we only need the name
            .map(|d| &d.name)
            .collect::<Vec<_>>();

        // remove any duplicates
        deps.sort();
        deps.dedup();

        // register all the dependencies
        for &dep_name in deps.iter() {
            edges.push((crate_info.name.clone(), dep_name.clone()));
        }
    }

    // it would be nice to use the `graphviz` crate, but that doesn't
    // seem to allow attaching arbitrary attributes at the moment.
    println!("digraph cratesio {{");
    for &(ref source, ref target) in edges.iter() {
        println!("  \"{}\" -> \"{}\"", source, target)
    }
    println!("}}");

    Ok(())
}
