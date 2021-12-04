use crate::core::Diff;
use anyhow::bail;
use log::debug;

use chrono::Duration;
use chrono::NaiveTime;

use anyhow::{Context, Result};

type From = NaiveTime;
type To = NaiveTime;

pub fn diff(working_input: impl AsRef<str>, pausing_input: impl AsRef<str>) -> Result<Diff> {
    let work_ranges = parse_input(working_input)?;
    let pause_ranges = parse_input(pausing_input)?;

    debug!("work_ranges {:#?}", work_ranges);
    debug!("pause_ranges {:#?}", pause_ranges);

    let work_duration = duration(work_ranges)?;
    let pause_duration = duration(pause_ranges)?;

    debug!("work_duration {:#?}", work_duration);
    debug!("pause_duration {:#?}", pause_duration);

    // in seconds
    let working_time: i64 = work_duration.into_iter().map(|x| x.num_seconds()).sum();
    let breaking_time: i64 = pause_duration.into_iter().map(|x| x.num_seconds()).sum();

    Ok(Diff::new(working_time, breaking_time))
}

/// Extracts the time ranges from the given input. Different time ranges have to be
/// delimited by ";".
fn parse_input(input: impl AsRef<str>) -> Result<Vec<(From, To)>> {
    let mut times = Vec::new();

    if input.as_ref().trim() == "" {
        return Ok(times);
    }

    for line in input.as_ref().split(";").into_iter() {
        let (from, to) = extract_range(line)?;
        times.push((from, to));
    }

    Ok(times)
}

fn extract_range(input: impl AsRef<str>) -> Result<(From, To)> {
    let time_split: Vec<_> = input.as_ref().split("-").collect();

    debug!("extracted range {:?}", time_split);
    if time_split.len() == 1 {
        bail!("Invalid time");
    }

    let first = parse_time(time_split.get(0).unwrap()).context("When parsing the starting time")?;
    let second = parse_time(time_split.get(1).unwrap()).context("When parsing the ending time")?;

    Ok((first, second))
}

fn parse_time(s: impl AsRef<str>) -> Result<NaiveTime> {
    let fmt = "%H:%M";
    NaiveTime::parse_from_str(s.as_ref().trim(), fmt).context("Cannot parse the time")
}

fn duration(times: Vec<(From, To)>) -> Result<Vec<Duration>> {
    Ok(times.into_iter().map(|(x, y)| y - x).collect())
}