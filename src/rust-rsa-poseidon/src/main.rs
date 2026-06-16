use blind_rsa_signatures::{KeyPair, Options};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
mod poseidon;
use rug::Integer;
use std::fs::File;
use std::io::{self, Write, BufWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("../zkp/prover_inputs/input_poseidon.txt")?;
    let mut writer = BufWriter::new(file);
    let mut lines: Vec<String> = Vec::new();
    let options = Options::default();

    let seed_n: u64 = 52; // Choose any fixed seed
    let mut n_rng = StdRng::seed_from_u64(seed_n);

    // let rng = &mut rand::thread_rng();

    // [SERVER]: Generate a RSA-2048 key pair
    let kp = KeyPair::generate(&mut n_rng, 2048)?;
    let (pk, sk) = (kp.pk, kp.sk);
    // [CLIENT]: create a random message and blind it for the server whose public key is `pk`.
    // The client must store the message and the secret.
    // let msg = b"hello";

    //seed so consistent for testing
    let seed: u64 = 42; // Choose any fixed seed
    let mut msg_rng: StdRng = StdRng::seed_from_u64(seed);
    let mut ID_A = [0u8; 8]; // 23 byte Id

    // This is the ID of party A -- requesting a new token!
    msg_rng.fill(&mut ID_A); // Fill with random bytes
    println!("ID_A : {:?}", ID_A);
    lines.push(format!("{:?}", ID_A));

    let blinding_result = pk.blind(&mut n_rng, ID_A, true, &options, &mut lines)?;

    // [SERVER]: compute a signature for a blind message, to be sent to the client.
    // The client secret should not be sent to the server.
    let blind_sig = sk.blind_sign(&mut n_rng, &blinding_result.blind_msg, &options)?;

    // [CLIENT]: later, when the client wants to redeem a signed blind message,
    // using the blinding secret, it can locally compute the signature of the
    // original message.
    // The client then owns a new valid (message, signature) pair, and the
    // server cannot link it to a previous(blinded message, blind signature) pair.
    // Note that the finalization function also verifies that the new signature
    // is correct for the server public key.
    // TO-DO signature wont verify because the hash is not changed 
    // let sig = pk.finalize(
    //     &blind_sig,
    //     &blinding_result.secret,
    //     blinding_result.msg_randomizer,
    //     &ID_A,
    //     &options,
    // )?;

    // VERIFICATION does not work 
    // sig.verify(&pk, blinding_result.msg_randomizer, ID_A, &options)?;

    fn u8_to_u32_be(bytes: Vec<u8>) -> Vec<u32> {
        bytes.chunks_exact(4)
            .map(|chunk| u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect()
    }
    
    // Generate ID_B:
    let seed_n: u64 = 100; // Choose any fixed seed
    let mut idb_rng = StdRng::seed_from_u64(seed_n);
    let mut ID_B = [0u8; 8]; // 23 byte Id
    // This is the ID of party A -- requesting a new token!
    idb_rng.fill(&mut ID_B); // Fill with random bytes
    lines.push(format!("{:?}", ID_B));
    // println!("ID_B: {:?}", ID_B);

    // Generate nonce for hashing both IDs:
    let seed_n: u64 = 101; // Choose any fixed seed
    let mut both_rng = StdRng::seed_from_u64(seed_n);
    let mut N_H = [0u8; 32]; // 23 byte Id
    // This is the ID of party A -- requesting a new token!
    both_rng.fill(&mut N_H); // Fill with random bytes
    // println!("nonce for id hash (N_H): {:?}", N_H);
    lines.push(format!("{:?}", N_H));


    let unpacked_h1: Vec<u8> = N_H.iter().chain(ID_A.iter()).chain(ID_B.iter()).copied().collect();
    let int_unpacked_h1: Vec<Integer> = unpacked_h1.iter().map(|&b| Integer::from(b)).collect();
    let int_packed_h1 = poseidon::pack_vector(int_unpacked_h1, 248, 8);
    let h1 = poseidon::poseidon_hash(int_packed_h1); 
    

    let unpacked_h2: Vec<u8> = N_H.iter().chain(ID_B.iter()).chain(ID_A.iter()).copied().collect();
    let int_unpacked_h2: Vec<Integer> = unpacked_h2.iter().map(|&b| Integer::from(b)).collect();
    let int_packed_h2 = poseidon::pack_vector(int_unpacked_h2, 248, 8);
    let h2 = poseidon::poseidon_hash(int_packed_h2); 
    
        
    let mut bit_rng = rand::thread_rng();
    let random_bit: u8 = bit_rng.gen_range(0..=1);
    // println!("{}", random_bit);
    lines.push(format!("{:?}", random_bit));

    if random_bit == 1 {
        // println!("H: {:?}", h1);
        lines.push(format!("{:?}", h1));
    }
    else{
        // println!("H: {:?}", h2);
        lines.push(format!("{:?}", h2));
    }

    // Generate nonce for old token:
    let seed_n: u64 = 153; // Choose any fixed seed
    let mut old_token_rng = StdRng::seed_from_u64(seed_n);
    let mut old_token = [0u8; 32]; // 23 byte Id
    // This is the ID of party A -- requesting a new token!
    old_token_rng.fill(&mut old_token); // Fill with random bytes
    // println!("nonce for old token (old_token): {:?}", old_token);
    lines.push(format!("{:?}", old_token));

    let unpacked_n_t_dash: Vec<u8> = old_token.iter().chain(ID_A.iter()).copied().collect();
    let int_unpacked_n_t_dash: Vec<Integer> = unpacked_n_t_dash.iter().map(|&b| Integer::from(b)).collect();
    let int_packed_n_t_dash = poseidon::pack_vector(int_unpacked_n_t_dash, 248, 8);
    let old_token = poseidon::poseidon_hash(int_packed_n_t_dash); 
    // println!("old token (N_t_dash): {:?}", old_token);
    lines.push(format!("{:?}", old_token));

    println!("Creating input_poseidon.txt ....");
    for line in lines {
        writeln!(writer, "{}", line)?; // Write each line to the file
    }
    Ok(())

}