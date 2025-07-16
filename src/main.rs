use std::{time::Duration, env};

use sysinfo::{Disks, Networks, System};
use mongodb::{bson::doc, Client, error::Error, Collection};
use minreq::Response;
use chrono;

mod mongo;

#[tokio::main]
async fn main() {
    println!("Connecting to the MongoDB");
    let client = match create_client().await {
        Ok(resp) => resp,
        Err(err) => {
            println!("Connection error: {}", err);
            return;
        }
    };
    let db = client.database("system");
    let ip = get_ip();
    let collection = db.collection::<mongo::System>(&ip);
    println!("Connected to the MongoDB");

    let category: String;
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        category = "Unknown".to_string();
    } else {
        args[0] = String::new();
        category = args.join(" ").trim().to_string();
    }
    drop(args);

    let mut sys = System::new_all();
    let mut networks = Networks::new_with_refreshed_list();

    loop {
        update_stats(&collection, &mut sys, &mut networks, &category).await;
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

async fn update_stats(collection: &Collection<mongo::System>, sys: &mut System, networks: &mut Networks, category: &String) {
    let time = chrono::Utc::now().timestamp() * 1000;

    _ = collection.delete_many(doc!{"date": { "$lte": time - 86400000 }}).await; // 1000 * 60 * 60 * 24

    let mut networks_load: Vec<mongo::NetworkLoad> = Vec::new();
    for (interface_name, network) in networks.list() {
        let network_load = mongo::NetworkLoad {
            name: interface_name.clone(),
            inbount: network.received(),
            outbount: network.transmitted(),
        };
        networks_load.push(network_load);
    }

    networks.refresh(false);

    sys.refresh_cpu_usage();
    sys.refresh_cpu_frequency();
    sys.refresh_memory();

    let mut cpus_load: Vec<mongo::CpuLoad> = Vec::new();
    for cpu in sys.cpus() {
        let cpu_load = mongo::CpuLoad {
            load: format!("{}%", cpu.cpu_usage() as u32),
            frequency: cpu.frequency(),
        };
        cpus_load.push(cpu_load);
    }

    let disks = Disks::new_with_refreshed_list();
    let mut disks_load: Vec<mongo::DiskLoad> = Vec::new();
    for disk in disks.list() {
        if disk.name() == "overlay" {
            continue;
        }

        let disk_total = disk.total_space();

        if disk_total == 0 {
            continue;
        }

        let disk_name = format!("{:?} ({:?})", disk.name(), disk.kind());
        let disk_usage = disk_total - disk.available_space();
        let disk_size = 100 * disk_usage / disk_total;
        let disk_load = mongo::DiskLoad {
            name: disk_name,
            usage: disk_usage,
            load: format!("{}%", disk_size),
        };
        disks_load.push(disk_load);
    }

    let memory_data = mongo::MemLoad {
        total: sys.total_memory(),
        load: sys.total_memory() - sys.free_memory(),
        cache: sys.available_memory() - sys.free_memory(),
        used: sys.used_memory(),
        swap_total: sys.total_swap(),
        swap_free: sys.free_swap(),
        swap_used: sys.used_swap(),
    };

    let system_data = mongo::System {
        category: category.to_string(),
        name: System::host_name().unwrap_or("hostname".to_string()),
        uptime: System::uptime(),
        date: time,
        processes: sys.processes().len(),
        cpus: cpus_load,
        memory: memory_data,
        disks: disks_load,
        network: networks_load,
    };

    _ = collection.insert_one(system_data).await;
}

fn get_ip() -> String {
    match minreq::get("http://myexternalip.com/raw").send() {
        Ok(res) => {
            let ip = res_to_ip(res);
            if ip.split('.').count() == 4 {
                return ip;
            }
        },
        Err(_) => {},
    }
    match minreq::get("http://checkip.amazonaws.com/").send() {
        Ok(res) => {
            let ip = res_to_ip(res);
            if ip.split('.').count() == 4 {
                return ip;
            }
        },
        Err(_) => {},
    }
    match minreq::get("http://whatismyip.akamai.com/").send() {
        Ok(res) => {
            let ip = res_to_ip(res);
            if ip.split('.').count() == 4 {
                return ip;
            }
        },
        Err(_) => {},
    }
    match minreq::get("http://icanhazip.com/").send() {
        Ok(res) => {
            let ip = res_to_ip(res);
            if ip.split('.').count() == 4 {
                return ip;
            }
        },
        Err(_) => {},
    }
    match minreq::get("http://ifconfig.io/ip").send() {
        Ok(res) => {
            let ip = res_to_ip(res);
            if ip.split('.').count() == 4 {
                return ip;
            }
        },
        Err(_) => {},
    }
    match minreq::get("http://ipecho.net/plain").send() {
        Ok(res) => {
            let ip = res_to_ip(res);
            if ip.split('.').count() == 4 {
                return ip;
            }
        },
        Err(_) => {},
    }

    panic!("Cannot get ip");

    fn res_to_ip(res: Response) -> String {
        let body = res.as_str().unwrap();
        body.replace('\n', "").trim().to_string()
    }
}

async fn create_client() -> Result<Client, Error>{
    let crypt_mongo = include_crypt::include_crypt!("src/mongodb.txt");
    
    let mongo = match crypt_mongo.decrypt_str() {
        Ok(resp) => resp,
        Err(err) => return Err(Error::custom(
            format!("{}", err)
        ))
    };

    let client = Client::with_uri_str(mongo).await?;

    Ok(client)
}