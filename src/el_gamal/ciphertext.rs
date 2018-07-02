use ::arithmetic::mod_int::ModInt;

/// # ElGamal CipherText.
pub struct CipherText {
    pub big_g: ModInt,
    pub big_h: ModInt,
    pub random: ModInt
}