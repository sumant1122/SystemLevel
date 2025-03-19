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
            print_cpu_usage_bar(&format!("CPU {}", i), cpu.cpu_usage());
        }

        print!("\x1B[{}A", sys.cpus().len()+1);

        thread::sleep(Duration::from_secs(1));
    }
}

fn print_cpu_usage_bar(label: &String, usage: f32) {

    let bar_width = 20;

    let filled = (usage/100.0 * bar_width as f32).round() as usize;

    let bar = format!(
        "[{}{}]",
        "=".repeat(filled),
        " ".repeat(bar_width - filled)
    );

    println!("{}: {} {:.2}%", label, bar, usage)
}
