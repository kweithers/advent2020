fn main() {
    // Both public keys are 7^d mod 20201227 for some d
    let card: u64 = 1717001;
    let door: u64 = 523731;

    let subject_number: u64 = 7;
    let modulus: u64 = 20201227;
    let mut card_loop_size: u64 = 0;

    // Determine card loop size (exponent)
    let mut v = 1;
    while v != card {
        v *= subject_number;
        v %= modulus;
        card_loop_size += 1;
    }

    // Determine encryption key (private key)
    // door^card_loop_size mod 20201227
    v = 1;
    for _i in 0..card_loop_size {
        v *= door;
        v %= modulus;
    }
    println!("{v}");
}
