use std::thread;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    //Create new System object
    let mut sys = System::new_all();

    //Refresh all system information
    sys.refresh_all();

    loop {

        //Refresh CPU information
        sys.refresh_cpu();

        //clear the terminal each time
        println!("\x1B[2J\x1B[H");

        // CPU usage
        let global_cpu_usage = sys.global_cpu_info().cpu_usage();

        print_cpu_usage_bar("Global CPU usage", global_cpu_usage);

        for (i, cpu) in sys.cpus().iter().enumerate() {
            print_cpu_usage_bar(&format!("CPU {}", i), cpu.cpu_usage());
        }

        //Memory usage
        println!("\nMemory Usage");
        let total_memory = sys.total_memory();
        let used_memory= sys.used_memory();
        let memory_usage_percent = (used_memory as f64/total_memory as f64)*100.0;

        println!("Used Memory: {:.2}% ({}/{}) MB)",
                 memory_usage_percent,
                 used_memory / 1024 / 1024,
                 total_memory / 1024 /1024 );

        //Wait for second before the next update
        thread::sleep(Duration::from_secs(1));
    }
}


//Helper function to print CPU usage bars
fn print_cpu_usage_bar(label: &str, usage: f32) {

    //width of bar
    let bar_width = 20;

    //Calculate the number of filled blocks
    let filled = (usage/100.0 * bar_width as f32).round() as usize;

    // create the bar string
    let bar = format!(
        "[{}{}]",
        "=".repeat(filled),
        " ".repeat(bar_width - filled)
    );
    //print label, bar and percentage usage
    println!("{}: {} {:.2}%", label, bar, usage)
}
