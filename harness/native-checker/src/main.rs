//! Native C/Rust differential checker for the calculator's explicit environment.
use std::{ffi::CString, fmt::Write};
include!(concat!(env!("OUT_DIR"), "/calculator.rs"));
#[allow(dead_code)]
#[path = "../../../borg-rs/src/cctype.rs"]
mod cctype;
// The calculator only needs this value record and this single state field.
// Game-dependent value-name resolution is outside the selected kernel.
mod borg_formulas {
    #[derive(Clone, Copy)]
    pub struct ValueSec {
        pub vtype: i32,
        pub index: i32,
    }
}
mod state {
    #[derive(Default)]
    pub struct BorgState {
        pub formulas_calc: crate::borg_formulas_calc::CalcState,
    }
}
use borg_formulas::ValueSec;
use borg_formulas_calc::{CalcEnv, Op, TokenKind};
unsafe extern "C" {
    fn native_clear();
    fn native_snapshot(line: *const std::ffi::c_char, range: i32, out: *mut u8, size: usize)
        -> i32;
}
#[derive(Default)]
struct Env {
    log: String,
}
impl CalcEnv for Env {
    fn parse_value(&mut self, value: &[u8], _: &[u8]) -> Option<ValueSec> {
        let text = std::str::from_utf8(value).ok()?.to_ascii_lowercase();
        let index = text
            .strip_prefix("value(")?
            .strip_suffix(')')?
            .parse::<i32>()
            .ok()?;
        ((0..10).contains(&index) || index == 99).then_some(ValueSec { vtype: 0, index })
    }
    fn calculate_from_value(&mut self, value: &ValueSec, range: i32) -> i32 {
        if value.index == 99 {
            range
        } else {
            value.index * 7 - 10
        }
    }
    fn formula_error(&mut self, section: &[u8], full: &[u8], label: &str, error: &str) {
        writeln!(
            self.log,
            "E [{}][{}][{label}][{error}]",
            String::from_utf8_lossy(section),
            String::from_utf8_lossy(full)
        )
        .unwrap();
    }
    fn note(&mut self, text: &str) {
        writeln!(self.log, "N {text}").unwrap();
    }
}
fn snapshot(st: &mut state::BorgState, expression: &str, range: i32) -> String {
    let mut env = Env::default();
    let calculations = &mut st.formulas_calc.calculations;
    let index = borg_formulas_calc::parse_calculation_line_in(
        &mut env,
        calculations,
        expression.as_bytes(),
        expression.as_bytes(),
    );
    let result = if index < 0 {
        0
    } else {
        borg_formulas_calc::calculate_dynamic_in(&mut env, calculations, index, range)
    };
    writeln!(env.log, "R {index} {result}\nS {}", calculations.len()).unwrap();
    for f in calculations {
        writeln!(env.log, "F {} {}", f.max_depth, f.tokens.len()).unwrap();
        for t in &f.tokens {
            let (kind, a, b) = match t.kind {
                TokenKind::Value(v) => (1, v.vtype, v.index),
                TokenKind::Number(n) => (2, n, 0),
                TokenKind::Op(op) => (
                    match op {
                        Op::Minus => 3,
                        Op::Plus => 4,
                        Op::Mult => 5,
                        Op::Div => 6,
                        Op::And => 7,
                        Op::Or => 8,
                        Op::Equals => 9,
                        Op::Lt => 10,
                        Op::Gt => 11,
                        Op::Le => 12,
                        Op::Ge => 13,
                    },
                    0,
                    0,
                ),
            };
            writeln!(env.log, "T {kind} {} {} {a} {b}", u8::from(t.not), t.pdepth).unwrap();
        }
    }
    env.log
}
struct Rng(u64);
impl Rng {
    fn next(&mut self, n: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % n
    }
    fn expression(&mut self, depth: u32) -> String {
        if depth == 0 || self.next(4) == 0 {
            let value = if self.next(3) == 0 {
                format!(
                    "value({})",
                    if self.next(5) == 0 { 99 } else { self.next(10) }
                )
            } else {
                (self.next(15) as i32 - 7).to_string()
            };
            return if self.next(4) == 0 {
                format!("!{value}")
            } else {
                value
            };
        }
        let left = self.expression(depth - 1);
        let right = self.expression(depth - 1);
        let op = [
            "+", "-", "*", "/", "and", "or", "&&", "||", "=", "<", ">", "<=", ">=",
        ][self.next(13) as usize];
        if self.next(2) == 0 {
            format!("({left} {op} {right})")
        } else {
            format!("{left} {op} {right}")
        }
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let cases = args
        .get(1)
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap_or(30_000);
    let seed = args
        .get(2)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(0x5a17_2026);
    let control = args.iter().any(|s| s == "--fault-control");
    if env!("XCHECK_OVERRIDE") == "1" && !control {
        eprintln!("source override requires --fault-control; never valid parity evidence");
        std::process::exit(2);
    }
    let mut rng = Rng(seed);
    let mut st = state::BorgState::default();
    let mut buffer = vec![0u8; 65536];
    for i in 0..cases {
        if i % 47 == 0 {
            // SAFETY: the C oracle owns its independent single-threaded state.
            unsafe { native_clear() };
            borg_formulas_calc::calculations_free(&mut st);
        }
        let mut expression = rng.expression(2);
        for _ in 0..rng.next(5) {
            expression = format!("({expression})");
        }
        let range = rng.next(47) as i32 - 23;
        let c_string = CString::new(expression.as_bytes()).unwrap();
        // SAFETY: terminated input and writable output live for the complete C call;
        // the bounded C formatter aborts rather than overrun this buffer.
        let size =
            unsafe { native_snapshot(c_string.as_ptr(), range, buffer.as_mut_ptr(), buffer.len()) };
        assert!(size >= 0 && (size as usize) < buffer.len());
        let native = std::str::from_utf8(&buffer[..size as usize]).unwrap();
        let rust = snapshot(&mut st, &expression, range);
        if native != rust {
            eprintln!("MISMATCH case={i} seed={seed} range={range} expression={expression:?}\nC:\n{native}\nRust:\n{rust}");
            std::process::exit(1);
        }
    }
    if env!("XCHECK_OVERRIDE") == "1" {
        eprintln!("fault source unexpectedly matched; not valid evidence");
        std::process::exit(2);
    }
    println!("PASS {cases} native C/Rust cases, seed={seed}; result, diagnostics and complete calculation/token state identical");
    println!("Rust source: {}", env!("XCHECK_RUST_SOURCE"));
}
