use std::str::FromStr;

use ethers::prelude::*;
use ethers::abi::{AbiEncode};

use eyre::Result;

abigen!(VanityContract, "./out/VanityContract.sol/VanityContract.json");

fn main() -> Result<()> {
    let hash: [u8; 32] =  hex::decode("19bb34e293bba96bf0caeea54cdd3d2dad7fdf44cbea855173fa84534fcfb528")?.try_into()?;
    let x = IsValidSignatureCall::encode(
        IsValidSignatureCall
        {
            hash,
            signature: Bytes::from_str("hello").unwrap()
        });
    println!("{:?}", x);
    Ok(())
}
