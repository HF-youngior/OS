use riscv::register::time;
use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;

pub fn get_time() -> usize {
    time::read()
}

const TICKS_PER_SEC: usize = 100;

pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

const MSEC_PER_SEC: usize = 1000;

pub fn get_time_ms() -> usize {
    get_time() / (CLOCK_FREQ / MSEC_PER_SEC)
}
