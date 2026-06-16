// Helper code from Sig-pop

use rug::Integer;

/// DEFAULT_MODULUS_STR
pub const DEFAULT_MODULUS_STR: &str = "52435875175126190479447740508185965837690552500527637822603658699938581184513"; // circ
pub const POSEIDON_CONST_PATH: &str = "./constants.zok";
pub const RATE: usize = 4;
pub const WIDTH: usize = RATE + 1;

use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use lazy_static::lazy_static;

/// Convert double array constant in ZoKrates to constant in rust
pub fn read_double_array(filename: &str) -> Result<Vec<Vec<Integer>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut vec_vec_constants = Vec::new();
    let mut vec_constants = Vec::new(); // inner vector
    let nested = 2;
    let mut current_nested = 0;
    for line in reader.lines() {
        let line = line?;

        if line.contains("const field[") {
            if current_nested == nested {
                break;
            }
            current_nested += 1;
            continue;
        }
        if current_nested != nested && line.contains("[") {
            current_nested += 1;
            continue;
        }

        if current_nested == nested {
            if line.contains("...[") {
                let trimmed = line.trim();
                let inner_parts: Vec<&str> = trimmed
                                                .trim_matches(|c: char| c == '.' || c == '[' || c == ']')
                                                .split(';')
                                                .map(|s| s.trim())
                                                .collect();
                // println!("inner part len {}", inner_parts.len());
                if inner_parts.len() == 2 {
                    // println!("inner parts {:?}", inner_parts);
                    if let (Ok(val), Ok(repeat)) = (inner_parts[0].parse::<i32>(), inner_parts[1].parse::<usize>()) {
                        // println!("val {} repeat {}", val, repeat);
                        vec_constants.extend(vec![Integer::from(val); repeat]);
                    }
                }
            }
            else if line.contains("]") && vec_constants.len() != 0 {
                vec_vec_constants.push(vec_constants);
                vec_constants = Vec::new();
            }
            else {
                let numbers: Vec<Integer> = line
                .split(',')
                .filter_map(|s| {
                    let num_str = s.trim_matches(|c: char| !c.is_numeric() && c != '-');
                    if !num_str.is_empty() {
                        Some(Integer::from_str_radix(num_str, 10).unwrap())
                    } else {
                        None
                    }
                })
                .collect();
                vec_constants.extend(numbers);
            }

            // constants.push(numbers);
        }
    }

    Ok(vec_vec_constants)
}

