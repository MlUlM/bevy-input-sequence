use std::time::Duration;

use bevy::prelude::{Resource, TimerMode};
use bevy::time::{Time, Timer};

#[derive(Resource, Debug)]
pub enum Timeout {
    None,
    Frames { limit: u32, current: u32 },
    Time(Timer),
}



impl Timeout {
    #[inline(always)]
    pub fn from_duration(timeout: Duration) -> Timeout {
        Self::Time(Timer::new(timeout, TimerMode::Once))
    }


    #[inline(always)]
    pub const fn from_frame_count(limit_frame_count: u32) -> Timeout {
        Self::Frames { limit: limit_frame_count, current: 0 }
    }


    #[inline(always)]
    pub(crate) fn timeout(&mut self, time: &Time) -> bool {
        match self {
            Self::None => false,

            Self::Time(timer) => timer.tick(time.delta()).just_finished(),

            Self::Frames {
                limit,
                current
            } => {
                *current += 1;
                limit <= current
            }
        }
    }
}


impl Clone for Timeout {
    fn clone(&self) -> Self {
        match self {
            Timeout::None => Timeout::None,
            Timeout::Frames { limit, current: _ } => Timeout::from_frame_count(*limit),
            Timeout::Time(timer) => Timeout::from_duration(timer.duration())
        }
    }
}


impl Default for Timeout {
    #[inline(always)]
    fn default() -> Self {
        Self::from_duration(Duration::from_secs(1))
    }
}