use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::{thread, time};

pub fn average_cpu_usage() {
    let delay = time::Duration::from_millis(2000);
    let mut last: Stat = Stat::new();

    loop {
        let stats = get_stats(read_stat_file());
        let curr = stats[0];
        if !last.values.is_empty() {
            let usage = calculate_usage(&curr, &last);
            println!("{usage:02.0}");
        }
        last = curr;
        thread::sleep(delay);
    }
}

pub fn calculate_usage(curr: &Stat, last: &Stat) -> f32 {
    let usage = (curr.work_time() - last.work_time()) / (curr.total_time() - last.total_time()) * 100.0;
    log::debug!("({} - {}) / ({} - {}) * 100 = {}", curr.work_time(), last.work_time(), curr.total_time(), last.total_time(), usage);
    usage
}

#[derive(Debug, Copy, Clone)]
pub struct Stat {
    values: [f32; 10]
}
impl Stat {
    pub fn new() -> Self{
        Self { values: [0f32; 10]}
    }
    pub fn total_time(&self) -> f32 {
        self.values
            .iter()
            .sum::<f32>()
    }
    pub fn work_time(&self) -> f32 {
        self.values.iter().sum::<f32>() - self.values[StatIndex::Idle as usize]
    }
}
impl Default for Stat{
    fn default() -> Self {
        Stat::new()
    }
}

#[derive(Debug)]
pub enum StatIndex {
    User = 0,       // time spent in user mode
    Nice = 1,       // time spent in user mode with low priority
    System = 2,     // time spent in system mode
    Idle = 3,       // time spent in idle task (USER_HZ * /proc/uptime[1])
    IoWait = 4,     // time spent waiting for I/O to complete
    Irq = 5,        // time servicing interrupts
    SoftIrq = 6,    // time servicing softirqs
    Steal = 7,      // time spent in virtualized enviroments
    Guest = 8,      // time spent running virtual cpu for guest OS
    GuestNice = 9,  // time spent running niced guest
}

const STAT_PATH: &str = "/proc/stat";

pub fn read_stat_file() -> File {
    match File::open(STAT_PATH) {
        Ok(file) => file,
        Err(e) => panic!("Error reading file: {e}")
    }
}

pub fn get_stats(file: File) -> Vec<Stat> {
    let reader = BufReader::new(file);

    let parsed = reader.lines()
        .map(|x| x.unwrap())
        .filter(|s| s.contains("cpu"))
        .map(|x| {
            x.split(' ')
            .flat_map(|i| i.parse::<f32>().ok())
            .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>();

    let mut stats:Vec<Stat> = Vec::new();
    for values in parsed {
        let mut arr: [f32; 10] = [0.0;10];
        for (i, v) in values.iter().enumerate() {
            arr[i] = *v;
        }
        stats.push(Stat { values: arr });
    } 
    stats
}