/// Convert triple array constant in ZoKrates to constant in rust
pub fn read_triple_array(filename: &str) -> Result<Vec<Vec<Vec<Integer>>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut vec_vec_vec_constants = Vec::new();
    let mut vec_vec_constants = Vec::new();
    let mut vec_constants = Vec::new(); // inner vector
    let nested = 3;
    let mut current_nested = 0;
    let mut is_reading = false;
    for line in reader.lines() {
        let line = line?;
        if line.contains("POSEIDON_M") {
            is_reading = true;
        }
        if is_reading {
            // println!("{} current_nested={}", line.clone(), current_nested);
            if line.contains("const field[") {
                if current_nested == nested {
                    break;
                }
                current_nested += 1;
                continue;
            }
            if current_nested != nested && line.contains("[") && !line.contains("...[") {
                if current_nested == nested-1 && line.contains("[0;") { // handle [0; 7]
                    // println!("line {}", line);
                    let trimmed = line.trim();
                    let inner_parts: Vec<&str> = trimmed
                                                    .trim_matches(|c: char| c == '.' || c == '[' || c == ']')
                                                    .split(';')
                                                    .map(|s| s.trim())
                                                    .collect();
                    // println!("inner part len {}", inner_parts.len());
                    if inner_parts.len() == 2 {
                        // println!("inner parts {:?}", inner_parts);
                        if let (Ok(val), Ok(repeat)) = (inner_parts[0].parse::<i32>(), inner_parts[1].parse::<usize>()) {
                            // println!("val {} repeat {}", val, repeat);
                            vec_vec_constants.push(vec![Integer::from(val); repeat]);
                        }
                        continue;
                    }                    
                }
                current_nested += 1;
                continue;
            }

            if current_nested == nested-1 && line.contains("...[") { // assume of the form ...[[0, x]; y]
                let trimmed = line.trim();
                // let inner_parts: Vec<&str> = trimmed
                //     .trim_matches(|c: char| c == '.' || c == '[' || c == ']')
                //     .split(';')
                //     .map(|s| s.trim()) // This trims each part, removing whitespace
                //     .collect();
                let inner_parts: Vec<&str> = trimmed
                .trim_matches(|c: char| c == '.')
                .split('[') // Split by '[' to handle nested structure
                .flat_map(|s| s.split(';')) // Split by ';' to get individual numbers
                .flat_map(|s| s.split(']')) // Split by ']' to get individual numbers
                .map(|s| s.trim()) // Trim whitespace from each part
                .filter(|s| !s.is_empty()) // Remove empty strings resulting from split
                .collect();                
                // Check if there are two parts and the first part is a valid integer
                if inner_parts.len() == 3 {
                    if let (Ok(val), Ok(inner_repeat), Ok(outer_repeat)) = (
                        inner_parts[0].parse::<i32>(),
                        inner_parts[1].parse::<usize>(),
                        inner_parts[2].parse::<usize>(),
                    ) {
                        // println!("val {} inner_repeat {} outer_repeat {}", val, inner_repeat, outer_repeat);
                        vec_vec_constants.extend(vec![vec![Integer::from(val); inner_repeat]; outer_repeat]);
                    }
                }
                // println!("notice! {} {:?}", line, inner_parts);

            } else if current_nested == nested-1 && line.contains("]") {
                if vec_vec_constants.len() != 0 {
                    vec_vec_vec_constants.push(vec_vec_constants);
                    vec_vec_constants = Vec::new();
                }
            }

            if current_nested == nested {
                if line.contains("]") && vec_constants.len() != 0 {
                    vec_vec_constants.push(vec_constants);
                    vec_constants = Vec::new();
                    current_nested -= 1;
                }
                else {
                    let numbers: Vec<Integer> = line
                    .split(',')
                    .filter_map(|s| {
                        let num_str = s.trim_matches(|c: char| !c.is_numeric() && c != '-');
                        if !num_str.is_empty() {
                            Some(Integer::from_str_radix(num_str, 10).unwrap())
                        } else {
                            None
                        }
                    })
                    .collect();
                    vec_constants.extend(numbers);
                }
    
                // constants.push(numbers);
            }
        }

    }

    Ok(vec_vec_vec_constants)
}

lazy_static! {
    /// DEFAULT_MODULUS
    pub static ref DEFAULT_MODULUS: Integer = Integer::from_str_radix(DEFAULT_MODULUS_STR, 10).unwrap();
    /// POSEIDON_C
    pub static ref POSEIDON_C: Vec<Vec<Integer>> = read_double_array(POSEIDON_CONST_PATH).unwrap();
    /// POSEIDON_M
    pub static ref POSEIDON_M: Vec<Vec<Vec<Integer>>> = read_triple_array(POSEIDON_CONST_PATH).unwrap();
}

// The `ark` function in Rust.
fn ark(state: &mut Vec<Integer>, c: &Vec<Integer>, it: usize) {
    for i in 0..state.len() {
        state[i] = (state[i].clone() + c[it + i].clone()) % DEFAULT_MODULUS.clone();
    }
}

// The `sbox` function in Rust.
fn sbox(state: &mut Vec<Integer>, f: usize, p: usize, r: usize) {
    state[0] = state[0].clone().pow_mod(&Integer::from(5), &DEFAULT_MODULUS).unwrap();
    for i in 1..state.len() {
        if r < f / 2 || r >= f / 2 + p {
            state[i] = state[i].clone().pow_mod(&Integer::from(5), &DEFAULT_MODULUS).unwrap();
        }
    }
}

// The `mix` function in Rust.
fn mix(state: &Vec<Integer>, m: &Vec<Vec<Integer>>) -> Vec<Integer> {
    let n = state.len();
    let mut out = vec![Integer::new(); n];

    for i in 0..n {
        let mut acc = Integer::new();
        for j in 0..n {
            acc = (acc.clone() + state[j].clone() * m[i][j].clone()) % DEFAULT_MODULUS.clone();
        }
        out[i] = acc;
    }

    out
}

