use crate::config::CONFIG;
use crate::types::ThreadsData;
use battery::Manager;

// TODO: better error handeling

// getting battery percentage
pub async fn get_battery() -> ThreadsData {
    let battery_manager = if let Ok(manager) = Manager::new() {
        manager
    } else {
        return ThreadsData::Battery(String::from("Cannot Create Battery Manager!"));
    };

    let mut batteries = if let Ok(batteries) = battery_manager.batteries() {
        batteries
    } else {
        return ThreadsData::Battery(String::from("Cannot Get Battery!"));
    };

    let percentage = if let Some(battery) = batteries.next() {
        f32::from(battery.unwrap().state_of_charge()) * 100.0
    } else {
        return ThreadsData::Battery(String::from("Cannot Read Battery!"));
    };
    
    let icons_amount = CONFIG.volume.icons.len() as u64;
    let interval: f64 = 100.0 / icons_amount as f64;
    let mut selected_icon  = (percentage as f64 / interval) as u64;
    if selected_icon >= icons_amount {
        selected_icon = icons_amount - 1;
    }

    let icon = &CONFIG.battery.icon[selected_icon as usize];

    

    let result = format!(
        "  {}  {:.0}%  {}",
        icon, percentage, CONFIG.seperator
    );
    ThreadsData::Battery(result)
}
