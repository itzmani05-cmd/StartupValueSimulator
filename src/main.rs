use leptos::*;
use leptos_meta::*;
use startup_equity_scenario_builder::App;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");

    mount_to_body(|| {
        view! {
            <App />
        }
    });
}
