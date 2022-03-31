/*
    This file is part of Curv library
    Copyright 2018 by Kzen Networks
    (https://github.com/KZen-networks/curv)
    License MIT: <https://github.com/KZen-networks/curv/blob/master/LICENSE>
*/
pub mod arithmetic;
pub use crate::arithmetic::BigInt;

mod marker;
pub use marker::HashChoice;

mod test_utils;

#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum ErrorKey {
    InvalidPublicKey,
}

pub enum ErrorSS {
    VerifyShareError,
}
