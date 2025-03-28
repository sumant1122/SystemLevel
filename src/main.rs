use eframe::egui;
use std::time::Duration;
use sysinfo::{Disks, System};

struct CpuApp {
    system: System,
    usage: f32,
    usages: Vec<f32>,
    total_mem_usage: u64,
    used_mem_usage: u64,
}

impl Default for CpuApp {
    fn default() -> Self {
        let mut system = System::new_all();
        system.refresh_cpu();
        let usage = system.global_cpu_info().cpu_usage();
        let usages = system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
        let total_mem_usage = system.total_memory();
        let used_mem_usage = system.used_memory();

        CpuApp {
            system,
            usage,
            usages,
            total_mem_usage,
            used_mem_usage,
        }
    }
}

impl eframe::App for CpuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.system.refresh_cpu();
        self.usage = self.system.global_cpu_info().cpu_usage();
        self.usages = self
            .system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();
        self.system.refresh_memory();
        let tm = self.total_mem_usage / 1024 / 1024;
        let um = self.used_mem_usage / 1024 / 1024;

        egui::TopBottomPanel::top("topd_panel").show(ctx, |ui| {
            ui.heading("System Monitor");
            ui.label(format!("System name: {}", System::name().unwrap()));
            ui.label(format!(
                "System kernel version:  {}",
                System::kernel_version().unwrap()
            ));
            ui.label(format!(
                "System host name:         {}",
                System::host_name().unwrap()
            ));
            ui.label(format!(
                "System OS version:        {}",
                System::os_version().unwrap()
            ));
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.label(format!("Total Memory Usage: {} MB", tm));
            ui.label(format!("Used Memory Usage: {} MB", um));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CPU Usage");
            ui.label(format!("Total CPU Usage: {:.2}%", self.usage));
            ui.add(egui::ProgressBar::new(self.usage / 100.0).show_percentage());

            for (i, usage_itr) in self.usages.iter().enumerate() {
                ui.label(format!("CPU {} Usage: {:.2}%", i, usage_itr));
                ui.add(egui::ProgressBar::new(*usage_itr / 100.0).show_percentage());
            }

            ui.heading("System Disks");
            let disks = Disks::new_with_refreshed_list();
            for disk in disks.list() {
                ui.label(format!(
                    "[{:?}] {}GB",
                    disk.name(),
                    disk.available_space() / 1024 / 1024 / 1024
                ));
            }
        });

        ctx.request_repaint_after(Duration::from_secs_f32(1.0));
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "System Level",
        options,
        Box::new(|_cc| Box::new(CpuApp::default())),
    )
}
