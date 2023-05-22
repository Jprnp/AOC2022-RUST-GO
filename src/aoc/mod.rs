pub mod aoc1;
pub mod aoc2;
pub mod aoc3;

#[cfg(windows)]
const LINE_ENDING : &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING : &'static str = "\n";
