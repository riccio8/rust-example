use slint::SharedString;
use std::sync::{Arc, Mutex};
use sysinfo::{Components, Disks, Networks, System, ProcessesToUpdate,ProcessRefreshKind};
use tokio::time::{sleep, Duration};


slint::include_modules!();  // This macro should define `MainWIndows`
// in cargo.toml: [package.metadata.winres] windows_subsystem = "Windows"

#[tokio::main]
async fn main() {
let ui = MainWindow::new().unwrap();
let ui_handle = ui.as_weak();
// ui.window().set_maximized(true);
tokio::spawn(update_system_info(ui_handle.clone()));
tokio::spawn(update_process_info(ui_handle.clone()));
tokio::spawn(update_network_info(ui_handle.clone()));
tokio::spawn(update_disk_info(ui_handle.clone()));

ui.run().unwrap();
}


async fn update_system_info(ui_handle: slint::Weak<MainWindow>) {
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        let mem = format!(
            "Total memory: \t{} bytes \nUsed memory: \t{} bytes \nTotal swap: \t{} bytes \nUsed swap: \t{} bytes",
            sys.total_memory(),
            sys.used_memory(),
            sys.total_swap(),
            sys.used_swap()
        );

        let cpu = sys
            .cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu_info)| format!("CPU {} Usage: {}%", i, cpu_info.cpu_usage()))
            .collect::<Vec<_>>()
            .join("\n");

        let system = format!(
            "System Information:\n\
            \tSystem name:           {:#?}\n\
            \tKernel version:        {:#?}\n\
            \tOS version:            {:#?}\n\
            \tHost name:             {:#?}\n",
            System::name(),
            System::kernel_version(),
            System::os_version(),
            System::host_name()
        );

        let ui_handle_clone = ui_handle.clone();
        slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_handle_clone.upgrade() {
                ui.set_memory(SharedString::from(mem));
                ui.set_cpu(SharedString::from(cpu));
                ui.set_system(SharedString::from(system));
            }
        })
        .unwrap();

        sleep(Duration::from_secs(3)).await;
    }
}

async fn update_process_info(ui_handle: slint::Weak<MainWindow>) {
    let sys = Arc::new(Mutex::new(System::new_all()));

    loop {
        {
            let mut sys_guard = sys.lock().unwrap(); // Stop the mutex before accessing sys
            sys_guard.refresh_processes_specifics(
                ProcessesToUpdate::All,
                true,
                ProcessRefreshKind::everything().without_cpu().without_disk_usage().without_environ(),
            );
        }

        let sys_clone = Arc::clone(&sys);
        let ui_handle_clone = ui_handle.clone();

        slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_handle_clone.upgrade() {
                let sys_guard = sys_clone.lock().unwrap(); // Reloc before reading sys
                let mut procinfo = String::new();

                for (pid, process) in sys_guard.processes() {
                    procinfo.push_str(&format!(
                        "Process {:#?} (PID: {:#?})\nPath: {:?}\nMemory: {} KB\nCPU: {:.2}%\nStatus: {:?}\n",
                        process.name(),
                        pid,
                        process.exe(),
                        process.memory(),
                        process.cpu_usage(),
                        process.status()
                    ));

                    let disk_usage = process.disk_usage();
                    procinfo.push_str(&format!(
                        "Read bytes: new/total => {:?}/{:?}\nWritten bytes: new/total => {:?}/{:?}\n",
                        disk_usage.read_bytes,
                        disk_usage.total_read_bytes,
                        disk_usage.written_bytes,
                        disk_usage.total_written_bytes
                    ));
                }

                ui.set_processInfo(SharedString::from(procinfo));

                let sys_clone = Arc::clone(&sys_clone);
                ui.on_killProc(move |string: SharedString| {
                    let  sys_guard = sys_clone.lock().unwrap();
                    for (_, process) in sys_guard.processes() {
                        if process.name() == string.as_str() {
                            process.kill();
                        }
                    }
                });
            }
        })
        .unwrap();

        sleep(Duration::from_secs(3)).await;
    }
}



async fn update_network_info(ui_handle: slint::Weak<MainWindow>) {
    let mut networks = Networks::new_with_refreshed_list();

    loop {
        let mut nets = String::new();
        for (interface_name, data) in &networks {
            nets.push_str(&format!(
                "{}:\n\
                 \tTotal Data:\n\
                 \t\tReceived: {} B\n\
                 \t\tTransmitted: {} B\n\
                 \tTotal Packets:\n\
                 \t\tReceived: {}\n\
                 \t\tTransmitted: {}\n\
                 \tErrors:\n\
                 \t\tReceived: {}\n\
                 \t\tTransmitted: {}\n\n",
                interface_name,
                data.total_received(),
                data.total_transmitted(),
                data.packets_received(),
                data.packets_transmitted(),
                data.errors_on_received(),
                data.errors_on_transmitted()
            ));
        }

        let ui_handle_clone = ui_handle.clone();
        slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_handle_clone.upgrade() {
                ui.set_network(SharedString::from(nets.clone()));
            }
        })
        .unwrap();

        sleep(Duration::from_secs(3)).await;
        networks.refresh(true);
    }
}


async fn update_disk_info(ui_handle: slint::Weak<MainWindow>) {
    let mut disks = Disks::new_with_refreshed_list();
    let mut components = Components::new_with_refreshed_list();

    loop {
        let mut disks_info = String::new();
        let mut comp = String::new();

        for disk in disks.iter() {
            disks_info.push_str(&format!(
                "Disk: {:#?}\n\
                \tType: {:?}\n\
                \tFile system: {:?}\n\
                \tMount point: {:?}\n\
                \tTotal space: {} B\n\
                \tAvailable space: {} B\n\
                \tUsed space: {} B\n\
                \tIs removable: {}\n\
                \tIs read-only: {}\n\n",
                disk.name(),
                disk.kind(),
                disk.file_system(),
                disk.mount_point(),
                disk.total_space(),
                disk.available_space(),
                disk.total_space() - disk.available_space(),
                disk.is_removable(),
                disk.is_read_only()
            ));
        }

        if components.is_empty() {
            comp.push_str("No components found.\n");
        }

        for component in &components {
            if let Some(temperature) = component.temperature() {
                comp.push_str(&format!("{} \t{:#?}°C\n", component.label(), temperature));
            } else {
                comp.push_str(&format!("{} (unknown temperature)\n", component.label()));
            }
        }

        let ui_handle_clone = ui_handle.clone();
        slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_handle_clone.upgrade() {
                ui.set_disk(SharedString::from(disks_info.clone()));
                ui.set_components(SharedString::from(comp.clone()));
            }
        })
        .unwrap();

        sleep(Duration::from_secs(4)).await;
        disks.refresh(true);
        components.refresh(false);
    }
}
