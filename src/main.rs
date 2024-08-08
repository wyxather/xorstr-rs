use xorstr::XorStr;

mod xorstr;

#[inline(never)]
fn foo1() -> *mut XorStr<4> {
    let mut xorstr = xorstr!("hello world from earth");
    xorstr.print();
    &mut xorstr as *mut XorStr<4>
}

#[inline(never)]
fn foo2(xorstr: *mut XorStr<4>) {
    unsafe { (*xorstr).print() };
}

#[inline(never)]
fn foo3() {
    let mut xorstr = xorstr!("hello world from earth");
    xorstr.print();
}

fn main() {
    foo2(foo1());
    foo3();
}
