use std::fmt;

#[derive(Debug, Default, PartialEq)]
pub struct Diff {
    /// the total in seconds
    total_time: i64,
}

impl Diff {
    /**
     * Creates a new diff by subtracting the two arguments.
     * Both arguments have to be in seconds.
     */
    pub fn new(working_time: i64, breaking_time: i64) -> Self {
        Self {
            total_time: working_time - breaking_time,
        }
    }

    pub fn get_hours(&self) -> f64 {
        self.total_time as f64 / 60.0 / 60.0
    }
}

impl fmt::Display for Diff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Time in hours {:.3}",
            self.get_hours()
        )
    }
}
