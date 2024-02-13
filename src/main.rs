use std::env;
use std::fs;
use std::io;
use rustps::ProcessStat;
use rustps::get_cpu_frequency;
use rustps::format_process_stats;
use rustps::load_process_stats;
fn main() {
    if let Ok(process_stats) = load_process_stats() {
        format_process_stats(&process_stats);
    } else {
        eprintln!("Failed to load process stats.");
    }
}


