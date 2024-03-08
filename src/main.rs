use git2::{DiffOptions, Repository, Tree};

fn main() {
	let repo = Repository::open(".").unwrap();

	let mut revwalk = repo.revwalk().unwrap();

	revwalk.push_head().unwrap();

	let mut last: Option<Tree> = None;

	for c in revwalk {
		let c = c.unwrap();
		let c = repo.find_commit(c).unwrap();
		let t = c.tree().unwrap();

		let diff = repo
			.diff_tree_to_tree(
				last.as_ref(),
				Some(&t),
				Some(&mut DiffOptions::new()),
			)
			.unwrap();

		let s = diff.stats();

		println!("{:?}", s);

		last = Some(t);
	}
}
