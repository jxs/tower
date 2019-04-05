extern crate tokio_timer;

use hdrhistogram::Histogram;
use std::time::{Duration, Instant};

use tokio_timer::clock;

/// This represents a "rotating" histogram which stores two histogram, one which
/// should be read and one which should be written to.  Every period, the read
/// histogram is discarded and replaced by the write histogram.  The idea here
/// is that the read histogram should always contain a full period (the previous
/// period) of write operations.
pub struct RotatingHistogram {
    read: Histogram<u64>,
    write: Histogram<u64>,
    last_rotation: Instant,
    period: Duration,
}

impl RotatingHistogram {
    pub fn new(period: Duration) -> RotatingHistogram {
        RotatingHistogram {
            read: Histogram::<u64>::new_with_bounds(1, 10_000, 3)
                .expect("Invalid histogram params"),
            write: Histogram::<u64>::new_with_bounds(1, 10_000, 3)
                .expect("Invalid histogram params"),
            last_rotation: clock::now(),
            period,
        }
    }

    pub fn read(&mut self) -> &mut Histogram<u64> {
        self.maybe_rotate();
        &mut self.read
    }

    pub fn write(&mut self) -> &mut Histogram<u64> {
        self.maybe_rotate();
        &mut self.write
    }

    fn maybe_rotate(&mut self) {
        let delta = clock::now() - self.last_rotation;
        // TODO: replace with delta.duration_div when it becomes stable.
        let rotations =
            (Self::duration_as_nanos(&delta) / Self::duration_as_nanos(&self.period)) as u32;
        if rotations >= 2 {
            trace!("Time since last rotation is {:?}.  clearing!", delta);
            self.clear();
        } else if rotations == 1 {
            trace!("Time since last rotation is {:?}. rotating!", delta);
            self.rotate();
        }
        self.last_rotation += self.period * rotations;
    }

    fn rotate(&mut self) {
        std::mem::swap(&mut self.read, &mut self.write);
        trace!("Rotated {:?} points into read", self.read.len());
        self.write.clear();
    }

    fn clear(&mut self) {
        self.read.clear();
        self.write.clear();
    }

    fn duration_as_nanos(d: &Duration) -> u64 {
        d.as_secs() * 1_000_000_000 + (d.subsec_nanos() as u64)
    }
}
