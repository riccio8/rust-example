use slint::SharedString;
use sysinfo::System;
use std::thread;
use std::time::Duration;

slint::include_modules!();

fn main() {
    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    // Nuovo thread per aggiornare periodicamente la lista dei processi
    thread::spawn(move || {
        let mut sys = System::new_all();
        loop {
            sys.refresh_all();
            let mut list = String::new();

            for (pid, process) in sys.processes() {
                list.push_str(&format!("{:#?} con PID:\t {}\n", process.name(), pid));
            }

            // Usa invoke_from_event_loop per aggiornare la UI in modo sicuro
            let ui_handle_clone = ui_handle.clone();
            slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle_clone.upgrade() {
                    ui.set_processes(SharedString::from(list));
                    dbg!("Aggiornato");
                }
            })
            .unwrap(); // Se fallisce, esce con un panic

            thread::sleep(Duration::from_secs(4)); // Aggiorna ogni 4 secondi
        }
    });

    ui.run().unwrap();
}

// This macro should define `MainWIndows`. Ensure the `.slint` file is correctly included and compiled.