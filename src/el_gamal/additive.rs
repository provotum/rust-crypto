use ::el_gamal::ciphertext::CipherText;

/// # Homomorphic Operation
///
/// Operate in a homomorphic way on the given cipher text.
pub trait Operate {
    fn operate(self, cipher_text: CipherText) -> CipherText;
}

impl Operate for CipherText {

    fn operate(self, cipher_text: CipherText) -> CipherText {
        CipherText {
            big_g: self.big_g * cipher_text.big_g,
            big_h: self.big_h * cipher_text.big_h,
            random: self.random + cipher_text.random
        }
    }
}

#[cfg(test)]
mod additive_tests {

    use ::arithmetic::mod_int::From;
    use ::arithmetic::mod_int::ModInt;
    use ::el_gamal::additive::Operate;
    use ::el_gamal::ciphertext::CipherText;
    use ::num::BigInt;

    #[test]
    fn test_additive_elgamal() {
        let c1: CipherText = CipherText {
            big_g: ModInt::from_value(BigInt::from(2)),
            big_h: ModInt::from_value(BigInt::from(4)),
            random: ModInt::from_value(BigInt::from(1)),
        };

        let c2: CipherText = CipherText {
            big_g: ModInt::from_value(BigInt::from(2)),
            big_h: ModInt::from_value(BigInt::from(1)),
            random: ModInt::from_value(BigInt::from(1)),
        };


        let res: CipherText = c1.operate(c2);

        assert_eq!(BigInt::from(4), res.big_g.value);
        assert_eq!(BigInt::from(4), res.big_h.value);
        assert_eq!(BigInt::from(2), res.random.value);
    }
}