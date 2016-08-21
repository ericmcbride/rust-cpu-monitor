mod system;

#[macro_use]
extern crate clap;
extern crate libc;
extern crate psutil;

use std::mem;
use clap::App;

use std::ptr;
use std::mem::size_of_val;
use std::mem::size_of;
use std::io;


pub type PID = libc::pid_t;


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let system = system::system::SystemInformation::new();
    let processes = get_all_processes_darwin();
    println!("system info is {:?}", system);
    println!("processes are {:?}", processes);
}

pub fn get_all_processes_darwin() -> i32 {
    // Management Information Base (the name, namelen)
    // Apple Explanation Below in iphone docs:
    // https://developer.apple.com/library/ios/documentation/System/Conceptual/ManPages_iPhoneOS/man3/sysctl.3.html
    // How HTOP Does it:
    // https://github.com/hishamhm/htop/blob/master/darwin/DarwinProcess.c
    let mut mib = vec![
        libc::CTL_KERN as libc::c_int,
        libc::KERN_PROC as libc::c_int,
        libc::KERN_PROC_ALL as libc::c_int,
        0 as libc::c_int
    ];
    let mut size: libc::size_t = 0;

    // SYSCTL ARGUMENTS:
    // FIRST ARG:
    //      NAME: Points to an array of integers.  Each of the integer values identifies a sysctl
    //      item either a dir or a node file.
    // SECOND ARG:
    //      NLEN: States how many intger numbers are listen in the array name: to reach a
    //      particular entry you need to specify the path through the subdirectories so you need to
    //      tell how long is such path
    // THIRD ARG:
    //      OLDVAL: Is a pointer to a data buffer where the old value of the sysctl item must be
    //      stored. If NULL, the system wont return values to the user space
    // FOURTH ARG:
    //      NEWVAL: Points to a data buffer hosting replacement dataa.  The kernel will read this
    //      buffer to change the sysctl entry being acted upon.  If null the kernel value is not
    //      changed
    // FIFTH ARG:
    //      NEWLEN: The length of the newval
    // REFERENCE: http://www.linux.it/~rubini/docs/sysctl/
    unsafe {
        let mut err = libc::sysctl(mib.as_mut_ptr(), mib.len() as libc::c_uint, ptr::null_mut(), &mut size, ptr::null_mut(), 0 as libc::size_t);

        if err !=0 {
            println!("Err: {}", io::Error::last_os_error());
        } else {
            println!("Matches");
        }

        // we have to allocate memory on the heap to buffer the process list
        let process_list = libc::malloc(size);
        //let proccess_list = kinfo_proc { proc_list: libc::malloc(size) };
        println!("proc_list {:?}", process_list);

        // call sysctl once again, since now we know the size of the processes
        let mut err = libc::sysctl(mib.as_mut_ptr(), mib.len() as libc::c_uint, process_list, &mut size, ptr::null_mut(), 0 as libc::size_t);

        // divide the size by the size of the proc list
        let mut proc_count: libc::c_int = size as i32 / std::mem::size_of_val(&process_list) as i32;
        println!("proc_count is {:?}", proc_count);

        //TODO: Find out how to iterate through each item in the process list.
        for index in 0..proc_count {
            let process_list_index = process_list.offset(index as isize);
            println!("INDEX IN PROC LIST {:?}", process_list_index);
        }
        return err
    }
}


pub fn get_pid() -> PID {
    unsafe { libc::getpid() }
}

pub fn get_parent_pid() -> PID {
    unsafe { libc::getppid() }
}
