mod core;
mod diff;

#[cfg(test)]
mod tests;

use argh::FromArgs;
use log::debug;

use diff::diff;

/// Calculate the duration of given time ranges.
#[derive(Debug, FromArgs)]
struct Cli {
    /// time ranges for work
    #[argh(option)]
    work: String,

    /// time ranges for pauses
    #[argh(option)]
    pause: Option<String>,
}

fn main() {
    env_logger::init();
    let cli: Cli = argh::from_env();
    debug!("{:?}", cli);

    let result = diff(cli.work, cli.pause.unwrap_or(String::new()));
	if let Err(ref err) = result {
		eprintln!("An error occurred: {}", err);
		err.chain()
			.skip(1)
			.for_each(|cause| eprintln!("because: {}", cause));
	}

	println!("{}", result.unwrap());
}
