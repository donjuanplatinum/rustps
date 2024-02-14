use std::fmt::Debug;
use std::fs;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
#[derive(Debug)]
///进程
pub struct ProcessStat{
    ///pid
    pid: Option<i32>,
    ///进程占用的真实内存大小
    rss: Option<String>,
    ///调用命令的完整信息
    cmd: Option<String>,
    ///进程状态
    state: Option<String>,
    ///父进程PID
    ppid: Option<String>,
    ///进程及其子进程被调度进内核态的时间,以jiffy为单位 1jiffy=1/100s
    cstime: Option<String>,
    ///进程及其子进程被调度进用户态的时间
    cutime: Option<String>,
    ///虚拟内存大小
    vsize: Option<String>,
    ///cpu利用率
    cpu_use: Option<f64>,
    ///进程进入用户态的时间(单位jiffy)
    utime: Option<String>,
    ///进程进入内核态的时间
    stime: Option<String>,
    ///内存占用峰值
    vmpeak: Option<String>,
    ///实际用户
    ruid: Option<String>,
    ///实际组
    rgid: Option<String>,
}
use std::fmt;

impl fmt::Display for ProcessStat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PID: {:<8} PPID: {:<8} RSS: {:<8} VMPEAK: {:<8} STATE: {:<8} CPU%: {:.2} CMD: {}",
               self.pid.unwrap_or(-1),
               self.ppid.as_deref().unwrap_or("None"),
               self.rss.as_deref().unwrap_or("None"),
               self.vmpeak.as_deref().unwrap_or("None"),
               self.state.as_deref().unwrap_or("None"),
               self.cpu_use.unwrap_or(-1.0),
               self.cmd.as_deref().unwrap_or("None")
        )
    }
}

impl Default for ProcessStat{
    fn default() ->ProcessStat  {
	ProcessStat { pid: None, rss: None, cmd: None, state: None, ppid: None, cstime: None, vsize: None , cpu_use: None, cutime: None, stime: None, utime: None, vmpeak: None, ruid: None,rgid: None}
    }
}
impl ProcessStat{
    ///创建新的进程信息表
    pub fn new() -> ProcessStat {
	ProcessStat::default()
    }
    ///根据PID获取表
pub fn new_from_pid(pid: i32, cpu_frequency: f64) -> Result<ProcessStat, io::Error> {
    // 从/proc/pid/status获取进程信息
    // 通过定位/proc/{pid}/status中的行数,并通过trim_start_matches删除前缀,trim删除空格等
    let status_str = fs::read_to_string(format!("/proc/{}/status", pid))?;
    let stat_str = fs::read_to_string(format!("/proc/{}/stat", pid))?;
    // 从/proc/pid/cmdline获取
    let comm_str = fs::read_to_string(format!("/proc/{}/cmdline", pid))?;

    let utime = stat_str.split_whitespace().nth(14).map(|line| line.parse::<f64>().unwrap_or(0.0));
    let stime = stat_str.split_whitespace().nth(15).map(|line| line.parse::<f64>().unwrap_or(0.0));
    let u_time = utime.unwrap_or(0.0);
    let s_time = stime.unwrap_or(0.0);
    let cpu_usage = 100.0 * (s_time / cpu_frequency);  // 计算 CPU 利用率

    let rss = if let Some(line) = status_str.lines().nth(22) {
        if line.contains("VmRSS") {
            if line.trim_start_matches("VmRSS:").trim() == "sigign" {
                None
            } else {
                Some(line.trim_start_matches("VmRSS:").trim().to_string())
            }
        } else {
            None
        }
    } else {
        None
    };

    let vmpeak = if let Some(line) = status_str.lines().nth(17) {
        if line.contains("VmPeak") {
            if line.trim_start_matches("VmPeak:").trim() == "fffffffff" {
                None
            } else {
                Some(line.trim_start_matches("VmPeak:").trim().to_string())
            }
        } else {
            None
        }
    } else {
        None
    };
    let status_str_lines = status_str.lines();
    Ok(ProcessStat {
        pid: Some(pid),
        state: status_str.lines().nth(2).map(|line| line.trim_start_matches("State:").trim().to_string()),
        cmd: Some(comm_str),
        rss,
        ppid: status_str.lines().nth(6).map(|line| line.trim_start_matches("PPid:").trim().to_string()),
        cstime: stat_str.split_whitespace().nth(17).map(|line| line.to_string()),
        vsize: status_str.lines().nth(18).map(|line| line.trim_start_matches("VmSize:").trim().to_string()),
        cpu_use: Some(cpu_usage),
        utime: stat_str.split_whitespace().nth(13).map(|line| line.to_string()),
        stime: stat_str.split_whitespace().nth(14).map(|line| line.to_string()),
        cutime: stat_str.split_whitespace().nth(16).map(|line| line.to_string()),
        vmpeak,
	ruid: status_str.lines().nth(8).map(|line| line.trim_start_matches("Uid:").trim().to_string()),
	rgid: status_str.lines().nth(9).map(|line| line.trim_start_matches("Gid:").trim().to_string())
    })
}
}

