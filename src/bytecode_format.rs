use crate::{errors::Error, Result};
use std::{io::{Cursor, Read}, mem};

const LOW_NIBBLE: u8 = 0x0f;
const HIGH_NIBBLE: u8 = 0xf0;

pub struct DexInstructionFormatReader<'a> {
    cursor: Cursor<&'a [u8]>,
}

impl<'a> DexInstructionFormatReader<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(&stream),
        }
    }

    pub fn read_byte(&mut self) -> Result<(u8, usize)> {
        let position = self.cursor.position();
        let value = self.read_u8()?;
        Ok((value, position as usize))
    }

    pub fn r_12x(&mut self) -> Result<(u8, u8)> {
        self.get_single_byte_regs()
    }

    pub fn r_11n(&mut self) -> Result<(u8, i8)> {
        let (reg, value) = self.get_single_byte_regs()?;
        Ok((reg, value as i8))
    }

    pub fn r_11x(&mut self) -> Result<u8> {
        self.read_u8()
    }

    pub fn r_10t(&mut self) -> Result<i8> {
        self.read_i8()
    }

    pub fn r_20t(&mut self) -> Result<i16> {
        self.read_u8()?;
        self.read_i16()
    }

    pub fn r_22x(&mut self) -> Result<(u8, u16)> {
        Ok((self.read_u8()?, self.read_u16()?))
    }

    pub fn r_21t(&mut self) -> Result<(u8, i16)> {
        Ok((self.read_u8()?, self.read_i16()?))
    }

    pub fn r_21s(&mut self) -> Result<(u8, i16)> {
        self.r_21t()
    }

    pub fn r_21h(&mut self) -> Result<(u8, i16)> {
        self.r_21t()
    }

    pub fn r_21c(&mut self) -> Result<(u8, u16)> {
        self.r_22x()
    }

    pub fn r_23x(&mut self) -> Result<(u8, u8, u8)> {
        Ok((self.read_u8()?, self.read_u8()?, self.read_u8()?))
    }

    pub fn r_22b(&mut self) -> Result<(u8, u8, i8)> {
        Ok((self.read_u8()?, self.read_u8()?, self.read_i8()?))
    }

    pub fn r_22t(&mut self) -> Result<(u8, u8, i16)> {
        let (va, vb) = self.get_single_byte_regs()?;
        Ok((va, vb, self.read_i16()?))
    }

    pub fn r_22s(&mut self) -> Result<(u8, u8, i16)> {
        self.r_22t()
    }

    pub fn r_22c(&mut self) -> Result<(u8, u8, u16)> {
        let (va, vb) = self.get_single_byte_regs()?;
        Ok((va, vb, self.read_u16()?))
    }

    pub fn r_30t(&mut self) -> Result<i32> {
        self.read_u8()?;
        Ok(self.read_i32()?)
    }

    pub fn r_32x(&mut self) -> Result<(u16, u16)> {
        self.read_u8()?;
        Ok((self.read_u16()?, self.read_u16()?))
    }

    pub fn r_31i(&mut self) -> Result<(u8, i32)> {
        Ok((self.read_u8()?, self.read_i32()?))
    }

    pub fn r_31t(&mut self) -> Result<(u8, i32)> {
        Ok((self.read_u8()?, self.read_i32()?))
    }

    pub fn r_31c(&mut self) -> Result<(u8, u32)> {
        Ok((self.read_u8()?, self.read_u32()?))
    }

    pub fn r_35c(&mut self) -> Result<(Vec<u8>, u16)> {
        let mut registers = vec![];

        let (g, number_of_registers) = self.get_single_byte_regs()?;
        let value = self.read_u16()?;

        let (c, d) = self.get_single_byte_regs()?;
        let (e, f) = self.get_single_byte_regs()?;

        registers.push(c);
        registers.push(d);
        registers.push(e);
        registers.push(f);
        registers.push(g);

        while number_of_registers as usize != registers.len() {
            registers.pop();
        }

        Ok((registers, value))
    }

    pub fn r_3rc(&mut self) -> Result<(u8, u16, u16)> {
        Ok((self.read_u8()?, self.read_u16()?, self.read_u16()?))
    }

    pub fn r_45cc(&mut self) -> Result<(Vec<u8>, u16, u16)> {
        let mut registers = vec![];

        let (g, number_of_registers) = self.get_single_byte_regs()?;
        let field1 = self.read_u16()?;
        let (c, d) = self.get_single_byte_regs()?;
        let (e, f) = self.get_single_byte_regs()?;
        let field2 = self.read_u16()?;

        registers.push(c);
        registers.push(d);
        registers.push(e);
        registers.push(f);
        registers.push(g);

        while number_of_registers as usize != registers.len() {
            registers.pop();
        }

        Ok((registers, field1, field2))
    }

    pub fn r_4rcc(&mut self) -> Result<(u8, u16, u16, u16)> {
        Ok((
            self.read_u8()?,
            self.read_u16()?,
            self.read_u16()?,
            self.read_u16()?,
        ))
    }

    pub fn r_51(&mut self) -> Result<(u8, u64)> {
        Ok((self.read_u8()?, self.read_u64()?))
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut out = [0u8; 1];
        let please = match self.cursor.read_exact(&mut out) {
            Ok(_) => Ok(u8::from_le_bytes(out)),
            Err(_) => Err(Error::ReadByteFailed),
        };

        please
    }

    fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    fn read_u16(&mut self) -> Result<u16> {
        Ok(((self.read_u8()? as u16) << (mem::size_of::<u16>() * 4)) + self.read_u8()? as u16)
    }

    fn read_i16(&mut self) -> Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    fn read_u32(&mut self) -> Result<u32> {
        Ok(((self.read_u16()? as u32) << (mem::size_of::<u32>() * 4)) + self.read_u16()? as u32)
    }

    fn read_i32(&mut self) -> Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    fn read_u64(&mut self) -> Result<u64> {
        Ok(((self.read_u32()? as u64) << (mem::size_of::<u64>() * 4)) + self.read_u32()? as u64)
    }

    fn get_single_byte_regs(&mut self) -> Result<(u8, u8)> {
        let value = self.read_u8()?;
        Ok((value & LOW_NIBBLE, (value & HIGH_NIBBLE) >> mem::size_of::<u8>() * 4))
    }
}
