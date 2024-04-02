use std::path::PathBuf;

use clap::{ArgAction, Parser};
use git2::{DiffOptions, Repository, Sort, Tree};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Options {
	/// Sets the path to the repository
	#[arg(default_value_t = String::from("."))]
	repository: String,

	/// Filenames to ignore from statistics
	#[arg(short, long, action = ArgAction::Append, group = "ignore")]
	ignore: Vec<String>,

	/// Path to a file that lists filenames to ignore
	#[arg(short = 'I', long, group = "ignore")]
	ignore_file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
	let options = Options::parse();

	let mut diff_option = options.ignore.into_iter().fold(
		DiffOptions::new(),
		|mut diff_option, pathspec| {
			diff_option.pathspec(pathspec);
			diff_option
		},
	);

	let repo = Repository::open(options.repository)?;

	let mut revwalk = repo.revwalk()?;

	revwalk.set_sorting(Sort::REVERSE)?;
	revwalk.push_head()?;

	let mut last: Option<Tree> = None;

	let mut loc: isize = 0;

	for oid in revwalk {
		let t = repo.find_commit(oid?)?.tree()?;

		let s = repo
			.diff_tree_to_tree(
				last.as_ref(),
				Some(&t),
				Some(&mut diff_option),
			)?
			.stats()?;

		loc += s.insertions() as isize;
		loc -= s.deletions() as isize;
		println!("{}", loc);

		last = Some(t);
	}

	Ok(())
}
