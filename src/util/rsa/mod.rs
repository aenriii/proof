use num_traits::cast::ToPrimitive;

use crate::notation;
pub type PubKey = (u64, u64);
pub type PrivKey = (u64, u64);
pub type KeyPair = (PubKey, PrivKey);
pub type Message = Vec<u64>;

pub fn fermat(n: u64) -> bool {
    return mod_pow(2, n-1, n) == 1;
}

pub fn mod_pow(a_base: u64, a_exponent: u64, modulus: u64) -> u64 {
    let mut base = a_base;
    let mut exponent = a_exponent;
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
    println!("{} = {}^{} mod {}", result, a_base, a_exponent, modulus);
    return result;
}
pub fn gcd( num1: u64, num2: u64 ) -> u64 {

    if num1 == 0 {
        println!("{} = 0 {} gcd({}, {}) {} {}", num1, notation::IMPLIES, num1, num2, notation::IMPLIES, num2);
        return num2;
    } 
    // println!("{} {} 0", num1, notation::NOT_EQ);
    if num2 == 0 {
        println!("{} = 0 {} gcd({}, {}) {} {}", num2, notation::IMPLIES, num1, num2, notation::IMPLIES, num1);
        return num1;
    }
    // println!("{} {} 0", num2, notation::NOT_EQ);
    if num1 == num2 {
        println!("{} = {} {} gcd({}, {}) {} {}", num1, notation::IMPLIES, num2, num1, num2, notation::IMPLIES, num1);
        return num1;
    }
    // println!("{} {} {}", num1, notation::NOT_EQ, num2);
    println!("gcd({}, {}) = gcd({}, {})", num1, num2, num2, num1 % num2);
    return gcd(num2, num1 % num2);

}

pub fn lcm( num1: u64, num2: u64 ) -> u64 {
    return num1 * num2 / gcd(num1, num2);
}
#[derive(Debug)]
pub struct RsaData {
    pub n: u64,
    pub totient: u64,
    pub pubk: PubKey,
    pub privk: PrivKey,
}

pub fn get_rsa_data( p: u64, q: u64 ) -> RsaData {
    let n = p * q;
    let totient = lcm(p-1, q-1);
    let mut pubk = (0, 0);
    let mut privk = (0, 0);
    for i in 2..totient {
        if gcd(i, totient) == 1 {
            pubk = (i, n);
            break;
        }
    }
    for i in 2..totient {
        if (pubk.0 * i) % totient == 1 {
            privk = (i, n);
            break;
        }
    }
    return RsaData {
        n,
        totient,
        pubk,
        privk,
    }
}

pub fn encode(message: String, key: PrivKey) -> Message {
    let mut encoded = Vec::new();
    for c in message.chars() {
        encoded.push(
            mod_pow((c.clone() as u8).to_u64().unwrap(), 
            key.0.to_u64().unwrap(), 
            key.1.to_u64().unwrap()).to_u32().unwrap() as u64);
    }
    return encoded;
}
pub fn decode(message: Message, key: PubKey) -> String {
    let mut decoded = String::new();
    for c in message {
        decoded.push(
            mod_pow(c.to_u64().unwrap(), 
            key.0.to_u64().unwrap(), 
            key.1.to_u64().unwrap()).to_u32().unwrap() as u8 as char);
    }
    return decoded;
}