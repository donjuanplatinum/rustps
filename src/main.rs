use std::env;
use std::fs;
use std::io;
use rustps::ProcessStat;
use rustps::get_cpu_frequency;
use rustps::format_process_stats;
use rustps::load_process_stats;

fn main() {
    use clap::Command;

    let cmd = Command::new("Rustps")
	.version("0.5.0")
	.author("DonjuanPlatinum <donjuan@lecturify.net>")
	.about("rust procps")
	.get_matches();
    if let Ok(process_stats) = load_process_stats() {
        format_process_stats(&process_stats);
    } else {
        eprintln!("Failed to load process stats.");
    }
}