// The `main` function in Rust.
// fn poseidon_hash(inputs: Vec<Integer>, poseidon_c: Vec<Vec<Integer>>, poseidon_m: Vec<Vec<Vec<Integer>>>) -> Integer {
pub fn poseidon_hash(inputs: Vec<Integer>) -> Integer {
    let n: usize = inputs.len();
    assert!(n > 0 && n <= 6); // max 6 inputs

    let t = n + 1;
    let rounds_p: Vec<usize> = vec![56, 57, 56, 60, 60, 63, 64, 63];

    let f = 8;
    let p = rounds_p[t - 2];

    let c: Vec<Integer> = POSEIDON_C[t - 2].to_vec();
    let m: Vec<Vec<Integer>> = POSEIDON_M[t - 2].to_vec();

    let mut state = vec![Integer::new(); t];
    for i in 1..t {
        state[i] = inputs[i - 1].clone();
    }

    for r in 0..f + p {
        ark(&mut state, &c, r * t);
        sbox(&mut state, f, p, r);
        state = mix(&state, &m);
    }

    state[0].clone()
}

fn ceil_div(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

fn zero_pad(inputs: Vec<Integer>, padded_size: usize) -> Vec<Integer>{
    let n: usize = inputs.len();
    assert!(padded_size >= n);
    let mut padded = vec![Integer::from(0); padded_size];
    for i in 0..n {
        padded[i] = inputs[i].clone();
    }
    padded
}

// fn permutation(mut state: Vec<Integer>) -> Vec<Integer> {
//     let n: usize = state.len();
//     assert!(n > 2 && n <= 7); // max width of 7 inputs

//     let t = n;
//     let rounds_p: Vec<usize> = vec![56, 57, 56, 60, 60, 63, 64, 63];

//     let f = 8;
//     let p = rounds_p[t - 2];

//     let c: Vec<Integer> = POSEIDON_C[t - 2].to_vec();
//     let m: Vec<Vec<Integer>> = POSEIDON_M[t - 2].to_vec();

//     for r in 0..f + p {
//         ark(&mut state, &c, r * t);
//         sbox(&mut state, f, p, r);
//         state = mix(&state, &m);
//     }

//     state.clone()
// }

// fn sponge(inputs: Vec<Integer>, domain: Integer) -> Integer{
//     let n: usize = inputs.len();
//     assert!(n % RATE == 0);
//     let mut state = vec![Integer::new(); WIDTH];
//     state[WIDTH -1] = domain;
//     for i in 0..n/RATE {
//         for j in 0..RATE{
//             state[j] = state[j].clone() + inputs[RATE*i + j].clone()
//         }
//         state = permutation(state)
//     } 

//     state[0].clone()
// }

pub fn pack_words(inputs: Vec<Integer>) -> Integer {
    let mut base: Integer = Integer::from(1);
    //each elem is a byte
    let shift: Integer = Integer::from(256);
    let mut output: Integer = Integer::from(0);
    for input in inputs{
        output = output + input * base.clone();
        base = base * shift.clone();
    }
    output
}


pub fn pack_vector(inputs: Vec<Integer>, new_size: usize, old_size: usize) -> Vec<Integer>{
    let n: usize = inputs.len();
    let words_per_field: usize = new_size / old_size;
    let outsize: usize = ceil_div(n,words_per_field);
    let padded_size: usize = outsize * words_per_field;
    let padded_inputs: Vec<Integer> = zero_pad(inputs, padded_size);
    let mut out = vec![Integer::from(0); outsize];

    for i in 0..outsize{
        out[i] = pack_words(padded_inputs[words_per_field*i..words_per_field*(i+1)].to_vec());
    }
    out
}

// fn poseidon_no_restrict(inputs: Vec<Integer>, domain: Integer) -> Integer {
//     let n: usize = inputs.len();
//     let padded_size = ceil_div(n,RATE) * RATE;
//     let mut padded: Vec<Integer> = zero_pad(inputs, padded_size);
//     sponge(padded,domain)
// }
