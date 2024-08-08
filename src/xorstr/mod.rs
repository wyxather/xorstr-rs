mod time;

use core::{
    arch::x86_64::{__m128i, _mm_load_si128, _mm_store_si128, _mm_xor_si128},
    ptr::read_volatile,
    slice::from_raw_parts,
    str::{from_utf8, Utf8Error},
};
use time::__TIME__;

pub const fn buffer_size(size: usize) -> usize {
    let mut value = size / 16;
    if size % 16 != 0 {
        value += 1;
    }
    value * 2
}

const fn key4(seed: u32) -> u32 {
    const BYTES: &[u8] = __TIME__.as_bytes();
    let mut value = seed;
    let mut i = 0;
    while i < BYTES.len() {
        value ^= BYTES[i] as u32;
        value = value.wrapping_mul(16777619);
        i += 1;
    }
    value
}

const fn key8(seed: u64) -> u64 {
    let first_part = key4(2166136261 + seed as u32);
    let second_part = key4(first_part);
    ((first_part as u64) << 32) | (second_part as u64)
}

const fn load_xored_str8(str: &str, index: usize) -> u64 {
    let bytes = str.as_bytes();
    let mut value = key8(index as u64);
    let mut i = 0;
    let mut j = i + index * 8;
    while i < 8 && j < bytes.len() {
        value ^= (bytes[j] as u64) << (((i % 8) * 8) as u32);
        i += 1;
        j = i + index * 8;
    }
    value
}

fn load_from_reg(value: u64) -> u64 {
    let reg = value;
    unsafe { read_volatile(&reg) }
}

#[repr(C, align(16))]
pub struct XorKey<const SIZE: usize> {
    value: [u64; SIZE],
}

impl<const SIZE: usize> XorKey<SIZE> {
    pub fn new() -> Self {
        let mut instance = Self { value: [0; SIZE] };
        let mut i = 0;
        while i < SIZE {
            instance.value[i] = load_from_reg(key8(i as u64));
            i += 1;
        }
        instance
    }
}

#[repr(C, align(16))]
pub struct XorStr<const SIZE: usize> {
    value: [u64; SIZE],
}

impl<const SIZE: usize> XorStr<SIZE> {
    pub fn new(str: &str, key: XorKey<SIZE>) -> Self {
        let mut instance = Self { value: [0; SIZE] };
        let mut i = 0;
        while i < SIZE {
            instance.value[i] = load_from_reg(load_xored_str8(str, i));
            i += 1;
        }
        let key = key.value.as_ptr() as *const __m128i;
        let value = instance.value.as_mut_ptr() as *mut __m128i;
        i = 0;
        while i < size_of::<Self>() / size_of::<__m128i>() {
            unsafe {
                let key_chunk = key.add(i);
                let value_chunk = value.add(i);
                _mm_store_si128(
                    value_chunk,
                    _mm_xor_si128(_mm_load_si128(value_chunk), _mm_load_si128(key_chunk)),
                );
            };
            i += 1;
        }
        instance
    }

    fn clear(self: &mut Self) {
        self.value = [0; SIZE];
    }

    pub const fn as_slice<T>(self: &Self) -> &[T] {
        unsafe { from_raw_parts(self.value.as_ptr() as *const T, size_of::<u64>() * SIZE) }
    }

    pub const fn as_str(self: &Self) -> Result<&str, Utf8Error> {
        from_utf8(self.as_slice::<u8>())
    }

    pub fn print(self: &mut Self) {
        print!("\n[xorstr]\nbytes=");
        for i in &self.value {
            print!("{}({:?}), ", i, i as *const u64);
        }
        print!("\naddress={:?}\nvalue=", self.value.as_ptr());
        for byte in self.as_slice::<u8>() {
            print!("{}", *byte as char);
        }
        println!();
    }
}

impl<const SIZE: usize> Drop for XorStr<SIZE> {
    fn drop(self: &mut Self) {
        self.clear();
    }
}

#[macro_export]
macro_rules! xorstr {
    ($str:expr) => {{
        const STR: &str = $str;
        const BUFFER_SIZE: usize = $crate::xorstr::buffer_size(STR.len());
        $crate::xorstr::XorStr::<BUFFER_SIZE>::new(
            STR,
            $crate::xorstr::XorKey::<BUFFER_SIZE>::new(),
        )
    }};
}
