use std::{io::Result, mem::size_of};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Native,
    Little,
    Big,
}

macro_rules! impl_read_method {
    ($name:ident, $type:ty) => {
        #[inline]
        fn $name(&mut self) -> Result<$type> {
            let mut buf = [0; size_of::<$type>()];
            self.read_exact(&mut buf)?;
            Ok(<$type>::from_ne_bytes(buf))
        }
    };
}

macro_rules! impl_read_endian_method {
    ($name:ident, $type:ty) => {
        #[inline]
        fn $name(&mut self, endian: Endian) -> Result<$type> {
            let mut buf = [0; size_of::<$type>()];
            self.read_exact(&mut buf)?;
            Ok(match endian {
                Endian::Native => <$type>::from_ne_bytes(buf),
                Endian::Little => <$type>::from_le_bytes(buf),
                Endian::Big => <$type>::from_be_bytes(buf),
            })
        }
    };
}

macro_rules! impl_write_method {
    ($name:ident, $type:ty) => {
        #[inline]
        fn $name(&mut self, value: $type) -> Result<()> {
            self.write_all(&value.to_ne_bytes())
        }
    };
}

macro_rules! impl_write_endian_method {
    ($name:ident, $type:ty) => {
        #[inline]
        fn $name(&mut self, value: $type, endian: Endian) -> Result<()> {
            let bytes = match endian {
                Endian::Native => value.to_ne_bytes(),
                Endian::Little => value.to_le_bytes(),
                Endian::Big => value.to_be_bytes(),
            };
            self.write_all(&bytes)
        }
    };
}

pub trait ReadBytes: std::io::Read {
    impl_read_method!(read_u8, u8);

    impl_read_method!(read_u16, u16);
    impl_read_endian_method!(read_u16_endian, u16);

    impl_read_method!(read_u32, u32);
    impl_read_endian_method!(read_u32_endian, u32);

    impl_read_method!(read_u64, u64);
    impl_read_endian_method!(read_u64_endian, u64);

    impl_read_method!(read_u128, u128);
    impl_read_endian_method!(read_u128_endian, u128);

    impl_read_method!(read_i8, i8);

    impl_read_method!(read_i16, i16);
    impl_read_endian_method!(read_i16_endian, i16);

    impl_read_method!(read_i32, i32);
    impl_read_endian_method!(read_i32_endian, i32);

    impl_read_method!(read_i64, i64);
    impl_read_endian_method!(read_i64_endian, i64);

    impl_read_method!(read_i128, i128);
    impl_read_endian_method!(read_i128_endian, i128);

    impl_read_method!(read_f32, f32);
    impl_read_endian_method!(read_f32_endian, f32);

    impl_read_method!(read_f64, f64);
    impl_read_endian_method!(read_f64_endian, f64);

    #[inline]
    fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; count];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    #[inline]
    fn read_value<T: Default + Copy>(&mut self) -> Result<T> {
        let mut value = T::default();
        let buf = core::slice::from_mut(&mut value);
        let buf = unsafe {
            core::slice::from_raw_parts_mut::<u8>(buf.as_mut_ptr().cast(), size_of::<T>())
        };
        self.read_exact(buf)?;
        Ok(value)
    }
}

impl<R: std::io::Read> ReadBytes for R {}

pub trait WriteBytes: std::io::Write {
    #[inline]
    fn write_u8(&mut self, value: u8) -> Result<()> {
        self.write_all(&[value])
    }

    impl_write_method!(write_u16, u16);
    impl_write_endian_method!(write_u16_endian, u16);

    impl_write_method!(write_u32, u32);
    impl_write_endian_method!(write_u32_endian, u32);

    impl_write_method!(write_u64, u64);
    impl_write_endian_method!(write_u64_endian, u64);

    impl_write_method!(write_u128, u128);
    impl_write_endian_method!(write_u128_endian, u128);

    #[inline]
    fn write_i8(&mut self, value: i8) -> Result<()> {
        self.write_all(&value.to_ne_bytes())
    }

    impl_write_method!(write_i16, i16);
    impl_write_endian_method!(write_i16_endian, i16);

    impl_write_method!(write_i32, i32);
    impl_write_endian_method!(write_i32_endian, i32);

    impl_write_method!(write_i64, i64);
    impl_write_endian_method!(write_i64_endian, i64);

    impl_write_method!(write_i128, i128);
    impl_write_endian_method!(write_i128_endian, i128);

    impl_write_method!(write_f32, f32);
    impl_write_endian_method!(write_f32_endian, f32);

    impl_write_method!(write_f64, f64);
    impl_write_endian_method!(write_f64_endian, f64);

    #[inline]
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.write_all(bytes)
    }

    #[inline]
    fn write_value<T: Copy>(&mut self, value: &T) -> Result<()> {
        let buf = core::slice::from_ref(value);
        let buf = unsafe { core::slice::from_raw_parts::<u8>(buf.as_ptr().cast(), size_of::<T>()) };
        self.write_all(buf)
    }
}

impl<W: std::io::Write> WriteBytes for W {}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::{Endian, ReadBytes, WriteBytes};

    #[test]
    fn read_write_native_roundtrip() {
        let mut cursor = Cursor::new(Vec::new());

        cursor.write_u32(0x1234_5678).unwrap();
        cursor.write_i16(-1234).unwrap();
        cursor.write_f32(3.5).unwrap();

        cursor.set_position(0);

        assert_eq!(cursor.read_u32().unwrap(), 0x1234_5678);
        assert_eq!(cursor.read_i16().unwrap(), -1234);
        assert_eq!(cursor.read_f32().unwrap(), 3.5);
    }

    #[test]
    fn read_write_endian_roundtrip() {
        let mut cursor = Cursor::new(Vec::new());

        cursor.write_u32_endian(0x0102_0304, Endian::Big).unwrap();
        cursor.write_u16_endian(0x0506, Endian::Little).unwrap();

        cursor.set_position(0);

        assert_eq!(cursor.read_u32_endian(Endian::Big).unwrap(), 0x0102_0304);
        assert_eq!(cursor.read_u16_endian(Endian::Little).unwrap(), 0x0506);
    }
}
