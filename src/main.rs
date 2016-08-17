mod system;

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
            num_cpus: system::system::get_cpus(),
            processor_speed: system::system::get_cpu_speed(),
            os_type: system::system::get_os_type(),
            os_release: system::system::get_os_release(),
            disk_info_free: system::system::get_disk_info_free(),
            disk_info_total: system::system::get_disk_info_total(),
            hostname: system::system::get_hostname(),
        }
    }
}


fn main() {
    let system = SystemInformation::new();
    println!("system info is {:?}", system);
}
