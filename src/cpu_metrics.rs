use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuStat {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
}

impl CpuStat {
    pub fn read_from_file() -> Option<CpuStat> {
        let contents = fs::read_to_string("/proc/stat").ok()?;
        let cpu_line = contents.lines().find(|line| line.starts_with("cpu "))?;
        let mut parts = cpu_line.split_whitespace();
        if parts.clone().count() < 9 {
            return None;
        }
        Some(CpuStat {
            user: parts.nth(1)?.parse().ok()?,
            nice: parts.next()?.parse().ok()?,
            system: parts.next()?.parse().ok()?,
            idle: parts.next()?.parse().ok()?,
            iowait: parts.next()?.parse().ok()?,
            irq: parts.next()?.parse().ok()?,
            softirq: parts.next()?.parse().ok()?,
            steal: parts.next()?.parse().ok()?,
        })
    }

    pub fn total(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + self.iowait + self.irq + self.softirq + self.steal
    }

    pub fn busy(&self) -> u64 {
        self.total() - self.idle - self.iowait
    }

    pub fn calculate_cpu_load(prev: &CpuStat, curr: &CpuStat) -> f64 {
        let prev_total = prev.total();
        let curr_total = curr.total();
        let prev_busy = prev.busy();
        let curr_busy = curr.busy();

        let total_diff = curr_total - prev_total;
        let busy_diff = curr_busy - prev_busy;

        if total_diff == 0 {
            0.0
        } else {
            (busy_diff as f64 / total_diff as f64) * 100.0
        }
    }
}