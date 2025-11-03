use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

use crate::{Error, Result};

pub trait ReadFromBytes: Sized {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self>;
}

pub fn read_value<T: ReadFromBytes>(bytes: &[u8], offset: &mut usize) -> Result<T> {
    if *offset >= bytes.len() {
        return Err(Error::IndexOutOfRange);
    }

    let remaining = &bytes[*offset..];
    let mut cursor = Cursor::new(remaining);
    let value = T::read_from(&mut cursor)?;
    let position = cursor.position() as usize;
    *offset = (*offset)
        .checked_add(position)
        .ok_or(Error::IndexOutOfRange)?;
    Ok(value)
}

pub fn read_slice<'a>(bytes: &'a [u8], offset: &mut usize, len: usize) -> Result<&'a [u8]> {
    if *offset >= bytes.len() {
        return Err(Error::IndexOutOfRange);
    }

    let end = (*offset).checked_add(len).ok_or(Error::IndexOutOfRange)?;

    if end > bytes.len() {
        return Err(Error::NotEnoughData);
    }

    let slice = &bytes[*offset..end];
    *offset = end;
    Ok(slice)
}

// Unsigned integers
impl ReadFromBytes for u8 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor.read_u8().map_err(Error::from_std_error)
    }
}

impl ReadFromBytes for u16 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor
            .read_u16::<BigEndian>()
            .map_err(Error::from_std_error)
    }
}

impl ReadFromBytes for u32 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor
            .read_u32::<BigEndian>()
            .map_err(Error::from_std_error)
    }
}

impl ReadFromBytes for u64 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor
            .read_u64::<BigEndian>()
            .map_err(Error::from_std_error)
    }
}

impl ReadFromBytes for u128 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor
            .read_u128::<BigEndian>()
            .map_err(Error::from_std_error)
    }
}

// Signed integers
impl ReadFromBytes for i8 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor.read_i8().map_err(Error::from_std_error)
    }
}

impl ReadFromBytes for i16 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor
            .read_i16::<BigEndian>()
            .map_err(Error::from_std_error)
    }
}

impl ReadFromBytes for i32 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor
            .read_i32::<BigEndian>()
            .map_err(Error::from_std_error)
    }
}

impl ReadFromBytes for i64 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor
            .read_i64::<BigEndian>()
            .map_err(Error::from_std_error)
    }
}

impl ReadFromBytes for i128 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor
            .read_i128::<BigEndian>()
            .map_err(Error::from_std_error)
    }
}

// Floating point
impl ReadFromBytes for f32 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor
            .read_f32::<BigEndian>()
            .map_err(Error::from_std_error)
    }
}

impl ReadFromBytes for f64 {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        cursor
            .read_f64::<BigEndian>()
            .map_err(Error::from_std_error)
    }
}
