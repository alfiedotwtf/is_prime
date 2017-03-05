extern crate ramp;

use ramp::int::Int;

pub fn is_prime(n_str: &str) -> bool {
    let mut witnesses = n_str.len();

    if witnesses < 3 {
        witnesses = 3;
    }

    is_prime_with_witnesses(n_str, witnesses)
}

// translated from https://rosettacode.org/wiki/Miller%E2%80%93Rabin_primality_test#Perl
pub fn is_prime_with_witnesses(n_str: &str, witnesses: usize) -> bool {
    let two        = Int::from_str_radix("2", 10).unwrap();
    let n          = Int::from_str_radix(n_str, 10).unwrap();
    let n_sub: Int = n.clone() - 1;

    if n == 2 || n == 3 {
       return true;
    }

    if n < 2 || (n.divmod(&two).1 == 0) {
       return false;
    }

    let mut exponent = n_sub.clone();
    let mut trials   = Int::zero();

    while exponent.divmod(&two).1 == 0 {
        exponent = exponent / 2;
        trials   = trials + 1;
    }

    'LOOP: for i in 1..(witnesses + 1) {
        let mut result = powmod(&(two.clone() + i), &exponent, &n);

        if result == 1 || result == n_sub {
            continue;
        }

        let mut verified = 1;

        while verified < trials {
            result = result.square().divmod(&n).1;

            if result == 1 {
                return false;
            }

            if result == n_sub {
                continue 'LOOP;
            }

            verified = verified + 1;
        }

        return false;
    }

    return true
}

// translated from http://search.cpan.org/~pjacklam/Math-BigInt-1.999810/lib/Math/BigInt.pm#Arithmetic_methods
fn powmod(base: &Int, exponent: &Int, modulus: &Int) -> Int {
    if *base == Int::zero() {
        return match *exponent == Int::zero() {
            true  => Int::one(),
            false => Int::zero(),
        }
    }

    if *modulus == Int::one() {
        return Int::zero();
    }

    let exponent_in_binary      = Int::to_str_radix(&exponent, 2, false);
    let mut exponent_chars_revd = exponent_in_binary.chars().rev();
    let mut exponent_length     = exponent_in_binary.len();
    let mut my_base             = base.clone();
    let mut result              = Int::one();

    while exponent_length > 0 {
        exponent_length = exponent_length - 1;

        if exponent_chars_revd.next().unwrap() == '1' {
            result = result * my_base.clone();
            result = result.divmod(&modulus).1;
        }

        my_base = my_base.square();
        my_base = my_base.divmod(&modulus).1;
    }

    result
}
