mod system;

#[macro_use]
extern crate clap;
use clap::App;


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let system = system::system::SystemInformation::new();
    println!("system info is {:?}", system);

}
