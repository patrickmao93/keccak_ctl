use std::fs::File;
use std::io::Write;
use anyhow::Result;
use keccak_ctl::keccak::keccak256;
use keccak_hash::keccak;
use plonky2::{field::goldilocks_field::GoldilocksField, plonk::config::PoseidonGoldilocksConfig};

fn main() -> Result<()> {
    const D: usize = 2;
    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;

    // const MSG_LEN: usize = 1;
    const MSG_LEN: usize = 8704;
    // const MSG_LEN: usize = 139264;

    let input: Vec<u8> = (0..MSG_LEN).map(|_| rand::random()).collect();
    let expected = keccak(&input);
    let expected_false: Vec<u8> = (0..32).map(|_| rand::random()).collect();

    let (data, proof) = keccak256::<F, C, D>(&input, expected.as_bytes())?;

    println!("{:#?}", data.common);
    println!("{:#?}", data.verifier_only);

    println!("Proof size: {} bytes", proof.to_bytes().len());

    let proof_json = serde_json::to_string(&proof);
    let mut file = File::create(format!("keccak-proof-{}", MSG_LEN)).expect("failed to create file");
    file.write_all(proof_json.as_bytes()).expect("failed to write proof json");

    println!("file written");

    Ok(())
}