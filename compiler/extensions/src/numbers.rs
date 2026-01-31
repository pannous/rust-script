// Math constants and numeric utilities for script mode.
// #[allow(mixed_script_confusables)] can't do on crate level because parser.rs injection

use std::cell::Cell;

#[allow(nonstandard_style)]
#[allow(dead_code)]
pub const tau: f64 = std::f64::consts::TAU;
#[allow(nonstandard_style)]
#[allow(dead_code)]
pub const pi: f64 = std::f64::consts::PI;

#[allow(nonstandard_style)]
#[allow(dead_code)]
pub const τ: f64 = std::f64::consts::TAU;
#[allow(nonstandard_style)]
#[allow(dead_code)]
pub const π: f64 = std::f64::consts::PI;

thread_local! {
    static EPSILON: Cell<f64> = Cell::new(1e-6);
}

#[allow(dead_code)]
pub fn set_epsilon(eps: f64) {
    EPSILON.with(|e| e.set(eps));
}

#[allow(dead_code)]
pub fn get_epsilon() -> f64 {
    EPSILON.with(|e| e.get())
}

#[allow(dead_code)]
pub fn approx_eq(a: f64, b: f64) -> bool {
    let epsilon = get_epsilon();
    (a - b).abs() < epsilon.max(a.abs() * epsilon).max(b.abs() * epsilon)
}

#[allow(dead_code)]
pub fn exit(code: i32) -> ! {
    std::process::exit(code)
}


#[allow(dead_code)]
pub fn rand_index(bound: usize) -> usize {
	static mut STATE: u64 = 0x1234_5678_9ABC_DEF0;
	unsafe {
		STATE = STATE.wrapping_mul(6364136223846793005).wrapping_add(1);
		(STATE >> 32) as usize % bound
	}
}

#[allow(dead_code)]
pub fn randint(from: usize, to: usize) -> usize {
	// rand::rng().random_range(from..to)
	rand_index(to - from) + from
}

#[allow(dead_code)]
pub fn random() -> f64 {
    randint(0, 1000000000) as f64 / 1000000000.0
}
