use core::mem;

pub struct ByteReader<'a> {
    bytes: &'a [u8],
    little_endian: bool,
}

pub trait FromU64 {
    fn convert(v: u64) -> Self;
}

impl FromU64 for u32 {
    fn convert(v: u64) -> Self {
        v as u32
    }
}
impl FromU64 for u16 {
    fn convert(v: u64) -> Self {
        v as u16
    }
}
impl FromU64 for u8 {
    fn convert(v: u64) -> Self {
        v as u8
    }
}
impl FromU64 for u64 {
    fn convert(v: u64) -> Self {
        v
    }
}

impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8], little_endian: bool) -> ByteReader<'a> {
        ByteReader {
            bytes,
            little_endian,
        }
    }

    pub fn uint_at_offset<T: FromU64>(&self, offset: usize) -> T {
        let mut result = 0;
        let max_bytes = mem::size_of::<T>();
        for i in 0..max_bytes {
            match self.little_endian {
                true => {
                    result |= (self.bytes[i + offset] as u64) << (i * max_bytes);
                }
                false => {
                    result |= (self.bytes[i + offset] as u64) << ((max_bytes - 1 - i) * max_bytes);
                }
            }
        }
        T::convert(result)
    }
}
