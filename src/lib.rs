use std::io;

use io::Write;

use arrow_array::Array;
use arrow_array::ArrayRef;
use arrow_array::cast::AsArray;

use arrow_array::Float64Array;
use arrow_array::Int64Array;
use arrow_array::UInt64Array;

pub const NULL_BYTES: [u8; 4] = u32::to_be_bytes(0xffff_ffff);

pub trait QWordLike: Array {
    fn to_qword(&self, trusted_index: usize) -> [u8; 8];

    fn to_writer<W>(&self, trusted_index: usize, wtr: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        if self.is_null(trusted_index) {
            wtr.write_all(&NULL_BYTES)?;
            return Ok(());
        }

        let bytes: [u8; 8] = self.to_qword(trusted_index);

        // Buffer layout:
        // 3 bytes padding/header,
        // 1 byte for length (8),
        // 8 bytes for value
        // Total = 12 bytes
        let mut buf: [u8; 12] = [0; 12];
        buf[3] = 8;
        buf[4..12].copy_from_slice(&bytes);

        wtr.write_all(&buf)
    }
}

impl QWordLike for &Float64Array {
    fn to_qword(&self, trusted_index: usize) -> [u8; 8] {
        self.value(trusted_index).to_be_bytes()
    }
}

impl QWordLike for &Int64Array {
    fn to_qword(&self, trusted_index: usize) -> [u8; 8] {
        self.value(trusted_index).to_be_bytes()
    }
}

impl QWordLike for &UInt64Array {
    fn to_qword(&self, trusted_index: usize) -> [u8; 8] {
        self.value(trusted_index).to_be_bytes()
    }
}

pub fn ref2int64(r: &ArrayRef) -> Option<&Int64Array> {
    r.as_primitive_opt()
}

pub fn ref2uint64(r: &ArrayRef) -> Option<&UInt64Array> {
    r.as_primitive_opt()
}

pub fn ref2float64(r: &ArrayRef) -> Option<&Float64Array> {
    r.as_primitive_opt()
}
