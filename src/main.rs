use phylo::tree::{RootedPhyloTree, simple_rtree::*};
use rayon::iter::ParallelIterator;
use itertools::Itertools;
use std::env;
use std::{sync::Mutex, fs::OpenOptions};
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;


fn count_combinations(n: u64, r: u64) -> u64 {
    (n - r + 1..=n).product::<u64>() / (1..=r).product::<u64>()
}

fn write_tree_quarts(fname: String){
	let file = OpenOptions::new()
		.write(true)
		.append(true)
		.open(format!("{}.quarts", fname))
		.unwrap();	

	let file = Mutex::new(file);

	let binding = std::fs::read_to_string(fname)
		.expect("Should have been able to read the file");
	let mut contents: Vec<String> = binding.lines().map(String::from)  // make each slice into a string
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

	// let out_string: Vec<String> = quart_iter.collect();
	// for quartet in out_string{
	// 	writeln!(file, "{}", quartet);
	// 	pb.inc(1);
	// }
	println!("Saved");
}
fn main() {
	let args: Vec<String> = env::args().collect();
	println!("writing quartets of {} to {}.quarts", &args[1], &args[1]);
	write_tree_quarts("/home/sriramv/Projects/tree_project/get_quarts/asteroid.tree".to_string())
}
