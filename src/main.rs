use ethers::prelude::*;
use ethers::abi::{AbiEncode};

use eyre::Result;

use sha256::{, digest_bytes};

use rayon::prelude::*;

abigen!(VanityContract, "./out/VanityContract.sol/VanityContract.json");

fn main() -> Result<()> {
    let hash: [u8; 32] = hex::decode("19bb34e293bba96bf0caeea54cdd3d2dad7fdf44cbea855173fa84534fcfb528")?.try_into().unwrap();
    let selector: [u8; 4] = hex::decode("1626ba7e")?.try_into().unwrap();

    let res = (0..u64::MAX).into_par_iter().
        find_any(|i| {
            let signature = format!("{}",i);
            let signature: Vec<u8> = signature.as_bytes().into();
            let signature = Bytes::from(signature);
            let x = IsValidSignatureCall::encode(
                IsValidSignatureCall
                {
                    hash,
                    signature: signature.clone()
                });

            let digest = digest_bytes(&x);
            if digest[..8] == String::from("1626ba7e") {
                println!("i: {}", i);
                println!("signature: {}", signature);
                println!("digest: {}", digest);
                true
            }
            else {
                false
            }

        });

    println!("{:?}", res);
    Ok(())
}
