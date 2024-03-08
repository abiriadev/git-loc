use git2::{DiffOptions, Repository, Sort, Tree};

fn main() -> anyhow::Result<()> {
	let repo = Repository::open(".")?;

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
				Some(&mut DiffOptions::new()),
			)?
			.stats()?;

		loc += s.insertions() as isize;
		loc -= s.deletions() as isize;
		println!("{}", loc);

		last = Some(t);
	}

	Ok(())
}