///获取处理器频率
pub fn get_cpu_frequency() -> io::Result<f64> {
    let cpu_frequency = fs::read_to_string("/proc/cpuinfo")?
        .lines()
        .find(|line| line.contains("cpu MHz"))
        .and_then(|line| line.split(':').last())
        .and_then(|freq_str| freq_str.trim().parse::<f64>().ok())
        .map(|freq| freq * 1e6 / 100.0); // Converting MHz to Hz, and then to percentage
    cpu_frequency.ok_or(io::Error::new(io::ErrorKind::Other, "Failed to read CPU MHz"))
}

///获取进程信息
pub fn load_process_stats() -> Result<Vec<ProcessStat>, io::Error> {
    let mut process_stats: Vec<ProcessStat> = Vec::new();
    let entries = fs::read_dir("/proc")?;  // 读取/proc目录下的文件
    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(file_name) = entry.file_name().into_string() {
                if let Ok(pid) = file_name.parse::<i32>() {  // 尝试将文件名解析为PID
                    if let Ok(process_stat) = ProcessStat::new_from_pid(pid,get_cpu_frequency()?) {
                        process_stats.push(process_stat);
                    }
                }
            }
        }
    }
    Ok(process_stats)
}
/// 格式化输出进程信息表
pub fn format_process_stats(process_stats: &[ProcessStat]) {
    println!("{:<8} {:<8} {:<8} {:<8} {:<8} {:<8} {:<8}",
             "PID", "PPID", "RSS", "VMPEAK", "STATE", "CPU%", "CMD");
    for process_stat in process_stats {
        println!("{}", process_stat);
    }
}
///打印一个进程信息
pub fn format_one_process(pid: i32) -> Result<(), io::Error> {
    if let Ok(process_stat) = ProcessStat::new_from_pid(pid, get_cpu_frequency()?) {
        println!("{}", process_stat);
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to read process stats"))
    }
}

pub fn human_readable_size(size: usize) -> String {
    if size >= 1024 * 1024 * 1024 {
        format!("{:.2} G", size as f64 / (1024 * 1024 * 1024) as f64)
    } else if size >= 1024 * 1024 {
        format!("{:.2} MB", size as f64 / (1024 * 1024) as f64)
    } else if size >= 1024 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else {
        format!("{} B", size)
    }
}

use std::env;
///获取命令行选项
pub fn get_command_line_options() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        // 显示帮助信息
        println!("Displaying help...");
        // Add your help instructions here...
    }

    if args.contains(&"-m".to_string()) || args.contains(&"--memory".to_string()) {
        // 处理 -m 选项
        println!("Showing memory information...");
    }

    if args.contains(&"-n".to_string()) || args.contains(&"--name".to_string()) {
        // 处理 -n 选项
        println!("Showing process name...");
    }

    if args.contains(&"-d".to_string()) || args.contains(&"--details".to_string()) {
        // 处理 -d 选项
        println!("Showing detailed information...");
    }
}

