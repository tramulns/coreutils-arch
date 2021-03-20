use clap::App;
use platform_info::*;

fn main() {
    App::new("arch").get_matches();
    let uts = PlatformInfo::new().unwrap();
    println!("{}", uts.machine().trim());
    std::process::exit(0)
}
