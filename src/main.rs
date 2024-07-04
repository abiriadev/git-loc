use std::path::PathBuf;

use anyhow::anyhow;
use clap::{ArgAction, Parser, ValueEnum};
use git2::{DiffOptions, Repository, Sort, Tree};
use rasciigraph::{plot, Config};
use serde::Serialize;
use serde_json::to_string;
use term_size::dimensions;
use time::OffsetDateTime;

type RawTime = i64;

struct LocByTime {
	time: RawTime,
	loc: isize,
}

#[derive(Clone, Copy, Default, ValueEnum)]
enum RenderMode {
	/// Show ascii graph on terminal
	#[default]
	Chart,

	/// Output each data point as ndjson
	Ndjson,
}

struct LocSeriesWindow<'a> {
	series: &'a [LocByTime],
	index: usize,
	start: RawTime,
	duration: RawTime,
}

impl<'a> Iterator for LocSeriesWindow<'a> {
	type Item = &'a [LocByTime];

	fn next(&mut self) -> Option<Self::Item> {
		let p = self.series[self.index..]
			.iter()
			.position(|&LocByTime { time, .. }| {
				time >= self.start + self.duration
			})
			.unwrap_or(self.series[self.index..].len());

		let window = &self.series[self.index..self.index + p];

		self.index += p;
		self.start += self.duration;

		if self.index >= self.series.len() {
			None
		} else {
			Some(window)
		}
	}
}

#[derive(Serialize)]
struct SerializableLocByTime {
	#[serde(with = "time::serde::rfc3339")]
	time: OffsetDateTime,
	lines: isize,
}

struct LocSeries(Vec<LocByTime>);

impl LocSeries {
	fn render(self, options: &Options) -> anyhow::Result<String> {
		match options.format {
			RenderMode::Chart => self.render_chart(options),
			RenderMode::Ndjson => self.render_ndjson(options),
		}
	}

	fn window(&self, start: RawTime, duration: RawTime) -> LocSeriesWindow {
		LocSeriesWindow {
			series: &self.0,
			index: 0,
			start,
			duration,
		}
	}

	fn render_chart(self, options: &Options) -> anyhow::Result<String> {
		let (
			Some(&LocByTime { time: start, .. }),
			Some(&LocByTime { time: end, .. }),
		) = (self.0.first(), self.0.last())
		else {
			return Err(anyhow!(
				"At least one data point is required to render a chart."
			));
		};

		let Some((t_w, t_h)) = dimensions() else {
			return Err(anyhow!(
				"Unable to determine terminal size."
			));
		};

		let slice_count = t_w as i64;
		let full_duration = end - start;
		let slice_duration = full_duration / slice_count;

		let mut last = 0;

		let series = self
			.window(start, slice_duration)
			.map(|window| {
				if !window.is_empty() {
					last = window
						.iter()
						.map(|&LocByTime { loc, .. }| loc)
						.sum::<isize>() / window.len() as isize;
				}

				last
			})
			.map(|i| i as f64)
			.collect::<Vec<_>>();

		Ok(plot(
			series,
			Config::default()
				.with_width(options.width.unwrap_or(t_w - 10) as u32)
				.with_height(options.height.unwrap_or(t_h - 10) as u32)
				.with_caption("LOC over time".to_owned()),
		))
	}

	fn render_ndjson(self, _options: &Options) -> anyhow::Result<String> {
		Ok(self
			.0
			.into_iter()
			.map(|LocByTime { time, loc }| {
				to_string(&SerializableLocByTime {
					time: OffsetDateTime::from_unix_timestamp(time).unwrap(),
					lines: loc,
				})
			})
			.collect::<Result<Vec<_>, _>>()?
			.join("\n"))
	}
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

	/// Output Format
	#[arg(short, long, value_enum, default_value_t)]
	format: RenderMode,

	/// Width of the chart
	#[arg(long)]
	width: Option<usize>,

	/// Height of the chart
	#[arg(long)]
	height: Option<usize>,
}

fn count_loc(options: &Options) -> anyhow::Result<LocSeries> {
	let mut diff_option = options.ignore.iter().fold(
		DiffOptions::new(),
		|mut diff_option, pathspec| {
			diff_option.pathspec(pathspec);
			diff_option
		},
	);

	let repo = Repository::open(&options.repository)?;

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

		locs.push(LocByTime { time, loc });

		last = Some(tree);
	}

	Ok(LocSeries(locs))
}

fn main() -> anyhow::Result<()> {
	let options = Options::parse();

	let locs = count_loc(&options)?;

	let rendered = locs.render(&options)?;

	println!("{}", rendered);

	Ok(())
}
