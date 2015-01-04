#![feature(old_orphan_check)]
extern crate "graphviz" as dot;
extern crate glob;
extern crate "rustc-serialize" as rustc_serialize;

use std::borrow::IntoCow;
use std::collections::{HashMap, HashSet};
use std::io::{BufferedReader, File};

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

fn main() {
    let mut opts = glob::MatchOptions::new();
    opts.require_literal_leading_dot = true;

    let mut crates = vec![];
    let mut edges = vec![];
    let mut interacts = HashSet::new();

    for path in glob::glob_with("crates.io-index/*/*/*", &opts) {
        let file = File::open(&path).unwrap();
        let last_line = BufferedReader::new(file).lines().last().unwrap().unwrap();
        let crate_info: CrateInfo = rustc_serialize::json::decode(&*last_line).unwrap();

        crates.push(crate_info.name.clone());
        if !crate_info.deps.is_empty() {
            interacts.insert(crate_info.name.clone());
        }
        let mut deps = crate_info.deps.iter()
            .filter(|d| d.kind.as_ref().map_or(true, |s| &**s != "dev"))
            .map(|d| &d.name)
            .collect::<Vec<_>>();

        deps.sort();
        deps.dedup();
        for &dep_name in deps.iter() {
            interacts.insert(dep_name.clone());
            edges.push((crate_info.name.clone(), dep_name.clone()));
        }
    }

    println!("total crates: {}", crates.len());
    crates.retain(|name| interacts.contains(name));
    println!("filtered crates: {}", crates.len());

    let graph = Graph {
        crates: crates,
        deps: edges,
    };
    dot::render(&graph, &mut std::io::stdout()).ok().expect("bad render");
}


struct Graph {
    crates: Vec<String>,
    deps: Vec<(String, String)>,
}

impl<'a> dot::Labeller<'a, String, (String, String)> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("cratesiodeps".to_string()).ok().expect("bad graph id")
    }

    fn node_id(&'a self, n: &String) -> dot::Id<'a> {
        dot::Id::new(n.replace("-", "_")).ok().expect("bad id")
    }

    fn node_label<'b>(&self, n: &String) -> dot::LabelText<'a> {
        dot::LabelStr(format!("<a href=\"https://crates.io/crates/{0}\">{0}</a>", n).into_cow())
    }
}

impl<'a> dot::GraphWalk<'a, String, (String, String)> for Graph {
    fn nodes(&self) -> dot::Nodes<'a, String> {
        self.crates.clone().into_cow()
    }
    fn edges(&self) -> dot::Edges<'a, (String, String)> {
        self.deps.clone().into_cow()
    }

    fn source(&self, e: &(String, String)) -> String {
        e.0.clone()
    }
    fn target(&self, e: &(String, String)) -> String {
        e.1.clone()
    }
}
