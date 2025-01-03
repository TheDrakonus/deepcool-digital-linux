use super::{device_error, Mode};
use crate::{error, monitor::cpu::Cpu};
use hidapi::HidApi;
use rusb::{DeviceHandle, GlobalContext};
use std::{process::exit, thread::sleep, time::Duration};

pub const DEFAULT_MODE: Mode = Mode::Temperature;
pub const POLLING_RATE: u64 = 750;
pub const TEMP_LIMIT_C: u8 = 90;
pub const TEMP_LIMIT_F: u8 = 194;

pub struct Display {
    mode: Mode,
    fahrenheit: bool,
    alarm: bool,
    cpu: Cpu,
}

// Configs hit endpoint 1
// Status updates maybe endpoint 2

pub enum DisplayMode {
    Monitor,
    Media,
    Recording,
}
pub enum MainDisplayArea {
    // byte offset 2
    CPUFreq,
    Time,
    PumpSpeed,
    CPUFanSpeed,
    CPUFanSpeedAndPumpSpeed,
    Temperature,
}

pub enum StyleDispType {
    // byte offset 3
    Normal,
    Ring,
    Stopwatch,
}

pub enum GyroEnabled {
    // byte offset 4
    Off,
    On,
}
pub enum ScreenRotation {
    // byte offset 5
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

pub enum AuxDispMode {
    // byte offset 6
    Voltage,
    SysMon,
    CoreData,
}

pub struct ConfigureDisplay {
    mode: DisplayMode,
    main_display_area: MainDisplayArea,
    aux_mode: AuxDispMode,
    style_type: StyleDispType,
}

impl ConfigureDisplay {
    pub fn new(
        mode: DisplayMode,
        main_display_area: MainDisplayArea,
        aux_mode: AuxDispMode,
        style_type: StyleDispType,
    ) -> Self {
        ConfigureDisplay {
            mode,
            main_display_area,
            aux_mode,
            style_type,
        }
    }

    pub fn set(&self, api: DeviceHandle<GlobalContext>) {}
}

impl Display {
    pub fn new(mode: &Mode, fahrenheit: bool, alarm: bool) -> Self {
        // Verify the display mode
        let mode = match mode {
            Mode::Default => DEFAULT_MODE,
            Mode::Auto => Mode::Auto,
            Mode::Temperature => Mode::Temperature,
            Mode::Power => Mode::Power,
            _ => mode.support_error(),
        };

        Display {
            mode,
            fahrenheit,
            alarm,
            cpu: Cpu::new(),
        }
    }

    pub fn run(&self, api: &HidApi, vid: u16, pid: u16) {
        // Connect to device
        let device = api.open(vid, pid).unwrap_or_else(|_| device_error());

        // Check if `rapl_max_uj` was read correctly
        if self.cpu.rapl_max_uj == 0 {
            error!("Failed to get CPU power details");
            exit(1);
        }

        // Data packet
        let mut data: [u8; 48] = [0; 48];
        data[0] = 0xaa;
        data[1] = 0x2e;

        // Init sequence
        {
            let mut init_data = data.clone();
            device.write(&init_data).unwrap();
        }

        // // Display loop
        // match self.mode {
        //     Mode::Auto => loop {
        //         for _ in 0..8 {
        //             device.write(&self.status_message(&data, &Mode::Temperature)).unwrap();
        //         }
        //         for _ in 0..8 {
        //             device.write(&self.status_message(&data, &Mode::Power)).unwrap();
        //         }
        //     },
        //     _ => loop {
        //         device.write(&self.status_message(&data, &self.mode)).unwrap();
        //     },
        // }
    }
}
