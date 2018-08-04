/*
    Cryptography utilities

    Copyright 2018 by Kzen Networks

    This file is part of Cryptography utilities library
    (https://github.com/KZen-networks/cryptography-utils)

    Cryptography utilities is free software: you can redistribute
    it and/or modify it under the terms of the GNU General Public
    License as published by the Free Software Foundation, either
    version 3 of the License, or (at your option) any later version.

    @license GPL-3.0+ <https://github.com/KZen-networks/cryptography-utils/blob/master/LICENSE>
*/
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod elliptic;

// TODO: When we will have more than one type of elliptic curve, add as features
//pub use elliptic::curves::secp256_k1::EC;
//pub use elliptic::curves::secp256_k1::PK;
//pub use elliptic::curves::secp256_k1::SK;
//pub use elliptic::curves::secp256_k1::GE;
//pub use elliptic::curves::secp256_k1::FE;

pub use elliptic::curves::curve25519::PK;
pub use elliptic::curves::curve25519::SK;
pub use elliptic::curves::curve25519::GE;
pub use elliptic::curves::curve25519::FE;


// TODO: When we will have more than one type of big num library, add as features
pub mod arithmetic;
pub use arithmetic::big_gmp::BigInt;

pub mod cryptographic_primitives;