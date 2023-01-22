pub struct DexInstructionFormatReader<'a> {
    stream: &'a [u8],
    cursor: usize,
}

const LOW_NIBBLE: u8 = 0x0f;
const HIGH_NIBBLE: u8 = 0xf0;

#[allow(dead_code)]
impl<'a> DexInstructionFormatReader<'a> {
    pub fn new(stream: &'a [u8]) -> Self {
        Self { stream, cursor: 0 }
    }

    pub fn read_byte(&mut self) -> Option<(u8, usize)> {
        let cursor = self.cursor;
        Some((self.read_u8()?, cursor))
    }

    pub fn r_10x(&mut self) {
        self.read_u8();
    }

    pub fn r_12x(&mut self) -> Option<(u8, u8)> {
        self.get_single_byte_regs()
    }

    pub fn r_11n(&mut self) -> Option<(u8, u8)> {
        self.r_12x()
    }

    pub fn r_11x(&mut self) -> Option<u8> {
        Some(self.read_u8()?)
    }

    pub fn r_10t() -> Option<u8> {
        // idk wtf
        todo!()
    }

    pub fn r_20t(&mut self) -> Option<u16> {
        self.read_u8()?;
        Some(self.read_u16()?)
    }

    pub fn r_22x(&mut self) -> Option<(u8, u16)> {
        Some((self.read_u8()?, self.read_u16()?))
    }

    pub fn r_21c(&mut self) -> Option<(u8, u16)> {
        self.r_22x()
    }

    pub fn r_23x(&mut self) -> Option<(u8, u8, u8)> {
        Some((self.read_u8()?, self.read_u8()?, self.read_u8()?))
    }

    pub fn r_22b(&mut self) -> Option<(u8, u8, u8)> {
        self.r_23x()
    }

    pub fn r_22t(&mut self) -> Option<(u8, u8, u16)> {
        let (va, vb) = self.get_single_byte_regs()?;
        Some((va, vb, self.read_u16()?))
    }

    pub fn r_22s(&mut self) -> Option<(u8, u8, u16)> {
        self.r_22t()
    }

    pub fn r_22c(&mut self) -> Option<(u8, u8, u16)> {
        self.r_22t()
    }

    pub fn r_30t(&mut self) -> Option<u32> {
        self.read_u8()?;
        Some(self.read_u32()?)
    }

    pub fn r_32x(&mut self) -> Option<(u16, u16)> {
        self.read_u8()?;
        Some((self.read_u16()?, self.read_u16()?))
    }

    pub fn r_31i(&mut self) -> Option<(u8, u32)> {
        Some((self.read_u8()?, self.read_u32()?))
    }

    pub fn r_31t(&mut self) -> Option<(u8, u32)> {
        self.r_31i()
    }

    pub fn r_31c(&mut self) -> Option<(u8, u32)> {
        self.r_31i()
    }

    pub fn r_35c(&mut self) -> Option<(Vec<u8>, u16)> {
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

        Some((registers, value))
    }

    pub fn r_3rc(&mut self) -> Option<(u8, u16, u16)> {
        Some((self.read_u8()?, self.read_u16()?, self.read_u16()?))
    }

    pub fn r_45cc(&mut self) -> Option<(Vec<u8>, u16, u16)> {
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

        Some((registers, field1, field2))
    }

    pub fn r_4rcc(&mut self) -> Option<(u8, u16, u16, u16)> {
        Some((
            self.read_u8()?,
            self.read_u16()?,
            self.read_u16()?,
            self.read_u16()?,
        ))
    }

    pub fn r_51(&mut self) -> Option<(u8, u64)> {
        todo!()
    }

    fn read_u8(&mut self) -> Option<u8> {
        if self.cursor >= self.stream.len() {
            None
        } else {
            let result = self.stream[self.cursor];
            self.cursor += 1;
            Some(result)
        }
    }

    fn get_single_byte_regs(&mut self) -> Option<(u8, u8)> {
        let value = self.read_u8()?;
        Some((value & LOW_NIBBLE, value & HIGH_NIBBLE))
    }

    fn read_u16(&mut self) -> Option<u16> {
        Some(((self.read_u8()? as u16) << 8) + self.read_u8()? as u16)
    }

    fn read_u32(&mut self) -> Option<u32> {
        Some(((self.read_u16()? as u32) << 16) + self.read_u16()? as u32)
    }
}
