// ------------------------------------------------------------------------------
// Copyright 2018 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

//! Driver for AMS AS5048A Magnetic Rotary Encoder

#![no_std]

use core::fmt;

use embedded_hal::spi::SpiDevice;

/// Error
pub enum Error<SPI>
where
    SPI: SpiDevice<u8>,
{
    Spi(SPI::Error),
}

impl<SPI> fmt::Debug for Error<SPI>
where
    SPI: SpiDevice<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Spi(error) => write!(f, "Spi({:?})", error),
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

/// AS5048A driver
pub struct AS5048A<SPI: SpiDevice<u8>> {
    spi: SPI,
}

impl<SPI, E> AS5048A<SPI>
where
    SPI: SpiDevice<u8, Error = E>,
{
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    pub fn diag_gain(&mut self) -> Result<(u8, u8), Error<SPI>> {
        self.read(Register::DiagAgc)
            .map(|arr| (arr[0] & 0x0f, arr[1]))
    }

    pub fn magnitude(&mut self) -> Result<u16, Error<SPI>> {
        self.read_u16(Register::Magnitude)
    }

    /// Read the rotation angle as u16 (only 14 bits are significant)
    pub fn angle(&mut self) -> Result<u16, Error<SPI>> {
        self.read_u16(Register::Angle)
    }

    fn read_u16(&mut self, reg: Register) -> Result<u16, Error<SPI>> {
        match self.read(reg) {
            Ok(arr) => {
                let y = u16::from_be_bytes(arr);
                Ok(y & 0b0011_1111_1111_1111)
            }
            Err(e) => Err(e),
        }
    }

    fn read(&mut self, reg: Register) -> Result<[u8; 2], Error<SPI>> {
        // send cmd
        let mut cmd: u16 = 0b_0100_0000_0000_0000;
        cmd |= reg as u16;
        cmd = set_parity(cmd);

        let bytes = cmd.to_be_bytes();
        let mut result = [0u8; 2];
        self.spi.transfer(&mut result, &bytes).map_err(Error::Spi)?;

        // send nop to get result back
        let nop = [0x00, 0x00];
        self.spi.transfer(&mut result,&nop).map_err(Error::Spi)?;

        Ok(result)
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
