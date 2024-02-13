use std::env;
use std::fs;
use std::io;
use algori::sort::quicksort; // Import quicksort from algori library

fn read_proc_pid(pid: i32, filename: &str) -> io::Result<String> {
    let file_path = format!("/proc/{}/{}", pid, filename);
    fs::read_to_string(file_path)
}

fn get_clock_ticks() -> io::Result<f64> {
    let clock_ticks = fs::read_to_string("/proc/cpuinfo")?
        .lines()
        .find(|line| line.contains("cpu MHz"))
        .and_then(|line| line.split(':').last())
        .and_then(|freq_str| freq_str.trim().parse::<f64>().ok())
        .map(|freq| freq * 1e6 / 100.0); // Converting MHz to Hz, and then to percentage
    clock_ticks.ok_or(io::Error::new(io::ErrorKind::Other, "Failed to read CPU MHz"))
}

fn search_and_print_process_info(sort_by_mem: bool, clock_ticks: f64) -> io::Result<()> {
    println!("{:<8} {:<12} {:<8} {}", "PID", "CMD", "RSS (kB)", "CPU (%)");
    let mut processes: Vec<(i32, String, f64, f64)> = Vec::new();

    for entry in fs::read_dir("/proc")? {
        if let Ok(entry) = entry {
            if let Ok(file_name) = entry.file_name().into_string() {
                if let Ok(pid) = file_name.trim().parse::<i32>() {
                    if let Ok(cmdline) = read_proc_pid(pid, "cmdline") {
                        let cmd = cmdline.replace('\0', " ");
                        if let Ok(status) = read_proc_pid(pid, "status") {
                            if let Some(rss_line) = status.lines().find(|line| line.starts_with("VmRSS")) {
                                let rss: Vec<&str> = rss_line.split_whitespace().collect();
                                if let Some(mem_kb) = rss.get(1) {
                                    if let Ok(stat) = read_proc_pid(pid, "stat") {
                                        let stat_split: Vec<&str> = stat.split_whitespace().collect();
                                        if let (Some(utime), Some(stime)) = (stat_split.get(13), stat_split.get(14)) {
                                            let total_time: f64 = utime.parse::<f64>().unwrap() + stime.parse::<f64>().unwrap();
                                            let cpu_usage = 100.0 * (total_time / clock_ticks);
                                            processes.push((pid, cmd, mem_kb.parse::<f64>().unwrap(), cpu_usage));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if sort_by_mem {
        quicksort(&mut processes); // Sort the vector based on memory
    }

    for process in processes {
        println!("{:<8} {:<12} {} kB {:.2}%", process.0, process.1, process.2, process.3);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "-m" {
        if let Ok(clock_ticks) = get_clock_ticks() {
            if let Err(e) = search_and_print_process_info(true, clock_ticks) {
                eprintln!("Error: {}", e);
            }
        }
    } else {
        if let Ok(clock_ticks) = get_clock_ticks() {
            if let Err(e) = search_and_print_process_info(false, clock_ticks) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
