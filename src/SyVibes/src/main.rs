use slint::SharedString;
use sysinfo::{Components, Disks, Networks, System};
use std::thread;
use std::time::Duration;

slint::include_modules!();  // This macro should define `MainWIndows`
// in cargo.toml: [package.metadata.winres] windows_subsystem = "Windows"


fn main() {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();
    // ui.window().set_maximized(true);

    // Clone the ui_handle at the start before moving it into the threads otherwise it won't access it
    let ui_handle_clone_1 = ui_handle.clone();
    let ui_handle_clone_2 = ui_handle.clone();
    let ui_handle_clone_3 = ui_handle.clone();
    sysinfo::set_open_files_limit(0);

    // Start the first thread for system information.
    thread::spawn(move || {
        let mut sys = System::new_all();
        loop {
            sys.refresh_all();
            let mut list = String::new();
            let mut mem = String::new();
            let mut cpu = String::new();
            let mut system = String::new();

            for (pid, process) in sys.processes() {
                list.push_str(&format!("{:#?} with PID:\t {}\n", process.name(), pid));
            }

            cpu.push_str(&format!("NB CPUs: {}\n", sys.cpus().len()));
            for (i, cpu_info) in sys.cpus().iter().enumerate() {
                cpu.push_str(&format!("CPU {} Usage: {}%\n", i, cpu_info.cpu_usage()));
            }
            mem.push_str(&format!("Total memory: \t{} bytes \nUsed memory: \t{} bytes \nTotal swap: \t{} bytes \nUsed swap: \t{} bytes",
             sys.total_memory(), 
            sys.used_memory(), 
            sys.total_swap(),
            sys.used_swap()));

            system.push_str(&format!(        "System Information:\n\
            \tSystem name:           {:#?}\n\
            \tKernel version:        {:#?}\n\
            \tOS version:            {:#?}\n\
            \tHost name:             {:#?}",
            System::name(),
            System::kernel_version(),
            System::os_version(),
            System::host_name()));

            let ui_handle_clone = ui_handle_clone_1.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle_clone.upgrade() {
                    ui.set_processes(SharedString::from(list));
                    ui.set_memory(SharedString::from(mem));
                    ui.set_cpu(SharedString::from(cpu));
                    ui.set_system(SharedString::from(system));
                }
            })
            .unwrap();

            thread::sleep(Duration::from_secs(4)); 
        }
    });

    // Start the second thread for network information.
    thread::spawn(move || {
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
                     \t\tTransmitted: {}\n\
                     \t\tTransmitted: {}\n\n",
                    interface_name,
                    data.total_received(),
                    data.total_transmitted(),
                    data.packets_received(),
                    data.packets_transmitted(),
                    data.errors_on_received(),
                    data.errors_on_transmitted(),
                    data.transmitted(),
                ));
            }

            let ui_handle_clone = ui_handle_clone_2.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle_clone.upgrade() {
                    ui.set_network(SharedString::from(nets.clone()));
                }
            })
            .unwrap();

            thread::sleep(Duration::from_secs(4));
            networks.refresh(true);
        }
    });

    // Start the third thread for disk information.
    thread::spawn(move || -> ! {
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
                disk.is_read_only(),
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

            // Use the cloned handle in the closure
            let ui_handle_clone = ui_handle_clone_3.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle_clone.upgrade() {
                    ui.set_disk(SharedString::from(disks_info.clone()));
                    ui.set_components(SharedString::from(comp.clone()));
                }
            })
            .unwrap();

            thread::sleep(Duration::from_secs(4));
            disks.refresh(true); 
            components.refresh(false);
            
        }
    });

    // Run the UI event loop
    ui.run().unwrap();
}
