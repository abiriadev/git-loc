use std::path::PathBuf;

use clap::{ArgAction, Parser};
use git2::{DiffOptions, Repository, Sort, Tree};

struct Loc {
	time: i64,
	loc: isize,
}

#[derive(Parser)]
#[command(author, version, about)]
struct Options {
	/// Sets the path to the repository
	#[arg(default_value_t = String::from("."))]
	repository: String,

	/// Filenames to ignore from statistics
	#[arg(short, long, action = ArgAction::Append, group = "i")]
	ignore: Vec<String>,

	/// Path to a file that lists filenames to ignore
	#[arg(short = 'I', long, group = "i")]
	ignore_file: Option<PathBuf>,
}

fn count_loc(options: Options) -> anyhow::Result<Vec<Loc>> {
	let mut diff_option = options.ignore.into_iter().fold(
		DiffOptions::new(),
		|mut diff_option, pathspec| {
			diff_option.pathspec(pathspec);
			diff_option
		},
	);

	let repo = Repository::open(options.repository)?;

	let mut revwalk = repo.revwalk()?;

	revwalk.set_sorting(Sort::TIME)?;
	revwalk.set_sorting(Sort::REVERSE)?;
	revwalk.simplify_first_parent()?;
	revwalk.push_head()?;

	let mut last: Option<Tree> = None;

	let mut loc: isize = 0;
	let mut locs = Vec::new();

	for oid in revwalk {
		let commit = repo.find_commit(oid?)?;
		let time = commit.time().seconds();
		let tree = commit.tree()?;

		let s = repo
			.diff_tree_to_tree(
				last.as_ref(),
				Some(&tree),
				Some(&mut diff_option),
			)?
			.stats()?;

		loc += s.insertions() as isize;
		loc -= s.deletions() as isize;

		locs.push(Loc { time, loc });

		last = Some(tree);
	}

	Ok(locs)
}

fn main() -> anyhow::Result<()> {
	let options = Options::parse();

	let locs = count_loc(options)?;

	for loc in locs {
		println!("{} {}", loc.time, loc.loc);
	}

	Ok(())
}
