// ------------------------------------------------------------------------------
// Copyright 2018 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

//! Driver for AMS AS5048A Magnetic Rotary Encoder

#![no_std]

use core::fmt;

use embedded_hal as hal;
use hal::blocking::spi::Transfer;
use hal::digital::v2::OutputPin;

pub enum Error<SPI, CS>
where
    SPI: Transfer<u8>,
    CS: OutputPin,
{
    Spi(SPI::Error),
    ChipSelect(CS::Error),
}

impl<SPI, CS> fmt::Debug for Error<SPI, CS>
where
    SPI: Transfer<u8>,
    <SPI as Transfer<u8>>::Error: fmt::Debug,
    CS: OutputPin,
    <CS as OutputPin>::Error: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Spi(error) => write!(f, "Spi({:?})", error),
            Error::ChipSelect(error) => write!(f, "ChipSelect({:?})", error),
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum Register {
    ClearErrorFlag = 0x0001,
    ProgrammingControl = 0x0003,
    OtpRegisterZeroPosHigh = 0x0016,
    OtpRegisterZeroPosLow = 0x0017,
    DiagAgc = 0x3FFD,
    Magnitude = 0x3FFE,
    Angle = 0x3FFF,
}

pub struct AS5048A<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, CS, E> AS5048A<SPI, CS>
where
    SPI: Transfer<u8, Error = E>,
    CS: OutputPin,
{
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self { spi, cs }
    }

    pub fn diag_gain(&mut self) -> Result<(u8, u8), Error<SPI, CS>> {
        self.read(Register::DiagAgc)
            .map(|arr| (arr[0] & 0x0f, arr[1]))
    }

    pub fn magnitude(&mut self) -> Result<u16, Error<SPI, CS>> {
        self.read_u16(Register::Magnitude)
    }

    pub fn angle(&mut self) -> Result<u16, Error<SPI, CS>> {
        self.read_u16(Register::Angle)
    }

    fn read_u16(&mut self, reg: Register) -> Result<u16, Error<SPI, CS>> {
        match self.read(reg) {
            Ok(arr) => {
                let y = u16::from_be_bytes(arr);
                Ok(y & 0b0011_1111_1111_1111)
            }
            Err(e) => Err(e),
        }
    }

    fn read(&mut self, reg: Register) -> Result<[u8; 2], Error<SPI, CS>> {
        // send cmd
        let mut cmd: u16 = 0b_0100_0000_0000_0000;
        cmd |= reg as u16;
        cmd = set_parity(cmd);

        let mut bytes = cmd.to_be_bytes();

        self.cs.set_low().map_err(Error::ChipSelect)?;
        self.spi.transfer(&mut bytes).map_err(Error::Spi)?;
        self.cs.set_high().map_err(Error::ChipSelect)?;

        // send nop to get result back
        let mut nop = [0x00, 0x00];
        self.cs.set_low().map_err(Error::ChipSelect)?;
        self.spi.transfer(&mut nop).map_err(Error::Spi)?;
        self.cs.set_high().map_err(Error::ChipSelect)?;

        Ok(nop)
    }
}

const fn set_parity(par: u16) -> u16 {
    let mut x = par;

    x = (x & 0x00FF) ^ (x >> 8);
    x = (x & 0x000F) ^ (x >> 4);
    x = (x & 0x0003) ^ (x >> 2);
    x = (x & 0x0001) ^ (x >> 1);

    if x == 0x0001 {
        par | 0b1000_0000_0000_0000
    } else {
        par
    }
}
