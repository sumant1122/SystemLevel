use std::thread;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    loop {
        sys.refresh_cpu();

        let global_cpu_usage = sys.global_cpu_info().cpu_usage();

        println!("Global CPU usage: {:.2}%", global_cpu_usage);

        for (i, cpu) in sys.cpus().iter().enumerate() {
            println!("CPU {} usage: {:.2}%", i, cpu.cpu_usage());
        }

        thread::sleep(Duration::from_secs(1));
    }
}
