#!/usr/bin/env rustc +script

// Direct test of custom syntax desugaring

// Test 1: def/fun keywords
def add(a: i32, b: i32) -> i32 {
    a + b
}

fun multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Test 2: and/or/xor operators
fn test_logic() {
    let x = true and false;
    let y = true or false;
    let z = 5 xor 3;
    println!("and={} or={} xor={}", x, y, z);
}

// Test 3: import fn (FFI declarations)
import fn cos(x: f64) -> f64;
import fn sin(x: f64) -> f64;

// Test 4: include (library linking)
include m;  // Links libm

fn main() {
    println!("add(2, 3) = {}", add(2, 3));
    println!("multiply(4, 5) = {}", multiply(4, 5));
    test_logic();

    unsafe {
        println!("cos(0.0) = {}", cos(0.0));
        println!("sin(0.0) = {}", sin(0.0));
    }
}
