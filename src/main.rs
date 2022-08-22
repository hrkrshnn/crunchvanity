use ethers::abi::AbiEncode;
use ethers::prelude::*;

use sha256::digest_bytes;

use hex;

use rayon::prelude::*;

// Generates the binding `IsValidSignatureCall`
// Need to run `forge build` before `cargo build`.
abigen!(VanityContract, "./out/IERC1271.sol/IERC1271.json");

fn to_signature(i: u64) -> Bytes {
    let signature: Vec<u8> = format!("{}", i).as_bytes().into();
    Bytes::from(signature)
}

fn abi_encode(hash: [u8; 32], signature: Bytes) -> Vec<u8> {
    IsValidSignatureCall { hash, signature }.encode()
}

fn main() {
    // >>> cast keccak "CHALLENGE_MAGIC"
    // 0x19bb34e293bba96bf0caeea54cdd3d2dad7fdf44cbea855173fa84534fcfb528
    let hash: [u8; 32] =
        hex::decode("19bb34e293bba96bf0caeea54cdd3d2dad7fdf44cbea855173fa84534fcfb528")
            .unwrap()
            .try_into()
            .unwrap();

    let res = (0..u64::MAX).into_par_iter().find_any(|i| {
        let signature = to_signature(*i);
        let digest = digest_bytes(&abi_encode(hash, signature));
        // >>> cast sig "isValidSignature(bytes32,bytes)"
        // 0x1626ba7e
        digest[..8] == String::from("1626ba7e")
    });

    if let Some(i) = res {
        println!("i: {}", i);
        let signature = to_signature(i);
        let abi_encoding = abi_encode(hash, signature.clone());
        let digest = digest_bytes(&abi_encoding);
        println!("signature: {}", signature);
        println!("ABI encoding: {:?}", hex::encode(abi_encoding));
        println!("sha256: {}", digest);
    } else {
        println!("Crunching failed. Bigger range?");
    }
}
