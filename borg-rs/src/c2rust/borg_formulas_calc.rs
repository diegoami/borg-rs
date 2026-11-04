extern "C" {
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn atol(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_long;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn mem_alloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_zalloc(len: size_t) -> *mut ::core::ffi::c_void;
    fn mem_free(p: *mut ::core::ffi::c_void);
    fn string_make(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn prefix_i(s: *const ::core::ffi::c_char, t: *const ::core::ffi::c_char) -> bool;
    fn borg_array_add(a: *mut borg_array, item: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn borg_formula_error(
        section: *const ::core::ffi::c_char,
        full_line: *const ::core::ffi::c_char,
        section_label: *const ::core::ffi::c_char,
        error: *const ::core::ffi::c_char,
    );
    fn calculate_from_value(value: *mut value_sec, range_index: ::core::ffi::c_int) -> int32_t;
    fn parse_value(
        line: *mut ::core::ffi::c_char,
        full_line: *const ::core::ffi::c_char,
    ) -> *mut value_sec;
    fn borg_note(what: *const ::core::ffi::c_char);
}
pub type __int32_t = i32;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type int32_t = __int32_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_calculation {
    pub token_array: *mut borg_array,
    pub max_depth: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct borg_array {
    pub max: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub items: *mut *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct token {
    pub type_0: token_type,
    pub not: bool,
    pub token: *mut ::core::ffi::c_void,
    pub pdepth: int32_t,
}
pub type token_type = ::core::ffi::c_uint;
pub const TOK_GE: token_type = 13;
pub const TOK_LE: token_type = 12;
pub const TOK_GT: token_type = 11;
pub const TOK_LT: token_type = 10;
pub const TOK_EQUALS: token_type = 9;
pub const TOK_OR: token_type = 8;
pub const TOK_AND: token_type = 7;
pub const TOK_DIV: token_type = 6;
pub const TOK_MULT: token_type = 5;
pub const TOK_PLUS: token_type = 4;
pub const TOK_MINUS: token_type = 3;
pub const TOK_NUMBER: token_type = 2;
pub const TOK_VALUE: token_type = 1;
pub const TOK_NONE: token_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct operator_struct {
    pub type_0: token_type,
    pub level: ::core::ffi::c_int,
    pub position: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct value_sec {
    pub type_0: value_type,
    pub index: int32_t,
}
pub type value_type = ::core::ffi::c_int;
pub const VT_MAX: value_type = 41;
pub const VT_GOLD: value_type = 40;
pub const VT_OTHER_BOOK: value_type = 39;
pub const VT_SHADOW_BOOK: value_type = 38;
pub const VT_NATURE_BOOK: value_type = 37;
pub const VT_PRAYER_BOOK: value_type = 36;
pub const VT_MAGIC_BOOK: value_type = 35;
pub const VT_MUSHROOM: value_type = 34;
pub const VT_FOOD: value_type = 33;
pub const VT_FLASK: value_type = 32;
pub const VT_POTION: value_type = 31;
pub const VT_SCROLL: value_type = 30;
pub const VT_ROD: value_type = 29;
pub const VT_WAND: value_type = 28;
pub const VT_STAFF: value_type = 27;
pub const VT_RING: value_type = 26;
pub const VT_AMULET: value_type = 25;
pub const VT_LIGHT: value_type = 24;
pub const VT_DRAG_ARMOR: value_type = 23;
pub const VT_HARD_ARMOR: value_type = 22;
pub const VT_SOFT_ARMOR: value_type = 21;
pub const VT_CLOAK: value_type = 20;
pub const VT_SHIELD: value_type = 19;
pub const VT_CROWN: value_type = 18;
pub const VT_HELM: value_type = 17;
pub const VT_GLOVES: value_type = 16;
pub const VT_BOOTS: value_type = 15;
pub const VT_SWORD: value_type = 14;
pub const VT_POLEARM: value_type = 13;
pub const VT_HAFTED: value_type = 12;
pub const VT_DIGGING: value_type = 11;
pub const VT_BOW: value_type = 10;
pub const VT_BOLT: value_type = 9;
pub const VT_ARROW: value_type = 8;
pub const VT_SHOT: value_type = 7;
pub const VT_CHEST: value_type = 6;
pub const VT_NULL: value_type = 5;
pub const VT_CLASS: value_type = 4;
pub const VT_ACTIVATION: value_type = 3;
pub const VT_CONFIG: value_type = 2;
pub const VT_TRAIT: value_type = 1;
pub const VT_RANGE_INDEX: value_type = 0;
pub const VT_NONE: value_type = -1;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub static mut calculations: borg_array = borg_array {
    max: 0,
    count: 0,
    items: ::core::ptr::null::<*mut ::core::ffi::c_void>() as *mut *mut ::core::ffi::c_void,
};
pub const OP_LEVELS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn new_token(
    mut ttype: token_type,
    mut token: *mut ::core::ffi::c_void,
    mut pdepth: ::core::ffi::c_int,
    mut not: bool,
) -> *mut token {
    let mut tok: *mut token = mem_zalloc(::core::mem::size_of::<token>() as size_t) as *mut token;
    (*tok).type_0 = ttype;
    (*tok).token = token;
    (*tok).pdepth = pdepth as int32_t;
    (*tok).not = not;
    return tok;
}
unsafe extern "C" fn get_operator(
    mut in_0: *mut *mut ::core::ffi::c_char,
    mut ttype: *mut token_type,
) -> bool {
    let mut line: *mut ::core::ffi::c_char = *in_0;
    if *line as ::core::ffi::c_int == '+' as i32 {
        *ttype = TOK_PLUS;
        *in_0 = (*in_0).offset(1);
        return true_0 != 0;
    }
    if *line as ::core::ffi::c_int == '-' as i32 {
        if *(*__ctype_b_loc())
            .offset(*line.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
        {
            return false_0 != 0;
        }
        *in_0 = (*in_0).offset(1);
        *ttype = TOK_MINUS;
        return true_0 != 0;
    }
    if *line as ::core::ffi::c_int == '*' as i32 {
        *ttype = TOK_MULT;
        *in_0 = (*in_0).offset(1);
        return true_0 != 0;
    }
    if *line as ::core::ffi::c_int == '/' as i32 {
        *ttype = TOK_DIV;
        *in_0 = (*in_0).offset(1);
        return true_0 != 0;
    }
    if prefix_i(line, b"and\0" as *const u8 as *const ::core::ffi::c_char) {
        *ttype = TOK_AND;
        *in_0 = (*in_0).offset(3 as ::core::ffi::c_int as isize);
        return true_0 != 0;
    }
    if prefix_i(line, b"&&\0" as *const u8 as *const ::core::ffi::c_char) {
        *ttype = TOK_AND;
        *in_0 = (*in_0).offset(2 as ::core::ffi::c_int as isize);
        return true_0 != 0;
    }
    if prefix_i(line, b"or\0" as *const u8 as *const ::core::ffi::c_char) {
        *ttype = TOK_OR;
        *in_0 = (*in_0).offset(2 as ::core::ffi::c_int as isize);
        return true_0 != 0;
    }
    if prefix_i(line, b"||\0" as *const u8 as *const ::core::ffi::c_char) {
        *ttype = TOK_OR;
        *in_0 = (*in_0).offset(2 as ::core::ffi::c_int as isize);
        return true_0 != 0;
    }
    if *line as ::core::ffi::c_int == '=' as i32 {
        *ttype = TOK_EQUALS;
        *in_0 = (*in_0).offset(1);
        return true_0 != 0;
    }
    if *line as ::core::ffi::c_int == '>' as i32 {
        *ttype = TOK_GT;
        *in_0 = (*in_0).offset(1);
        if *line.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '=' as i32 {
            *ttype = TOK_GE;
            *in_0 = (*in_0).offset(1);
        }
        return true_0 != 0;
    }
    if *line as ::core::ffi::c_int == '<' as i32 {
        *ttype = TOK_LT;
        *in_0 = (*in_0).offset(1);
        if *line.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '=' as i32 {
            *ttype = TOK_LE;
            *in_0 = (*in_0).offset(1);
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn get_value_string(
    mut line: *mut ::core::ffi::c_char,
    mut value: *mut *mut ::core::ffi::c_char,
) -> bool {
    if prefix_i(line, b"value\0" as *const u8 as *const ::core::ffi::c_char) {
        let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut len: ::core::ffi::c_int = 0;
        let mut pos: *mut ::core::ffi::c_char = line;
        line = strchr(line, ')' as i32);
        if !line.is_null() {
            len = (line.offset_from(pos) as ::core::ffi::c_long + 1 as ::core::ffi::c_long)
                as ::core::ffi::c_int;
        } else {
            len = strlen(pos) as ::core::ffi::c_int;
        }
        strncpy(&raw mut buf as *mut ::core::ffi::c_char, pos, len as size_t);
        buf[len as usize] = 0 as ::core::ffi::c_char;
        *value = string_make(&raw mut buf as *mut ::core::ffi::c_char);
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn tokenize_math(
    mut f: *mut borg_calculation,
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> bool {
    let mut fail: bool = false_0 != 0;
    let mut negate: bool = false_0 != 0;
    let mut pdepth: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut start: *mut ::core::ffi::c_char = line;
    let mut type_0: token_type = TOK_NONE;
    let mut value_token: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*f).token_array =
        mem_zalloc(::core::mem::size_of::<borg_array>() as size_t) as *mut borg_array;
    while *line != 0 {
        if get_value_string(line, &raw mut value_token) {
            line = line.offset(strlen(value_token) as isize);
            let mut v: *mut value_sec = parse_value(value_token, full_line);
            if v.is_null() {
                fail = true_0 != 0;
            }
            mem_free(value_token as *mut ::core::ffi::c_void);
            value_token = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if fail {
                mem_free(v as *mut ::core::ffi::c_void);
                v = ::core::ptr::null_mut::<value_sec>();
                break;
            } else {
                if pdepth as int32_t > (*f).max_depth {
                    (*f).max_depth = pdepth as int32_t;
                }
                borg_array_add(
                    (*f).token_array,
                    new_token(TOK_VALUE, v as *mut ::core::ffi::c_void, pdepth, negate)
                        as *mut ::core::ffi::c_void,
                );
                negate = false_0 != 0;
            }
        } else if *line as ::core::ffi::c_int == '!' as i32 {
            negate = true_0 != 0;
            line = line.offset(1);
        } else if *line as ::core::ffi::c_int == '(' as i32 {
            pdepth += 1;
            line = line.offset(1);
        } else if *line as ::core::ffi::c_int == ')' as i32 {
            pdepth -= 1;
            if pdepth < 0 as ::core::ffi::c_int {
                borg_formula_error(
                    start,
                    full_line,
                    b"calculation\0" as *const u8 as *const ::core::ffi::c_char,
                    b"** unmatched parenthesis\0" as *const u8 as *const ::core::ffi::c_char,
                );
                fail = true_0 != 0;
                break;
            } else {
                line = line.offset(1);
            }
        } else if *(*__ctype_b_loc())
            .offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
            || *line as ::core::ffi::c_int == '-' as i32
                && *(*__ctype_b_loc()).offset(*line.offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_uchar
                    as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                    != 0
        {
            let mut val_start: *mut ::core::ffi::c_char = line;
            let mut val: *mut int32_t =
                mem_alloc(::core::mem::size_of::<int32_t>() as size_t) as *mut int32_t;
            *val = atol(line) as int32_t;
            if pdepth as int32_t > (*f).max_depth {
                (*f).max_depth = pdepth as int32_t;
            }
            borg_array_add(
                (*f).token_array,
                new_token(TOK_NUMBER, val as *mut ::core::ffi::c_void, pdepth, negate)
                    as *mut ::core::ffi::c_void,
            );
            negate = false_0 != 0;
            while *(*__ctype_b_loc())
                .offset(*line as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
                || *line as ::core::ffi::c_int == '-' as i32 && line == val_start
            {
                line = line.offset(1);
            }
            if *line as ::core::ffi::c_int == '-' as i32 && line != val_start {
                borg_formula_error(
                    start,
                    full_line,
                    b"calculation\0" as *const u8 as *const ::core::ffi::c_char,
                    b"** number parsing error\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        } else if get_operator(&raw mut line, &raw mut type_0) {
            if pdepth as int32_t > (*f).max_depth {
                (*f).max_depth = pdepth as int32_t;
            }
            borg_array_add(
                (*f).token_array,
                new_token(type_0, NULL, pdepth, false_0 != 0) as *mut ::core::ffi::c_void,
            );
            if negate {
                borg_formula_error(
                    start,
                    full_line,
                    b"calculation\0" as *const u8 as *const ::core::ffi::c_char,
                    b"** not (!) can only be applied to values\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                fail = true_0 != 0;
            }
        } else {
            line = line.offset(1);
        }
    }
    if pdepth != 0 && !fail {
        borg_formula_error(
            start,
            full_line,
            b"calculation\0" as *const u8 as *const ::core::ffi::c_char,
            b"** unmatched parenthesis\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fail = true_0 != 0;
    }
    return fail;
}
unsafe extern "C" fn is_operator(mut type_0: token_type) -> bool {
    return type_0 as ::core::ffi::c_uint >= TOK_MINUS as ::core::ffi::c_int as ::core::ffi::c_uint;
}
unsafe extern "C" fn operation_level(mut type_0: token_type) -> ::core::ffi::c_int {
    if type_0 as ::core::ffi::c_uint == TOK_DIV as ::core::ffi::c_int as ::core::ffi::c_uint
        || type_0 as ::core::ffi::c_uint == TOK_MULT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if type_0 as ::core::ffi::c_uint == TOK_PLUS as ::core::ffi::c_int as ::core::ffi::c_uint
        || type_0 as ::core::ffi::c_uint == TOK_MINUS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 1 as ::core::ffi::c_int;
    }
    if type_0 as ::core::ffi::c_uint == TOK_AND as ::core::ffi::c_int as ::core::ffi::c_uint
        || type_0 as ::core::ffi::c_uint == TOK_OR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 3 as ::core::ffi::c_int;
    }
    return 2 as ::core::ffi::c_int;
}
unsafe extern "C" fn add_depth(mut f: *mut borg_calculation, mut pos: ::core::ffi::c_int) {
    let mut tok: *mut token = *(*(*f).token_array).items.offset(pos as isize) as *mut token;
    let mut pdepth: ::core::ffi::c_int = (*tok).pdepth as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    (*tok).pdepth += 1;
    i = pos - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        tok = *(*(*f).token_array).items.offset(i as isize) as *mut token;
        if (*tok).pdepth < pdepth as int32_t
            || is_operator((*tok).type_0) as ::core::ffi::c_int != 0
                && (*tok).pdepth == pdepth as int32_t
        {
            break;
        }
        (*tok).pdepth += 1;
        if (*tok).pdepth > (*f).max_depth {
            (*f).max_depth = (*tok).pdepth;
        }
        i -= 1;
    }
    let mut end: ::core::ffi::c_int = (*(*f).token_array).count;
    i = pos + 1 as ::core::ffi::c_int;
    while i < end {
        tok = *(*(*f).token_array).items.offset(i as isize) as *mut token;
        if (*tok).pdepth < pdepth as int32_t
            || is_operator((*tok).type_0) as ::core::ffi::c_int != 0
                && (*tok).pdepth == pdepth as int32_t
        {
            break;
        }
        (*tok).pdepth += 1;
        if (*tok).pdepth > (*f).max_depth {
            (*f).max_depth = (*tok).pdepth;
        }
        i += 1;
    }
}
unsafe extern "C" fn adjust_order_ops_depth_block(
    mut f: *mut borg_calculation,
    mut pdepth: ::core::ffi::c_int,
    mut start: ::core::ffi::c_int,
    mut end: ::core::ffi::c_int,
) -> bool {
    let mut operators: *mut borg_array =
        mem_zalloc(::core::mem::size_of::<borg_array>() as size_t) as *mut borg_array;
    let mut o: *mut operator_struct = ::core::ptr::null_mut::<operator_struct>();
    let mut tok: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut lvl: ::core::ffi::c_int = 0;
    let mut l: [::core::ffi::c_int; 4] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    tok = start;
    while tok < end {
        let mut token: *mut token = *(*(*f).token_array).items.offset(tok as isize) as *mut token;
        if !((*token).pdepth != pdepth as int32_t) {
            if is_operator((*token).type_0) {
                o = mem_alloc(::core::mem::size_of::<operator_struct>() as size_t)
                    as *mut operator_struct;
                (*o).type_0 = (*token).type_0;
                (*o).level = operation_level((*token).type_0);
                (*o).position = tok;
                l[(*o).level as usize] += 1;
                borg_array_add(operators, o as *mut ::core::ffi::c_void);
            }
        }
        tok += 1;
    }
    let mut higher: bool = false;
    if (*operators).count > 1 as ::core::ffi::c_int {
        let mut count_at_level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        lvl = 0 as ::core::ffi::c_int;
        while lvl < OP_LEVELS {
            if l[lvl as usize] != 0 {
                i = 0 as ::core::ffi::c_int;
                while i < (*operators).count {
                    o = *(*operators).items.offset(i as isize) as *mut operator_struct;
                    if (*o).level == lvl {
                        higher = false_0 != 0;
                        j = lvl + 1 as ::core::ffi::c_int;
                        while j < OP_LEVELS {
                            if l[j as usize] != 0 {
                                higher = true_0 != 0;
                                break;
                            } else {
                                j += 1;
                            }
                        }
                        if higher as ::core::ffi::c_int != 0
                            || count_at_level < (*operators).count - 1 as ::core::ffi::c_int
                        {
                            add_depth(f, (*o).position);
                            count_at_level += 1;
                        }
                    }
                    i += 1;
                }
            }
            lvl += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*operators).count {
        mem_free(*(*operators).items.offset(i as isize));
        let ref mut fresh1 = *(*operators).items.offset(i as isize);
        *fresh1 = NULL;
        i += 1;
    }
    mem_free(operators as *mut ::core::ffi::c_void);
    operators = ::core::ptr::null_mut::<borg_array>();
    return false_0 != 0;
}
unsafe extern "C" fn adjust_order_operations(mut f: *mut borg_calculation) -> bool {
    let mut fail: bool = false_0 != 0;
    let mut pdepth: ::core::ffi::c_int = 0;
    let mut itoken: ::core::ffi::c_int = 0;
    let mut start: ::core::ffi::c_int = 0;
    pdepth = 0 as ::core::ffi::c_int;
    while pdepth as int32_t <= (*f).max_depth {
        start = -(1 as ::core::ffi::c_int);
        itoken = 0 as ::core::ffi::c_int;
        while itoken < (*(*f).token_array).count {
            let mut tok: *mut token =
                *(*(*f).token_array).items.offset(itoken as isize) as *mut token;
            if start == -(1 as ::core::ffi::c_int) && (*tok).pdepth == pdepth as int32_t {
                start = itoken;
            } else if start != -(1 as ::core::ffi::c_int) && (*tok).pdepth < pdepth as int32_t {
                fail = adjust_order_ops_depth_block(
                    f,
                    pdepth,
                    start,
                    itoken - 1 as ::core::ffi::c_int,
                );
                start = -(1 as ::core::ffi::c_int);
            }
            itoken += 1;
        }
        if start != -(1 as ::core::ffi::c_int) {
            fail = adjust_order_ops_depth_block(f, pdepth, start, itoken - 1 as ::core::ffi::c_int);
            start = -(1 as ::core::ffi::c_int);
        }
        pdepth += 1;
    }
    return fail;
}
unsafe extern "C" fn validate_calculation(
    mut f: *mut borg_calculation,
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> bool {
    if (*(*f).token_array).count == 0 {
        borg_formula_error(
            line,
            full_line,
            b"calculation\0" as *const u8 as *const ::core::ffi::c_char,
            b"** formula must contain values\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return true_0 != 0;
    }
    let mut ntoken: ::core::ffi::c_int = 0;
    let mut tok: *mut token = ::core::ptr::null_mut::<token>();
    ntoken = 0 as ::core::ffi::c_int;
    while ntoken < (*(*f).token_array).count {
        tok = *(*(*f).token_array).items.offset(ntoken as isize) as *mut token;
        if ntoken % 2 as ::core::ffi::c_int != 0 {
            if !is_operator((*tok).type_0) {
                borg_formula_error(
                    line,
                    full_line,
                    b"calculation\0" as *const u8 as *const ::core::ffi::c_char,
                    b"** formula must be values separated by operators\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
        }
        if ntoken % 2 as ::core::ffi::c_int == 0 {
            if is_operator((*tok).type_0) {
                borg_formula_error(
                    line,
                    full_line,
                    b"calculation\0" as *const u8 as *const ::core::ffi::c_char,
                    b"** formula must be values separated by operators\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return true_0 != 0;
            }
        }
        ntoken += 1;
    }
    tok = *(*(*f).token_array)
        .items
        .offset(((*(*f).token_array).count - 1 as ::core::ffi::c_int) as isize)
        as *mut token;
    if is_operator((*tok).type_0) {
        borg_formula_error(
            line,
            full_line,
            b"calculation\0" as *const u8 as *const ::core::ffi::c_char,
            b"** formula can't end in an operation\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn parse_calculation(
    mut f: *mut borg_calculation,
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> bool {
    if tokenize_math(f, line, full_line) {
        return true_0 != 0;
    }
    if adjust_order_operations(f) {
        return true_0 != 0;
    }
    return validate_calculation(f, line, full_line);
}
unsafe extern "C" fn token_free(mut t: *mut token) {
    if !(*t).token.is_null() {
        mem_free((*t).token);
        (*t).token = NULL;
    }
    mem_free(t as *mut ::core::ffi::c_void);
    t = ::core::ptr::null_mut::<token>();
}
unsafe extern "C" fn calc_free(mut calc: *mut borg_calculation) {
    if !(*calc).token_array.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*(*calc).token_array).count {
            token_free(*(*(*calc).token_array).items.offset(i as isize) as *mut token);
            let ref mut fresh0 = *(*(*calc).token_array).items.offset(i as isize);
            *fresh0 = NULL;
            i += 1;
        }
        if !(*calc).token_array.is_null() {
            mem_free((*calc).token_array as *mut ::core::ffi::c_void);
        }
        (*calc).token_array = ::core::ptr::null_mut::<borg_array>();
    }
    mem_free(calc as *mut ::core::ffi::c_void);
    calc = ::core::ptr::null_mut::<borg_calculation>();
}
#[no_mangle]
pub unsafe extern "C" fn parse_calculation_line(
    mut line: *mut ::core::ffi::c_char,
    mut full_line: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut f: *mut borg_calculation =
        mem_zalloc(::core::mem::size_of::<borg_calculation>() as size_t) as *mut borg_calculation;
    if parse_calculation(f, line, full_line) {
        calc_free(f);
        f = ::core::ptr::null_mut::<borg_calculation>();
        return -(1 as ::core::ffi::c_int);
    }
    let mut i: ::core::ffi::c_int =
        borg_array_add(&raw mut calculations, f as *mut ::core::ffi::c_void);
    if i == -(1 as ::core::ffi::c_int) {
        borg_formula_error(
            line,
            full_line,
            b"calculation\0" as *const u8 as *const ::core::ffi::c_char,
            b"** can't add to formula array\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return i;
}
unsafe extern "C" fn calculate_value_from_formula_depth(
    mut f: *mut borg_array,
    mut i: *mut ::core::ffi::c_int,
    mut pdepth: ::core::ffi::c_int,
    mut range_index: ::core::ffi::c_int,
) -> int32_t {
    let mut left_token: *mut token = *(*f).items.offset(*i as isize) as *mut token;
    let mut left_value: int32_t = 0 as int32_t;
    if (*left_token).pdepth > pdepth as int32_t {
        left_value =
            calculate_value_from_formula_depth(f, i, pdepth + 1 as ::core::ffi::c_int, range_index);
    } else if (*left_token).type_0 as ::core::ffi::c_uint
        == TOK_NUMBER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        left_value = *((*left_token).token as *mut int32_t);
        if (*left_token).not {
            left_value = (left_value == 0) as ::core::ffi::c_int as int32_t;
        }
    } else if (*left_token).type_0 as ::core::ffi::c_uint
        == TOK_VALUE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        left_value = calculate_from_value((*left_token).token as *mut value_sec, range_index);
        if (*left_token).not {
            left_value = (left_value == 0) as ::core::ffi::c_int as int32_t;
        }
    }
    if *i + 1 as ::core::ffi::c_int >= (*f).count {
        return left_value;
    }
    let mut operation: *mut token =
        *(*f).items.offset((*i + 1 as ::core::ffi::c_int) as isize) as *mut token;
    if (*operation).pdepth < pdepth as int32_t {
        return left_value;
    }
    *i += 1;
    let mut right_value: int32_t = 0 as int32_t;
    *i += 1;
    let mut right_token: *mut token = *(*f).items.offset(*i as isize) as *mut token;
    if (*right_token).pdepth > pdepth as int32_t {
        right_value =
            calculate_value_from_formula_depth(f, i, pdepth + 1 as ::core::ffi::c_int, range_index);
    } else if (*right_token).type_0 as ::core::ffi::c_uint
        == TOK_NUMBER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        right_value = *((*right_token).token as *mut int32_t);
        if (*right_token).not {
            right_value = (right_value == 0) as ::core::ffi::c_int as int32_t;
        }
    } else if (*right_token).type_0 as ::core::ffi::c_uint
        == TOK_VALUE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        right_value = calculate_from_value((*right_token).token as *mut value_sec, range_index);
        if (*right_token).not {
            right_value = (right_value == 0) as ::core::ffi::c_int as int32_t;
        }
    }
    let mut total: int32_t = 0 as int32_t;
    match (*operation).type_0 as ::core::ffi::c_uint {
        3 => {
            total = left_value - right_value;
        }
        4 => {
            total = left_value + right_value;
        }
        5 => {
            total = left_value * right_value;
        }
        6 => {
            if right_value == 0 as int32_t {
                borg_note(
                    b"** borg formula failure ** \0" as *const u8 as *const ::core::ffi::c_char,
                );
                borg_note(
                    b"** divide by zero error calculating a formula value\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                borg_note(
                    b"** setting denominator to one\0" as *const u8 as *const ::core::ffi::c_char,
                );
                total = left_value;
            } else {
                total = left_value / right_value;
            }
        }
        7 => {
            total = (left_value != 0 && right_value != 0) as ::core::ffi::c_int as int32_t;
        }
        8 => {
            total = (left_value != 0 || right_value != 0) as ::core::ffi::c_int as int32_t;
        }
        9 => {
            total = (left_value == right_value) as ::core::ffi::c_int as int32_t;
        }
        10 => {
            total = (left_value < right_value) as ::core::ffi::c_int as int32_t;
        }
        11 => {
            total = (left_value > right_value) as ::core::ffi::c_int as int32_t;
        }
        12 => {
            total = (left_value <= right_value) as ::core::ffi::c_int as int32_t;
        }
        13 => {
            total = (left_value >= right_value) as ::core::ffi::c_int as int32_t;
        }
        _ => {
            borg_note(b"** borg formula failure ** \0" as *const u8 as *const ::core::ffi::c_char);
            borg_note(
                b"** error calculating a formula value\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if *i + 1 as ::core::ffi::c_int >= (*f).count {
        return total;
    }
    let mut next_token: *mut token =
        *(*f).items.offset((*i + 1 as ::core::ffi::c_int) as isize) as *mut token;
    if (*next_token).pdepth != pdepth as int32_t {
        return total;
    }
    borg_note(b"** borg formula failure ** \0" as *const u8 as *const ::core::ffi::c_char);
    borg_note(b"** error calculating a formula value\0" as *const u8 as *const ::core::ffi::c_char);
    return 0 as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn borg_calculate_dynamic(
    mut formula: ::core::ffi::c_int,
    mut range_index: ::core::ffi::c_int,
) -> int32_t {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut f: *mut borg_calculation =
        *calculations.items.offset(formula as isize) as *mut borg_calculation;
    return calculate_value_from_formula_depth(
        (*f).token_array,
        &raw mut i,
        0 as ::core::ffi::c_int,
        range_index,
    );
}
#[no_mangle]
pub unsafe extern "C" fn calculations_free() {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < calculations.count {
        calc_free(*calculations.items.offset(i as isize) as *mut borg_calculation);
        let ref mut fresh2 = *calculations.items.offset(i as isize);
        *fresh2 = NULL;
        i += 1;
    }
    calculations.count = 0 as ::core::ffi::c_int;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
