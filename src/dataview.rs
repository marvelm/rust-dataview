struct DataView {
    buffer: Vec<u8>
}

impl DataView {
    fn new() -> DataView {
        let buf: Vec<u8> = Vec::new();
        DataView{
            buffer: buf,
        }
    }

    fn set_i8(&mut self, offset: usize, value: i8) {
        // 1 byte
        self.buffer.insert(offset, value as u8);
    }

    fn set_i16(&mut self, offset: usize, value: i16) {
        // 2 bytes
        self.buffer.insert(offset, ((value >> 8) & 0xFF) as u8);
        self.buffer.insert(offset+1, (value & 0xFF) as u8);
    }

    fn set_i32(&mut self, offset: usize, value: i32) {
        // 4 bytes
        self.buffer.insert(offset, ((value >> 24) & 0xFF) as u8);
        self.buffer.insert(offset+1, ((value >> 16) & 0xFF) as u8);
        self.buffer.insert(offset+2, ((value >> 8) & 0xFF) as u8);
        self.buffer.insert(offset+3, (value & 0xFF) as u8);
    }

    fn set_u8(&mut self, offset: usize, value: u8) {
        // 1 bytes
        self.buffer.insert(offset, value);
    }

    fn set_u16(&mut self, offset: usize, value: u16) {
        // 2 bytes
        self.buffer.insert(offset, ((value >> 8) & 0xFF) as u8);
        self.buffer.insert(offset+1, (value & 0xFF) as u8);
    }

    fn set_u32(&mut self, offset: usize, value: u32) {
        // 4 bytes
        self.buffer.insert(offset, ((value >> 24) & 0xFF) as u8);
        self.buffer.insert(offset+1, ((value >> 16) & 0xFF) as u8);
        self.buffer.insert(offset+2, ((value >> 8) & 0xFF) as u8);
        self.buffer.insert(offset+3, (value & 0xFF) as u8);
    }

    fn set_f32(&mut self, offset: usize, value: f32) {
        // 4 bytes
        // mantSize 23, expSize 8
        let intRepr = value as u32;
        self.buffer.insert(offset, ((intRepr >> 24) & 0xFF) as u8);
        self.buffer.insert(offset+1, ((intRepr >> 16) & 0xFF) as u8);
        self.buffer.insert(offset+2, ((intRepr >> 8) & 0xFF) as u8);
        self.buffer.insert(offset+3, (intRepr  & 0xFF) as u8);
    }

    fn set_f64(&mut self, offset: usize, value: f64) {
       // 8 bytes
        let intRepr = value as u64;
        self.buffer.insert(offset, ((intRepr >> 56) & 0xFF) as u8);
        self.buffer.insert(offset+1, ((intRepr >> 48) & 0xFF) as u8);
        self.buffer.insert(offset+2, ((intRepr >> 40) & 0xFF) as u8);
        self.buffer.insert(offset+3, ((intRepr >> 32) & 0xFF) as u8);
        self.buffer.insert(offset+4, ((intRepr >> 24) & 0xFF) as u8);
        self.buffer.insert(offset+5, ((intRepr >> 16) & 0xFF) as u8);
        self.buffer.insert(offset+6, ((intRepr >> 8) & 0xFF) as u8);
        self.buffer.insert(offset+7, (intRepr  & 0xFF) as u8);
    }
}
