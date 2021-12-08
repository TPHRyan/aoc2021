pub mod pool;

pub use pool::LanternfishPool;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

const INITIAL_REPRODUCTION_TICKS: u32 = 9;
const RESET_REPRODUCTION_TICKS: u32 = 7;

enum TickAction {
    DoNothing,
    Reproduce,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Lanternfish {
    pub quantity: usize,
    t_last_reproduction: u32,
    t_next_reproduction: u32,
}

impl Lanternfish {
    pub fn new(quantity: usize) -> Lanternfish {
        Lanternfish {
            quantity,
            t_last_reproduction: 0,
            t_next_reproduction: INITIAL_REPRODUCTION_TICKS,
        }
    }

    pub fn from(n: u32) -> Lanternfish {
        Lanternfish {
            quantity: 1,
            t_last_reproduction: 0,
            t_next_reproduction: n,
        }
    }

    pub fn combine(&mut self, other: Self) {
        self.quantity += other.quantity;
    }

    pub fn can_combine(&self, other: &Self) -> bool {
        self.t_next_reproduction == other.t_next_reproduction
    }

    pub fn should_reproduce_on(&self, tick: u32) -> bool {
        tick >= self.t_next_reproduction
    }

    pub fn reset(&mut self, current_tick: u32) {
        self.t_last_reproduction = current_tick;
        self.t_next_reproduction = current_tick + INITIAL_REPRODUCTION_TICKS;
    }

    pub fn tick(&mut self, current_tick: u32) -> Option<Lanternfish> {
        match self.check_tick_action(current_tick) {
            TickAction::Reproduce => Some(Lanternfish::new(self.quantity)),
            TickAction::DoNothing => None,
        }
    }

    fn check_tick_action(&mut self, current_tick: u32) -> TickAction {
        if self.should_reproduce_on(current_tick) {
            self.t_last_reproduction = self.t_next_reproduction;
            self.t_next_reproduction += RESET_REPRODUCTION_TICKS;
            return TickAction::Reproduce;
        }
        return TickAction::DoNothing;
    }
}

impl Ord for Lanternfish {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t_next_reproduction.cmp(&other.t_next_reproduction)
    }

    fn max(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Greater => self,
            _ => other,
        }
    }

    fn min(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Greater => other,
            _ => self,
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        match self.cmp(&min) {
            Ordering::Less => min,
            _ => match self.cmp(&max) {
                Ordering::Greater => max,
                _ => self,
            },
        }
    }
}

impl PartialOrd<Self> for Lanternfish {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Display for Lanternfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.t_next_reproduction)
    }
}
