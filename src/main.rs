mod system;

use system::system::{
    get_cpus, get_cpu_speed, get_os_type, get_os_release, get_disk_info_free, get_disk_info_total, get_hostname
};
#[derive(Debug)]
struct SystemInformation {
    num_cpus: usize,
    processor_speed: u64,
    os_type: String,
    os_release: String,
    disk_info_free: u64,
    disk_info_total: u64,
    hostname: String,
}

impl SystemInformation {
    fn new() -> SystemInformation {
        SystemInformation {
            num_cpus: get_cpus(),
            processor_speed: get_cpu_speed(),
            os_type: get_os_type(),
            os_release: get_os_release(),
            disk_info_free: get_disk_info_free(),
            disk_info_total: get_disk_info_total(),
            hostname: get_hostname(),
        }
    }
}


fn main() {
    let system = SystemInformation::new();
    println!("system info is {:?}", system);
}
