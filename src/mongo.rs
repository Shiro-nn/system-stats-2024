use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct System {
    pub category: String,
    pub name: String,
    pub uptime: u64,
    pub date: i64,
    pub processes: usize,
    pub cpus: Vec<CpuLoad>,
    pub memory: MemLoad,
    pub disks: Vec<DiskLoad>,
    pub network: Vec<NetworkLoad>,
}

#[derive(Serialize, Deserialize)]
pub struct CpuLoad {
    pub load: String,
    pub frequency: u64,
}

#[derive(Serialize, Deserialize)]
pub struct MemLoad {
    pub total: u64,
    pub load: u64,
    pub cache: u64,
    pub used: u64,
    pub swap_total: u64,
    pub swap_free: u64,
    pub swap_used: u64,
}

#[derive(Serialize, Deserialize)]
pub struct DiskLoad {
    pub load: String,
    pub usage: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkLoad {
    pub name: String,
    pub inbount: u64,
    pub outbount: u64,
}