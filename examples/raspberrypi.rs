// ------------------------------------------------------------------------------
// Copyright 2018 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

use linux_embedded_hal::sysfs_gpio::Pin;
use linux_embedded_hal::{self as hal, Delay};

use as5048a::AS5048A;

use crate::hal::spidev::{self, SpidevOptions};
use crate::hal::sysfs_gpio::Direction;
use std::thread;
use std::time::Duration;
use linux_embedded_hal::SpidevDevice;

fn main() -> Result<(), std::io::Error> {
    let mut spi = SpidevDevice::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new()
        .max_speed_hz(1_000_000)
        .mode(spidev::SpiModeFlags::SPI_MODE_1)
        .build();
    spi.configure(&options).unwrap();

    
    // CS pin on SparkFun Breakout
    let ncs = Pin::new(8);
    ncs.export().unwrap();
    while !ncs.is_exported() {}
    ncs.set_direction(Direction::Out).unwrap();
    ncs.set_value(1).unwrap();

    let mut as5048: AS5048A<_, Delay, 0> = AS5048A::new(spi, None);

    println!("AS5048A Example");
    loop {
        println!("-------------------------------------------------------------------------");

        let (diag, gain) = as5048.diag_gain().unwrap();
        println!("diag: {:08b} gain: {}", diag, gain);
        println!("magnitude: {:?}", as5048.magnitude());
        println!("angle: {:?}", as5048.angle());
        thread::sleep(Duration::from_millis(1000));
    }
}
