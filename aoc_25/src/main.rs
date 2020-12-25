fn fast_modular_exponentiation(base: i128, exponent: i128, modulus: i128) -> i128 {
    let powers_of_two = std::iter::successors(Some((base % modulus, 2)), |(val, power)| {
        if *power > exponent { None } else { Some(((val * val) % modulus, power << 1)) }
    }).map(|(val, _)| val).collect::<Vec<i128>>();

    let (mut out, mut exponent_left, mut power_idx) = (1, exponent, 0);
    while exponent_left != 0 {
        if (exponent_left & 1) == 1 { out = (out * powers_of_two[power_idx]) % modulus; }
        exponent_left >>= 1;
        power_idx += 1;
    }
    out
}

fn brute_force_loop_size(public_key: i128) -> i128 {
    (2..).find(|loop_size| fast_modular_exponentiation(7, *loop_size, 20201227) == public_key).unwrap()
}

fn part1(public_keys: (i128, i128)) {
    let key1_loop_size = brute_force_loop_size(public_keys.0);
    let encryption_key = fast_modular_exponentiation(public_keys.1, key1_loop_size, 20201227);
    println!("[Part 1] Encryption key: {}", encryption_key);
}

fn main() {
    let public_keys: (i128, i128) = (8421034, 15993936);
    part1(public_keys);
}