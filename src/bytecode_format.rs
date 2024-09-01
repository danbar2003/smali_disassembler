use crate::{errors::Error, Result};
use std::io::{Cursor, Read};

pub struct DexInstructionFormatReader<'a> {
    cursor: Cursor<&'a [u8]>,
}

const LOW_NIBBLE: u8 = 0x0f;
const HIGH_NIBBLE: u8 = 0xf0;

#[allow(dead_code)]
impl<'a> DexInstructionFormatReader<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(&stream),
        }
    }

    pub fn reset(&mut self) {
        self.cursor.set_position(0);
    }

    pub fn read_byte(&mut self) -> Result<(u8, usize)> {
        let position = self.cursor.position();
        let value = self.read_u8()?;
        Ok((value, position as usize))
    }

    pub fn r_12x(&mut self) -> Result<(u8, u8)> {
        self.get_single_byte_regs()
    }

    pub fn r_11n(&mut self) -> Result<(u8, u8)> {
        self.r_12x()
    }

    pub fn r_11x(&mut self) -> Result<u8> {
        self.read_u8()
    }

    pub fn r_10t() -> Option<u8> {
        // idk wtf
        todo!()
    }

    pub fn r_20t(&mut self) -> Result<u16> {
        self.read_u8()?;
        self.read_u16()
    }

    pub fn r_22x(&mut self) -> Result<(u8, u16)> {
        Ok((self.read_u8()?, self.read_u16()?))
    }

    pub fn r_21s(&mut self) -> Result<(u8, u16)> {
        self.r_22x()
    }

    pub fn r_21h(&mut self) -> Result<(u8, u16)> {
        self.r_22x()
    }

    pub fn r_21c(&mut self) -> Result<(u8, u16)> {
        self.r_22x()
    }

    pub fn r_23x(&mut self) -> Result<(u8, u8, u8)> {
        Ok((self.read_u8()?, self.read_u8()?, self.read_u8()?))
    }

    pub fn r_22b(&mut self) -> Result<(u8, u8, u8)> {
        self.r_23x()
    }

    pub fn r_22t(&mut self) -> Result<(u8, u8, u16)> {
        let (va, vb) = self.get_single_byte_regs()?;
        Ok((va, vb, self.read_u16()?))
    }

    pub fn r_22s(&mut self) -> Result<(u8, u8, u16)> {
        self.r_22t()
    }

    pub fn r_22c(&mut self) -> Result<(u8, u8, u16)> {
        self.r_22t()
    }

    pub fn r_30t(&mut self) -> Result<u32> {
        self.read_u8()?;
        Ok(self.read_u32()?)
    }

    pub fn r_32x(&mut self) -> Result<(u16, u16)> {
        self.read_u8()?;
        Ok((self.read_u16()?, self.read_u16()?))
    }

    pub fn r_31i(&mut self) -> Result<(u8, u32)> {
        Ok((self.read_u8()?, self.read_u32()?))
    }

    pub fn r_31t(&mut self) -> Result<(u8, u32)> {
        self.r_31i()
    }

    pub fn r_31c(&mut self) -> Result<(u8, u32)> {
        self.r_31i()
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
        todo!()
    }

    fn read_u8(&mut self) -> Result<u8> {
        let result_byte = 0;
        match self.cursor.read_exact(&mut [result_byte]) {
            Ok(_) => Ok(result_byte),
            Err(_) => Err(Error::ReadByteFailed),
        }
    }

    fn read_u16(&mut self) -> Result<u16> {
        Ok(((self.read_u8()? as u16) << 8) + self.read_u8()? as u16)
    }

    fn read_u32(&mut self) -> Result<u32> {
        Ok(((self.read_u16()? as u32) << 16) + self.read_u16()? as u32)
    }

    fn get_single_byte_regs(&mut self) -> Result<(u8, u8)> {
        let value = self.read_u8()?;
        Ok((value & LOW_NIBBLE, value & HIGH_NIBBLE))
    }
}
