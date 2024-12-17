use crate::{devices::Mode, error, DEFAULT_VENDOR};
use colored::*;
use hidapi::HidApi;
use std::{env::args, process::exit};

pub struct Args {
    pub mode: Mode,
    pub pid: u16,
    pub fahrenheit: bool,
    pub alarm: bool,
}

impl Args {
    pub fn read() -> Self {
        let args: Vec<String> = args().collect();
        let mut mode = Mode::Default;
        let mut pid = 0;
        let mut fahrenheit = false;
        let mut alarm = false;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-m" | "--mode" => {
                    if i + 1 < args.len() {
                        mode = match Mode::get(&args[i + 1]) {
                            Some(mode) => mode,
                            None => {
                                error!("Invalid mode");
                                exit(1);
                            }
                        };
                        i += 1;
                    } else {
                        error!("--mode requires a value");
                        exit(1);
                    }
                }
                "--pid" => {
                    if i + 1 < args.len() {
                        match args[i + 1].parse::<u16>() {
                            Ok(id) => {
                                if id > 0 {
                                    pid = id;
                                    i += 1;
                                } else {
                                    error!("Invalid PID");
                                    exit(1);
                                }
                            }
                            Err(_) => {
                                error!("Invalid PID");
                                exit(1);
                            }
                        }
                    } else {
                        error!("--pid requires a value");
                        exit(1);
                    }
                }
                "-f" | "--fahrenheit" => {
                    fahrenheit = true;
                }
                "-a" | "--alarm" => {
                    alarm = true;
                }
                "-l" | "--list" => {
                    println!("Device list [{} | {}]", "PID".bright_green().bold(), "Name".bright_green());
                    println!("-----");
                    let api = HidApi::new().unwrap_or_else(|err| {
                        error!(err);
                        exit(1);
                    });
                    let mut products = 0;
                    for device in api.device_list() {
                        if device.vendor_id() == DEFAULT_VENDOR {
                            products += 1;
                            println!(
                                "{} | {}",
                                device.product_id().to_string().bright_green().bold(),
                                device.product_string().unwrap().bright_green()
                            );
                            break;
                        }
                    }
                    if products == 0 {
                        error!("No DeepCool device was found");
                        exit(1);
                    }
                    exit(0);
                }
                "-h" | "--help" => {
                    println!("{} [OPTIONS]", "Usage: deepcool-digital-linux".bold());
                    println!("\n{}", "Options:".bold());
                    println!("  {}, {} <MODE>  Change the display mode of your device", "-m".bold(), "--mode".bold());
                    println!("      {} <ID>     Specify the Product ID if you use mutiple devices", "--pid".bold());
                    println!("  {}, {}   Change the temperature unit to °F", "-f".bold(), "--fahrenheit".bold());
                    println!("  {}, {}        Enable the alarm", "-a".bold(), "--alarm".bold());
                    println!("\n{}", "Commands:".bold());
                    println!("  {}, {}         Print Product ID of the connected devices", "-l".bold(), "--list".bold());
                    println!("  {}, {}         Print help", "-h".bold(), "--help".bold());
                    println!("  {}, {}      Print version", "-v".bold(), "--version".bold());
                    exit(0);
                }
                "-v" | "--version" => {
                    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                    exit(0);
                }
                arg if arg.starts_with('-') && arg.len() > 1 => {
                    for c in arg.chars().skip(1) {
                        match c {
                            'm' => {
                                if i + 1 < args.len() && args[i].ends_with('m') {
                                    mode = match Mode::get(&args[i + 1]) {
                                        Some(mode) => mode,
                                        None => {
                                            error!("Invalid mode");
                                            exit(1);
                                        }
                                    };
                                    i += 1;
                                } else {
                                    error!("--mode requires a value");
                                    exit(1);
                                }
                            }
                            'f' => fahrenheit = true,
                            'a' => alarm = true,
                            _ => {
                                if arg.starts_with("--") {
                                    error!(format!("Invalid option {arg}"));
                                } else {
                                    error!(format!("Invalid option -{c}"));
                                }
                                exit(1);
                            }
                        }
                    }
                }
                _ => {
                    error!(format!("Invalid option {}", args[i]));
                    exit(1);
                }
            }
            i += 1;
        }

        Args {
            mode,
            pid,
            fahrenheit,
            alarm,
        }
    }
}
