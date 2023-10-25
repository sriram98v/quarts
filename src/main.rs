use phylo::tree::{RootedPhyloTree, simple_rtree::*};
use clap::{arg, Command};
use rayon::iter::ParallelIterator;
use itertools::Itertools;
use std::thread::available_parallelism;
use std::{sync::Mutex, fs::OpenOptions};
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::Path;


fn count_combinations(n: u64, r: u64) -> u64 {
    (n - r + 1..=n).product::<u64>() / (1..=r).product::<u64>()
}

fn write_tree_quarts(fname: &String){

	let out_fname = format!("{}.quarts", fname);

	if Path::new(&out_fname).exists() {
		std::fs::remove_file(out_fname.clone()).unwrap();
	}

	let file = OpenOptions::new()
		.create_new(true)
		.write(true)
		.open(out_fname)
		.unwrap();	

	let file = Mutex::new(file);

	let binding = std::fs::read_to_string(fname)
		.expect("Should have been able to read the file");
	let mut contents: Vec<String> = binding.lines().map(String::from)
		.collect();
	let mut tree = RootedPhyloTree::from_newick(contents.pop().expect("No trees found!"));
	tree.unweight();
	let binding = tree.get_leaves(tree.get_root());
	let num_leaves = binding.len();
	let pb = ProgressBar::new(count_combinations(num_leaves as u64, 4));
	pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
		.unwrap()
		.progress_chars("#>-"));

	binding
            .into_iter()
			.map(|(_, node)| node.taxa())
			.combinations(4)
			.par_bridge()
			.map(|x| {
				format!("{}\n",tree.induce_tree(x).to_newick())
			})
			.for_each(|x| {
				file.lock().unwrap().write_fmt(format_args!("{}\n", x)).expect("Failed!");
				pb.inc(1);
			})
			;

	println!("Saved");
}
fn main() {
	let matches = Command::new("Generate Quartets")
        .version("1.0")
        .author("Sriram Vijendran <vijendran.sriram@gmail.com>")
		.about("Write all quartets of file to .quart file")
		.arg(arg!(<SRC_FILE> "Source file with input tree (Will automatically use last tree of file is more than one is present)")
			.required(true)
			)
		.arg(arg!(-n --num <NUM_T> "Number of threads")
			.required(true)
			.value_parser(clap::value_parser!(usize))
			)
        .get_matches();
	let tree_file = matches.get_one::<String>("SRC_FILE").expect("Please provide tree file!");
	let num_threads = matches.get_one::<usize>("num");
	match num_threads{
		Some(n) => {rayon::ThreadPoolBuilder::new().num_threads(*n).build_global().unwrap()}
		None => {rayon::ThreadPoolBuilder::new().num_threads(available_parallelism().unwrap().get()).build_global().unwrap()}
	};
	println!("writing quartets of {} to {}.quarts", &tree_file, &tree_file);
	write_tree_quarts(tree_file);
}
