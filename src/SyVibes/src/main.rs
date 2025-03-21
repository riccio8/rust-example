use slint::SharedString;
use sysinfo::System;
use std::thread;
use std::time::Duration;

slint::include_modules!();  // This macro should define `MainWIndows`. Ensure the `.slint` file is correctly included and compiled.

fn main() {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    thread::spawn(move || {
        let mut sys = System::new_all();
        loop {
            sys.refresh_all();
            let mut list = String::new();

            for (pid, process) in sys.processes() {
                list.push_str(&format!("{:#?} with PID:\t {}\n", process.name(), pid));
            }
            let ui_handle_clone = ui_handle.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle_clone.upgrade() {
                    ui.set_processes(SharedString::from(list));
                }
            })
            .unwrap(); // If it falls here, it will panic and exit

            thread::sleep(Duration::from_secs(4)); 
        }
    });

    ui.run().unwrap();
}
