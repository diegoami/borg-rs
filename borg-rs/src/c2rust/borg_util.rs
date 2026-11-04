pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub use crate::headers::__stddef_size_t_h::size_t;
pub use crate::headers::ctype_h::{
    _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower, _ISprint, _ISpunct,
    _ISspace, _ISupper, _ISxdigit, __ctype_b_loc, C2RustUnnamedH_37,
};

pub use crate::headers::stdint_intn_h::int16_t;
use crate::headers::string_h::{strcmp, strlen};
pub use crate::headers::types_h::__int16_t;
#[no_mangle]
pub static mut borg_sort_comp: Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> bool,
> = None;
#[no_mangle]
pub static mut borg_sort_swap: Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
> = None;
unsafe extern "C" fn borg_sort_aux(
    mut u: *mut ::core::ffi::c_void,
    mut v: *mut ::core::ffi::c_void,
    mut p: ::core::ffi::c_int,
    mut q: ::core::ffi::c_int,
) {
    let mut z: ::core::ffi::c_int = 0;
    let mut a: ::core::ffi::c_int = 0;
    let mut b: ::core::ffi::c_int = 0;
    if p >= q {
        return;
    }
    z = p;
    a = p;
    b = q;
    loop {
        while !Some(borg_sort_comp.expect("non-null function pointer"))
            .expect("non-null function pointer")(u, v, b, z)
        {
            b -= 1;
        }
        while !Some(borg_sort_comp.expect("non-null function pointer"))
            .expect("non-null function pointer")(u, v, z, a)
        {
            a += 1;
        }
        if a >= b {
            break;
        }
        Some(borg_sort_swap.expect("non-null function pointer"))
            .expect("non-null function pointer")(u, v, a, b);
        a += 1;
        b -= 1;
    }
    borg_sort_aux(u, v, p, b);
    borg_sort_aux(u, v, b + 1 as ::core::ffi::c_int, q);
}
#[no_mangle]
pub unsafe extern "C" fn borg_sort(
    mut u: *mut ::core::ffi::c_void,
    mut v: *mut ::core::ffi::c_void,
    mut n: ::core::ffi::c_int,
) {
    borg_sort_aux(u, v, 0 as ::core::ffi::c_int, n - 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn borg_sort_comp_hook(
    mut u: *mut ::core::ffi::c_void,
    mut v: *mut ::core::ffi::c_void,
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> bool {
    let mut text: *mut *mut ::core::ffi::c_char = u as *mut *mut ::core::ffi::c_char;
    let mut what: *mut int16_t = v as *mut int16_t;
    let mut cmp: ::core::ffi::c_int = 0;
    cmp = strcmp(*text.offset(a as isize), *text.offset(b as isize));
    if cmp < 0 as ::core::ffi::c_int {
        return true_0 != 0;
    }
    if cmp > 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    return *what.offset(a as isize) as ::core::ffi::c_int
        <= *what.offset(b as isize) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn borg_sort_swap_hook(
    mut u: *mut ::core::ffi::c_void,
    mut v: *mut ::core::ffi::c_void,
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) {
    let mut text: *mut *mut ::core::ffi::c_char = u as *mut *mut ::core::ffi::c_char;
    let mut what: *mut int16_t = v as *mut int16_t;
    let mut texttmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut whattmp: int16_t = 0;
    texttmp = *text.offset(a as isize);
    let ref mut fresh0 = *text.offset(a as isize);
    *fresh0 = *text.offset(b as isize);
    let ref mut fresh1 = *text.offset(b as isize);
    *fresh1 = texttmp;
    whattmp = *what.offset(a as isize);
    *what.offset(a as isize) = *what.offset(b as isize);
    *what.offset(b as isize) = whattmp;
}
#[no_mangle]
pub unsafe extern "C" fn borg_trim(mut line: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    while *(*__ctype_b_loc()).offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        line = line.offset(1);
    }
    if *line as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return line;
    }
    let mut end: *mut ::core::ffi::c_char = line
        .offset(strlen(line) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    while end > line
        && *(*__ctype_b_loc()).offset(*end as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
    {
        end = end.offset(-1);
    }
    *end.offset(1 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    return line;
}
