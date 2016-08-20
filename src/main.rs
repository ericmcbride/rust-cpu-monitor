mod system;

#[macro_use]
extern crate clap;
extern crate libc;
extern crate psutil;

use clap::App;

use std::ptr;
use std::mem::size_of_val;
use std::io::Error;

pub type PID = libc::pid_t;


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let system = system::system::SystemInformation::new();
    let processes = get_all_processes_darwin();
    println!("system info is {:?}", system);
    println!("processes are {:?}", processes);
}

pub fn get_all_processes_darwin() {
    // Management Information Base (the name, namelen)
    // Apple Explanation Below in iphone docs:
    // https://developer.apple.com/library/ios/documentation/System/Conceptual/ManPages_iPhoneOS/man3/sysctl.3.html
    // How HTOP Does it:
    // https://github.com/hishamhm/htop/blob/master/darwin/DarwinProcess.c
    let mut mib: [libc::c_int; 2] = [libc::CTL_KERN, libc::KERN_MAXPROC];
    let mut value: libc::c_int = 16;
    let mut size = size_of_val(&value);
    let pointer: *mut libc::c_void = &mut value as *mut _ as *mut libc::c_void;

    unsafe {
        let result = libc::sysctl(mib.as_mut_ptr() as *mut libc::c_int, mib.len() as libc::c_uint, pointer, &mut size as *mut usize, ptr::null_mut(), 0);
    }
}

pub fn get_pid() -> PID {
    unsafe { libc::getpid() }
}

pub fn get_parent_pid() -> PID {
    unsafe { libc::getppid() }
}
