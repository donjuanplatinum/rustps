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




// //读取PID
// fn read_proc_pid(pid: i32, filename: &str) -> io::Result<String> {

//     let file_path = format!("/proc/{}/{}", pid, filename);
//     fs::read_to_string(file_path)
// }

// //获取处理器频率并经算利用率
// fn get_cpu_frequency() -> io::Result<f64> {
//     let cpu_frequency = fs::read_to_string("/proc/cpuinfo")?
//         .lines()
//         .find(|line| line.contains("cpu MHz"))
//         .and_then(|line| line.split(':').last())
//         .and_then(|freq_str| freq_str.trim().parse::<f64>().ok())
//         .map(|freq| freq * 1e6 / 100.0); // Converting MHz to Hz, and then to percentage
//     cpu_frequency.ok_or(io::Error::new(io::ErrorKind::Other, "Failed to read CPU MHz"))
// }


// //获取进程信息
// fn get_process_info(pid: i32) -> Result<(String, f64, f64), io::Error> {
//     if let Ok(cmdline) = read_proc_pid(pid, "cmdline") {
//         let cmd = cmdline.replace('\0', " ");
//         if let Ok(status) = read_proc_pid(pid, "status") {
//             if let Some(rss_line) = status.lines().find(|line| line.starts_with("VmRSS")) {
//                 let rss: Vec<&str> = rss_line.split_whitespace().collect();

//                 if let Some(mem_kb) = rss.get(1) {
//                     if let Ok(stat) = read_proc_pid(pid, "stat") {
//                         let stat_split: Vec<&str> = stat.split_whitespace().collect();
//                         if let (Some(utime), Some(stime)) = (stat_split.get(13), stat_split.get(14)) {
//                             let total_time: f64 = utime.parse::<f64>().unwrap() + stime.parse::<f64>().unwrap();
//                             return Ok((cmd, mem_kb.parse::<f64>().unwrap(), total_time));
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     Err(io::Error::new(io::ErrorKind::Other, "Failed to read process info"))
// }
// //进程查询
// fn search_processes(clock_ticks: f64) -> io::Result<Vec<(i32, String, f64, f64)>> {
//     let mut processes: Vec<(i32, String, f64, f64)> = Vec::new();

//     for entry in fs::read_dir("/proc")? {
//         if let Ok(entry) = entry {
//             if let Ok(file_name) = entry.file_name().into_string() {
//                 if let Ok(pid) = file_name.trim().parse::<i32>() {
//                     if let Ok((cmd, mem_kb, total_time)) = get_process_info(pid) {
//                         let cpu_usage = 100.0 * (total_time / clock_ticks);
//                         processes.push((pid, cmd, mem_kb, cpu_usage));
//                     }
//                 }
//             }
//         }
//     }
//     Ok(processes)
// }
// //输出模块
// fn print_processes(processes: Vec<(i32, String, f64, f64)>) {
//     println!("{:<8} {:<8} {:<8} {:<12}", "PID", "RSS", "CPU%", "CMD");
//     for process in processes {
//         println!("{:<8} {:<8} {:<8.2} {:<12}", process.0, process.2, process.3, process.1);
//     }
// }
