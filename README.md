# xorstr-rs

## Overview

`xorstr-rs` is a Rust port of the [xorstr](https://github.com/JustasMasiulis/xorstr) library, implemented in C++ by JustasMasiulis. This library provides a way to obfuscate strings in your Rust binaries at compile time.

## Usage / Example
```
mod xorstr;

fn main() {
    let xorstr = xorstr!("hello world from earth");
    println!("{}", xorstr.as_str().unwrap());
}
```

## Dissambler Output
```
__int64 sub_140001170()
{
  __int128 v1[2]; // [rsp+50h] [rbp-78h] BYREF
  __int128 v2; // [rsp+70h] [rbp-58h] BYREF
  void *v3; // [rsp+88h] [rbp-40h] BYREF
  __int128 v4; // [rsp+90h] [rbp-38h]
  __int64 v5; // [rsp+A0h] [rbp-28h]
  __int64 v6; // [rsp+A8h] [rbp-20h]
  __int64 v7[2]; // [rsp+B8h] [rbp-10h] BYREF

  v1[0] = (__int128)_mm_xor_ps(
                      _mm_movelh_ps((__m128)0xC52E84D49A88D251ui64, (__m128)0x5C8A84AAD2364D78ui64),
                      _mm_movelh_ps((__m128)0xAA59A4BBF6E4B739ui64, (__m128)0x31E5F6CCF252210Aui64));
  v1[1] = (__int128)_mm_xor_ps(
                      _mm_movelh_ps((__m128)0x84FE05F9012E5C73ui64, (__m128)0x73AE033EDA639ED4ui64),
                      _mm_movelh_ps((__m128)0x84FE6D8D734F3953ui64, (__m128)0x73AE033EDA639ED4ui64));
  core::str::converts::from_utf8::h3eb536f177e351b4(&v3, v1, 32i64);
  if ( v3 )
  {
    v2 = v4;
    core::result::unwrap_failed::h77060c222a1dc3ba(
      (unsigned int)"called `Result::unwrap()` on an `Err` value",
      43,
      (unsigned int)&v2,
      (unsigned int)&unk_14001A3C0,
      (__int64)&off_14001A480);
  }
  v2 = v4;
  v7[0] = (__int64)&v2;
  v7[1] = (__int64)sub_1400010B0;
  v3 = &unk_14001A4A0;
  *(_QWORD *)&v4 = 2i64;
  v6 = 0i64;
  *((_QWORD *)&v4 + 1) = v7;
  v5 = 1i64;
  return std::io::stdio::_print::hfb1cb878ecb43904(&v3);
}
```
