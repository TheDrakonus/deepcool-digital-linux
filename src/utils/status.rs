use crate::Mode;
use colored::*;

pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl TemperatureUnit {
    fn symbol(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
        }
    }
}

pub enum AlarmState {
    Auto,
    On,
    Off,
    NotSupported,
}

pub struct Alarm {
    pub state: AlarmState,
    pub temp_limit: u8,
}

pub fn print_device_status(mode: Mode, temp_unit: TemperatureUnit, alarm: Alarm, polling_rate: u64) {
    println!("-----");
    println!("DISP. MODE: {}", mode.symbol().bright_cyan());
    println!("TEMP. UNIT: {}", temp_unit.symbol().bright_cyan());
    match alarm.state {
        AlarmState::Auto => println!(
            "ALARM:      {} | {}",
            "auto".bright_green(),
            (alarm.temp_limit.to_string() + temp_unit.symbol()).bright_cyan()
        ),
        AlarmState::On => println!(
            "ALARM:      {} | {}",
            "on".bright_green(),
            (alarm.temp_limit.to_string() + temp_unit.symbol()).bright_cyan()
        ),
        AlarmState::Off => println!("ALARM:      {}", "off".bright_red()),
        AlarmState::NotSupported => println!("ALARM:      {}", "not supported".bright_black().italic()),
    }
    println!("-----");
    println!("Update interval: {}", format!("{}ms", polling_rate).bright_cyan());
    println!("\nPress {} to terminate", "Ctrl+C".bold());
}