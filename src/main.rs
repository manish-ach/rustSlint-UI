#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::SharedString;
use std::error::Error;

slint::include_modules!();

const PASSMARKS: i32 = 40;

fn main() -> Result<(), Box<dyn Error>> {
    // Creating a single instance of AppWindow
    let ui = AppWindow::new()?;

    // Creating a weak reference to the UI, which we can clone and move into the closure
    let ui_handle = ui.as_weak();

    // Setting up the event handler on this instance
    ui.on_marksCalc(
        move |s1: SharedString, s2: SharedString, s3: SharedString| {
            let marks = [s1.as_str(), s2.as_str(), s3.as_str()]
                .iter()
                .map(|&s| s.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>();

            let result = if marks.iter().any(|&mark| mark < PASSMARKS) {
                format!("Failed")
            } else {
                format!("Passed")
            };

            // using the weak reference to safely access the UI inside the closure
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_results(result.into());
            }
        },
    );

    // Run the app with the same instance
    ui.run()?;

    Ok(())
}
