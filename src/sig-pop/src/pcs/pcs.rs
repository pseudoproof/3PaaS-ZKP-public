/// run_zk example
use fxhash::FxHashMap as HashMap;
use crate::ir::term::Value;
use crate::convert::integer_to_field;
use crate::convert::str_to_field;
use std::fs::read_to_string;
use rug::{Integer, rand::RandState};
use crate::commitment::{poseidon_hash, pack_vector};
use crate::convert::rand_int;
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}


pub fn prover_input_pcs_sha(x: String) -> HashMap<String, Value> {
    let contents = read_lines(&x);
    let mut map = HashMap::<String, Value>::default();
    let ID_A_trimmed = &contents[0][1..contents[0].len() - 1];
    let ID_A_items: Vec<&str> = ID_A_trimmed.split(',').collect();
    
    for (index, word) in ID_A_items.iter().enumerate()  {
        map.insert(format!("ID_A.{}", index), str_to_field(&word.clone()));
    }
    
    let N_ID_A_trimmed = &contents[1][1..contents[1].len() - 1];
    let N_ID_A_items: Vec<&str> = N_ID_A_trimmed.split(',').collect();
    
    for (index, word) in N_ID_A_items.iter().enumerate()  {
        map.insert(format!("N_ID_A.{}", index), str_to_field(&word.clone()));
    }

    let salt_trimmed = &contents[2][1..contents[2].len() - 1];
    let salt_items: Vec<&str> = salt_trimmed.split(',').collect();
    
    for (index, word) in salt_items.iter().enumerate()  {
        map.insert(format!("salt.{}", index), str_to_field(&word.clone()));
    }


    let EM_trimmed = &contents[3][1..contents[3].len() - 1];
    let EM_items: Vec<&str> = EM_trimmed.split(',').collect();
    
    for (index, word) in EM_items.iter().enumerate()  {
        map.insert(format!("EM.{}", index), str_to_field(&word.clone()));
    }

    let r_trimmed = &contents[4][1..contents[4].len() - 1];
    let r_items: Vec<&str> = r_trimmed.split(',').collect();

    for (index, word) in r_items.iter().enumerate()  {
        map.insert(format!("r.{}", index), str_to_field(&word.clone()));
    }

    let blinder_trimmed = &contents[5][1..contents[5].len() - 1];
    let blinder_items: Vec<&str> = blinder_trimmed.split(',').collect();

    for (index, word) in blinder_items.iter().enumerate()  {
        map.insert(format!("blinder.{}", index), str_to_field(&word.clone()));
    }

    let blinded_trimmed = &contents[6][1..contents[6].len() - 1];
    let blinded_items: Vec<&str> = blinded_trimmed.split(',').collect();

    for (index, word) in blinded_items.iter().enumerate()  {
        map.insert(format!("blinded.{}", index), str_to_field(&word.clone()));
    }
    
    let ID_B_trimmed = &contents[7][1..contents[7].len() - 1];
    let ID_B_items: Vec<&str> = ID_B_trimmed.split(',').collect();
    
    for (index, word) in ID_B_items.iter().enumerate()  {
        map.insert(format!("ID_B.{}", index), str_to_field(&word.clone()));
    }
    
    let N_H_trimmed = &contents[8][1..contents[8].len() - 1];
    let N_H_items: Vec<&str> = N_H_trimmed.split(',').collect();
    
    for (index, word) in N_H_items.iter().enumerate()  {
        map.insert(format!("N_H.{}", index), str_to_field(&word.clone()));
    }
        
    let H_r = &contents[9];
    map.insert(format!("H_r"), str_to_field(&H_r.clone()));

    let H_trimmed = &contents[10][1..contents[10].len() - 1];
    let H_items: Vec<&str> = H_trimmed.split(',').collect();
    
    for (index, word) in H_items.iter().enumerate()  {
        map.insert(format!("H.{}", index), str_to_field(&word.clone()));
    } 

    let N_old_token_trimmed = &contents[11][1..contents[11].len() - 1];
    let N_old_token_items: Vec<&str> = N_old_token_trimmed.split(',').collect();
    
    for (index, word) in N_old_token_items.iter().enumerate()  {
        map.insert(format!("N_old_token.{}", index), str_to_field(&word.clone()));
    }

    let old_token_trimmed = &contents[12][1..contents[12].len() - 1];
    let old_token_items: Vec<&str> = old_token_trimmed.split(',').collect();
    
    for (index, word) in old_token_items.iter().enumerate()  {
        map.insert(format!("old_token.{}", index), str_to_field(&word.clone()));
    }
    
    let left_leaf = &contents[13];
    map.insert(format!("left_leaf"), str_to_field(&left_leaf.clone()));

    let direction_left_trimmed = &contents[14][1..contents[14].len() - 1];
    let direction_left_items: Vec<&str> = direction_left_trimmed.split(',').collect();
    
    for (index, word) in direction_left_items.clone().iter().enumerate()  {
        map.insert(format!("direction_left.{}", index), str_to_field(&word.clone()));
    }

    let auth_path_left_trimmed = &contents[15][1..contents[15].len() - 1];
    let auth_path_left_items: Vec<&str> = auth_path_left_trimmed.split(',').collect();
    
    for (index, word) in auth_path_left_items.clone().iter().enumerate()  {
        map.insert(format!("auth_path_left.{}", index), str_to_field(&word.clone()));
    }

    let right_leaf = &contents[16];
    map.insert(format!("right_leaf"), str_to_field(&right_leaf.clone()));

    let direction_right_trimmed = &contents[17][1..contents[17].len() - 1];
    let direction_right_items: Vec<&str> = direction_right_trimmed.split(',').collect();
    
    for (index, word) in direction_right_items.clone().iter().enumerate()  {
        map.insert(format!("direction_right.{}", index), str_to_field(&word.clone()));
    }

    let auth_path_right_trimmed = &contents[18][1..contents[18].len() - 1];
    let auth_path_right_items: Vec<&str> = auth_path_right_trimmed.split(',').collect();
    
    for (index, word) in auth_path_right_items.clone().iter().enumerate()  {
        map.insert(format!("auth_path_right.{}", index), str_to_field(&word.clone()));
    }

    let sub_root= &contents[19];
    map.insert(format!("sub_root"), str_to_field(&sub_root.clone()));

    let ID_A_trimmed = &contents[20][1..contents[20].len() - 1];
    let ID_A_items: Vec<&str> = ID_A_trimmed.split(',').collect();
    
    for (index, word) in ID_A_items.clone().iter().enumerate()  {
        map.insert(format!("ID_A.{}", index), str_to_field(&word.clone()));
    }

    let direction_root_trimmed = &contents[21][1..contents[21].len() - 1];
    let direction_root_items: Vec<&str> = direction_root_trimmed.split(',').collect();
    
    for (index, word) in direction_root_items.clone().iter().enumerate()  {
        map.insert(format!("direction_root.{}", index), str_to_field(&word.clone()));
    }

    let auth_path_root_trimmed = &contents[22][1..contents[22].len() - 1];
    let auth_path_root_items: Vec<&str> = auth_path_root_trimmed.split(',').collect();
    
    for (index, word) in auth_path_root_items.clone().iter().enumerate()  {
        map.insert(format!("auth_path_root.{}", index), str_to_field(&word.clone()));
    }

    let root= &contents[23];
    map.insert(format!("root"), str_to_field(&root.clone()));

    map
}

pub fn verifier_input_pcs_sha(x: String) -> HashMap<String, Value> {
    let contents = read_lines(&x);
    let mut map = HashMap::<String, Value>::default();

    let blinded_trimmed = &contents[6][1..contents[6].len() - 1];
    let blinded_items: Vec<&str> = blinded_trimmed.split(',').collect();
    
    for (index, word) in blinded_items.iter().enumerate()  {
        map.insert(format!("blinded.{}", index), str_to_field(&word.clone()));
    }
    
    let H_trimmed = &contents[10][1..contents[10].len() - 1];
    let H_items: Vec<&str> = H_trimmed.split(',').collect();
    
    for (index, word) in H_items.iter().enumerate()  {
        map.insert(format!("H.{}", index), str_to_field(&word.clone()));
    } 
    
    let old_token_trimmed = &contents[12][1..contents[12].len() - 1];
    let old_token_items: Vec<&str> = old_token_trimmed.split(',').collect();
    
    for (index, word) in old_token_items.iter().enumerate()  {
        map.insert(format!("old_token.{}", index), str_to_field(&word.clone()));
    }

    let root= &contents[23];
    map.insert(format!("root"), str_to_field(&root.clone()));


    map
}

pub fn prover_input_pcs_poseidon(x: String) -> HashMap<String, Value> {
    let contents = read_lines(&x);
    let mut map = HashMap::<String, Value>::default();
    let ID_A_trimmed = &contents[0][1..contents[0].len() - 1];
    let ID_A_items: Vec<&str> = ID_A_trimmed.split(',').collect();
    
    for (index, word) in ID_A_items.iter().enumerate()  {
        map.insert(format!("ID_A.{}", index), str_to_field(&word.clone()));
    }
    
    let N_ID_A_trimmed = &contents[1][1..contents[1].len() - 1];
    let N_ID_A_items: Vec<&str> = N_ID_A_trimmed.split(',').collect();
    
    for (index, word) in N_ID_A_items.iter().enumerate()  {
        map.insert(format!("N_ID_A.{}", index), str_to_field(&word.clone()));
    }

    let salt_trimmed = &contents[2][1..contents[2].len() - 1];

    let salt_items: Vec<&str> = salt_trimmed.split(',').collect();
    
    for (index, word) in salt_items.iter().enumerate()  {
        map.insert(format!("salt.{}", index), str_to_field(&word.clone()));
    }


    let EM_trimmed = &contents[3][1..contents[3].len() - 1];
    let EM_items: Vec<&str> = EM_trimmed.split(',').collect();
    
    for (index, word) in EM_items.iter().enumerate()  {
        map.insert(format!("EM.{}", index), str_to_field(&word.clone()));
    }

    let r_trimmed = &contents[4][1..contents[4].len() - 1];
    let r_items: Vec<&str> = r_trimmed.split(',').collect();

    for (index, word) in r_items.iter().enumerate()  {
        map.insert(format!("r.{}", index), str_to_field(&word.clone()));
    }

    let blinder_trimmed = &contents[5][1..contents[5].len() - 1];
    let blinder_items: Vec<&str> = blinder_trimmed.split(',').collect();

    for (index, word) in blinder_items.iter().enumerate()  {
        map.insert(format!("blinder.{}", index), str_to_field(&word.clone()));
    }

    let blinded_trimmed = &contents[6][1..contents[6].len() - 1];
    let blinded_items: Vec<&str> = blinded_trimmed.split(',').collect();

    for (index, word) in blinded_items.iter().enumerate()  {
        map.insert(format!("blinded.{}", index), str_to_field(&word.clone()));
    }
    
    let ID_B_trimmed = &contents[7][1..contents[7].len() - 1];
    let ID_B_items: Vec<&str> = ID_B_trimmed.split(',').collect();
    
    for (index, word) in ID_B_items.iter().enumerate()  {
        map.insert(format!("ID_B.{}", index), str_to_field(&word.clone()));
    }
    
    let N_H_trimmed = &contents[8][1..contents[8].len() - 1];
    let N_H_items: Vec<&str> = N_H_trimmed.split(',').collect();
    
    for (index, word) in N_H_items.iter().enumerate()  {
        map.insert(format!("N_H.{}", index), str_to_field(&word.clone()));
    }
        
    let H_r = &contents[9];
    map.insert(format!("H_r"), str_to_field(&H_r.clone()));

    let H = &contents[10];
    map.insert(format!("H"), str_to_field(&H.clone()));

    let N_old_token_trimmed = &contents[11][1..contents[11].len() - 1];
    let N_old_token_items: Vec<&str> = N_old_token_trimmed.split(',').collect();
    
    for (index, word) in N_old_token_items.iter().enumerate()  {
        map.insert(format!("N_old_token.{}", index), str_to_field(&word.clone()));
    }
    let old_token = &contents[12];
    map.insert(format!("old_token"), str_to_field(&old_token.clone()));

    let left_leaf = &contents[13];
    map.insert(format!("left_leaf"), str_to_field(&left_leaf.clone()));

    let direction_left_trimmed = &contents[14][1..contents[14].len() - 1];
    let direction_left_items: Vec<&str> = direction_left_trimmed.split(',').collect();
    
    for (index, word) in direction_left_items.clone().iter().enumerate()  {
        map.insert(format!("direction_left.{}", index), str_to_field(&word.clone()));
    }

    let auth_path_left_trimmed = &contents[15][1..contents[15].len() - 1];
    let auth_path_left_items: Vec<&str> = auth_path_left_trimmed.split(',').collect();
    
    for (index, word) in auth_path_left_items.clone().iter().enumerate()  {
        map.insert(format!("auth_path_left.{}", index), str_to_field(&word.clone()));
    }

    let right_leaf = &contents[16];
    map.insert(format!("right_leaf"), str_to_field(&right_leaf.clone()));

    let direction_right_trimmed = &contents[17][1..contents[17].len() - 1];
    let direction_right_items: Vec<&str> = direction_right_trimmed.split(',').collect();
    
    for (index, word) in direction_right_items.clone().iter().enumerate()  {
        map.insert(format!("direction_right.{}", index), str_to_field(&word.clone()));
    }

    let auth_path_right_trimmed = &contents[18][1..contents[18].len() - 1];
    let auth_path_right_items: Vec<&str> = auth_path_right_trimmed.split(',').collect();
    
    for (index, word) in auth_path_right_items.clone().iter().enumerate()  {
        map.insert(format!("auth_path_right.{}", index), str_to_field(&word.clone()));
    }

    let sub_root= &contents[19];
    map.insert(format!("sub_root"), str_to_field(&sub_root.clone()));

    let ID_A_trimmed = &contents[20][1..contents[20].len() - 1];
    let ID_A_items: Vec<&str> = ID_A_trimmed.split(',').collect();
    
    for (index, word) in ID_A_items.clone().iter().enumerate()  {
        map.insert(format!("ID_A.{}", index), str_to_field(&word.clone()));
    }

    let direction_root_trimmed = &contents[21][1..contents[21].len() - 1];
    let direction_root_items: Vec<&str> = direction_root_trimmed.split(',').collect();
    
    for (index, word) in direction_root_items.clone().iter().enumerate()  {
        map.insert(format!("direction_root.{}", index), str_to_field(&word.clone()));
    }

    let auth_path_root_trimmed = &contents[22][1..contents[22].len() - 1];
    let auth_path_root_items: Vec<&str> = auth_path_root_trimmed.split(',').collect();
    
    for (index, word) in auth_path_root_items.clone().iter().enumerate()  {
        map.insert(format!("auth_path_root.{}", index), str_to_field(&word.clone()));
    }

    let root= &contents[23];
    map.insert(format!("root"), str_to_field(&root.clone()));


    map
}

/// generate verifier input for run_zk_example
pub fn verifier_input_pcs_poseidon(x: String) -> HashMap<String, Value> {
    let contents = read_lines(&x);
    let mut map = HashMap::<String, Value>::default();

    let blinded_trimmed = &contents[6][1..contents[6].len() - 1];
    let blinded_items: Vec<&str> = blinded_trimmed.split(',').collect();
    
    for (index, word) in blinded_items.iter().enumerate()  {
        map.insert(format!("blinded.{}", index), str_to_field(&word.clone()));
    }
    
    let H = &contents[10];
    map.insert(format!("H"), str_to_field(&H.clone()));
    
    let old_token = &contents[12];
    map.insert(format!("old_token"), str_to_field(&old_token.clone()));

    let root= &contents[23];
    map.insert(format!("root"), str_to_field(&root.clone()));

    map
}

pub fn prover_input_pcs_poseidon_preprocess(x: String) -> HashMap<String, Value> {
    let contents = read_lines(&x);
    let mut map = HashMap::<String, Value>::default();
    let ID_A_trimmed = &contents[0][1..contents[0].len() - 1];
    let ID_A_items: Vec<&str> = ID_A_trimmed.split(',').collect();
    
    for (index, word) in ID_A_items.clone().iter().enumerate()  {
        map.insert(format!("ID_A.{}", index), str_to_field(&word.clone()));
    }
    
    let N_ID_A_trimmed = &contents[1][1..contents[1].len() - 1];
    let N_ID_A_items: Vec<&str> = N_ID_A_trimmed.split(',').collect();
    
    for (index, word) in N_ID_A_items.iter().enumerate()  {
        map.insert(format!("N_ID_A.{}", index), str_to_field(&word.clone()));
    }

    let salt_trimmed = &contents[2][1..contents[2].len() - 1];

    let salt_items: Vec<&str> = salt_trimmed.split(',').collect();
    
    for (index, word) in salt_items.iter().enumerate()  {
        map.insert(format!("salt.{}", index), str_to_field(&word.clone()));
    }


    let EM_trimmed = &contents[3][1..contents[3].len() - 1];
    let EM_items: Vec<&str> = EM_trimmed.split(',').collect();
    
    for (index, word) in EM_items.iter().enumerate()  {
        map.insert(format!("EM.{}", index), str_to_field(&word.clone()));
    }

    let r_trimmed = &contents[4][1..contents[4].len() - 1];
    let r_items: Vec<&str> = r_trimmed.split(',').collect();

    for (index, word) in r_items.iter().enumerate()  {
        map.insert(format!("r.{}", index), str_to_field(&word.clone()));
    }

    let blinder_trimmed = &contents[5][1..contents[5].len() - 1];
    let blinder_items: Vec<&str> = blinder_trimmed.split(',').collect();

    for (index, word) in blinder_items.iter().enumerate()  {
        map.insert(format!("blinder.{}", index), str_to_field(&word.clone()));
    }

    let blinded_trimmed = &contents[6][1..contents[6].len() - 1];
    let blinded_items: Vec<&str> = blinded_trimmed.split(',').collect();

    for (index, word) in blinded_items.iter().enumerate()  {
        map.insert(format!("blinded.{}", index), str_to_field(&word.clone()));
    }
    
    let ID_B_trimmed = &contents[7][1..contents[7].len() - 1];
    let ID_B_items: Vec<&str> = ID_B_trimmed.split(',').collect();
    
    for (index, word) in ID_B_items.iter().enumerate()  {
        map.insert(format!("ID_B.{}", index), str_to_field(&word.clone()));
    }
    
    let N_H_trimmed = &contents[8][1..contents[8].len() - 1];
    let N_H_items: Vec<&str> = N_H_trimmed.split(',').collect();
    
    for (index, word) in N_H_items.iter().enumerate()  {
        map.insert(format!("N_H.{}", index), str_to_field(&word.clone()));
    }
        
    let H_r = &contents[9];
    map.insert(format!("H_r"), str_to_field(&H_r.clone()));

    let H = &contents[10];
    map.insert(format!("H"), str_to_field(&H.clone()));

    let N_old_token_trimmed = &contents[11][1..contents[11].len() - 1];
    let N_old_token_items: Vec<&str> = N_old_token_trimmed.split(',').collect();
    
    for (index, word) in N_old_token_items.iter().enumerate()  {
        map.insert(format!("N_old_token.{}", index), str_to_field(&word.clone()));
    }
    let old_token = &contents[12];
    map.insert(format!("old_token"), str_to_field(&old_token.clone()));

    let mut rng = RandState::new_mersenne_twister();
    //for proof gen testing - so deterministic for now:
    rng.seed(&Integer::from(7));
    let mut commit_rand = rand_int(&mut rng);
    map.insert(format!("commit_rand"), integer_to_field(&commit_rand));

    let u8_ID_A_items: Vec<u8> = ID_A_items.iter()
    .map(|s| s.trim().parse::<u8>().unwrap())
    .collect();

    let mut unpacked: Vec<Integer> = u8_ID_A_items.iter().map(|&byte| Integer::from(byte)).collect();
    let mut packed = pack_vector(unpacked, 248,8);
    let commit_msg = vec![commit_rand, packed[0].clone()];
    let commit_ID_A = poseidon_hash(commit_msg);
    map.insert(format!("commit_ID_A"), integer_to_field(&commit_ID_A));

    map 
}

/// generate verifier input for run_zk_example
pub fn verifier_input_pcs_preprocess(x: String) -> HashMap<String, Value> {
    let contents = read_lines(&x);
    let mut map = HashMap::<String, Value>::default();

    let blinded_trimmed = &contents[6][1..contents[6].len() - 1];
    let blinded_items: Vec<&str> = blinded_trimmed.split(',').collect();
    
    for (index, word) in blinded_items.iter().enumerate()  {
        map.insert(format!("blinded.{}", index), str_to_field(&word.clone()));
    }
    
    let H = &contents[10];
    map.insert(format!("H"), str_to_field(&H.clone()));
    
    let old_token = &contents[12];
    map.insert(format!("old_token"), str_to_field(&old_token.clone()));

    let ID_A_trimmed = &contents[0][1..contents[0].len() - 1];
    let ID_A_items: Vec<&str> = ID_A_trimmed.split(',').collect();

    let u8_ID_A_items: Vec<u8> = ID_A_items.iter()
    .map(|s| s.trim().parse::<u8>().unwrap())
    .collect();

    let mut unpacked: Vec<Integer> = u8_ID_A_items.iter().map(|&byte| Integer::from(byte)).collect();
    let mut packed = pack_vector(unpacked, 248,8);
    
    let mut rng = RandState::new_mersenne_twister();
    rng.seed(&Integer::from(7));
    let mut commit_rand = rand_int(&mut rng);
    let commit_msg = vec![commit_rand, packed[0].clone()];
    let commit_ID_A = poseidon_hash(commit_msg);
    map.insert(format!("commit_ID_A"), integer_to_field(&commit_ID_A)); 

    map
}

pub fn prover_input_sha_preprocess(x: String) -> HashMap<String, Value> {
    let contents = read_lines(&x);
    let mut map = HashMap::<String, Value>::default();
    let ID_A_trimmed = &contents[0][1..contents[0].len() - 1];
    let ID_A_items: Vec<&str> = ID_A_trimmed.split(',').collect();
    
    for (index, word) in ID_A_items.clone().iter().enumerate()  {
        map.insert(format!("ID_A.{}", index), str_to_field(&word.clone()));
    }
    
    let N_ID_A_trimmed = &contents[1][1..contents[1].len() - 1];
    let N_ID_A_items: Vec<&str> = N_ID_A_trimmed.split(',').collect();
    
    for (index, word) in N_ID_A_items.iter().enumerate()  {
        map.insert(format!("N_ID_A.{}", index), str_to_field(&word.clone()));
    }

    let salt_trimmed = &contents[2][1..contents[2].len() - 1];

    let salt_items: Vec<&str> = salt_trimmed.split(',').collect();
    
    for (index, word) in salt_items.iter().enumerate()  {
        map.insert(format!("salt.{}", index), str_to_field(&word.clone()));
    }


    let EM_trimmed = &contents[3][1..contents[3].len() - 1];
    let EM_items: Vec<&str> = EM_trimmed.split(',').collect();
    
    for (index, word) in EM_items.iter().enumerate()  {
        map.insert(format!("EM.{}", index), str_to_field(&word.clone()));
    }

    let blinder_trimmed = &contents[4][1..contents[4].len() - 1];
    let blinder_items: Vec<&str> = blinder_trimmed.split(',').collect();

    for (index, word) in blinder_items.iter().enumerate()  {
        map.insert(format!("blinder.{}", index), str_to_field(&word.clone()));
    }

    let blinded_trimmed = &contents[5][1..contents[5].len() - 1];
    let blinded_items: Vec<&str> = blinded_trimmed.split(',').collect();

    for (index, word) in blinded_items.iter().enumerate()  {
        map.insert(format!("blinded.{}", index), str_to_field(&word.clone()));
    }
    
    let ID_B_trimmed = &contents[6][1..contents[6].len() - 1];
    let ID_B_items: Vec<&str> = ID_B_trimmed.split(',').collect();
    
    for (index, word) in ID_B_items.iter().enumerate()  {
        map.insert(format!("ID_B.{}", index), str_to_field(&word.clone()));
    }
    
    let N_H_trimmed = &contents[7][1..contents[7].len() - 1];
    let N_H_items: Vec<&str> = N_H_trimmed.split(',').collect();
    
    for (index, word) in N_H_items.iter().enumerate()  {
        map.insert(format!("N_H.{}", index), str_to_field(&word.clone()));
    }
        
    let H_r = &contents[8];
    map.insert(format!("H_r"), str_to_field(&H_r.clone()));

    let H_trimmed = &contents[9][1..contents[9].len() - 1];
    let H_items: Vec<&str> = H_trimmed.split(',').collect();
    
    for (index, word) in H_items.iter().enumerate()  {
        map.insert(format!("H.{}", index), str_to_field(&word.clone()));
    } 

    let N_old_token_trimmed = &contents[10][1..contents[10].len() - 1];
    let N_old_token_items: Vec<&str> = N_old_token_trimmed.split(',').collect();
    
    for (index, word) in N_old_token_items.iter().enumerate()  {
        map.insert(format!("N_old_token.{}", index), str_to_field(&word.clone()));
    }

    let old_token_trimmed = &contents[11][1..contents[11].len() - 1];
    let old_token_items: Vec<&str> = old_token_trimmed.split(',').collect();
    
    for (index, word) in old_token_items.iter().enumerate()  {
        map.insert(format!("old_token.{}", index), str_to_field(&word.clone()));
    }

    let mut rng = RandState::new_mersenne_twister();
    //for proof gen testing - so deterministic for now:
    rng.seed(&Integer::from(7));
    let mut commit_rand = rand_int(&mut rng);
    map.insert(format!("commit_rand"), integer_to_field(&commit_rand));

    let u8_ID_A_items: Vec<u8> = ID_A_items.iter()
    .map(|s| s.trim().parse::<u8>().unwrap())
    .collect();

    let mut unpacked: Vec<Integer> = u8_ID_A_items.iter().map(|&byte| Integer::from(byte)).collect();
    let mut packed = pack_vector(unpacked, 248,8);
    let commit_msg = vec![commit_rand, packed[0].clone()];
    let commit_ID_A = poseidon_hash(commit_msg);
    map.insert(format!("commit_ID_A"), integer_to_field(&commit_ID_A));

    map 
}

/// generate verifier input for run_zk_example
pub fn verifier_input_sha_preprocess(x: String) -> HashMap<String, Value> {
    let contents = read_lines(&x);
    let mut map = HashMap::<String, Value>::default();

    let blinded_trimmed = &contents[5][1..contents[5].len() - 1];
    let blinded_items: Vec<&str> = blinded_trimmed.split(',').collect();
    
    for (index, word) in blinded_items.iter().enumerate()  {
        map.insert(format!("blinded.{}", index), str_to_field(&word.clone()));
    }
    
    let H_trimmed = &contents[9][1..contents[9].len() - 1];
    let H_items: Vec<&str> = H_trimmed.split(',').collect();
    
    for (index, word) in H_items.iter().enumerate()  {
        map.insert(format!("H.{}", index), str_to_field(&word.clone()));
    } 

    let old_token_trimmed = &contents[11][1..contents[11].len() - 1];
    let old_token_items: Vec<&str> = old_token_trimmed.split(',').collect();
    
    for (index, word) in old_token_items.iter().enumerate()  {
        map.insert(format!("old_token.{}", index), str_to_field(&word.clone()));
    }

    let ID_A_trimmed = &contents[0][1..contents[0].len() - 1];
    let ID_A_items: Vec<&str> = ID_A_trimmed.split(',').collect();

    let u8_ID_A_items: Vec<u8> = ID_A_items.iter()
    .map(|s| s.trim().parse::<u8>().unwrap())
    .collect();

    let mut unpacked: Vec<Integer> = u8_ID_A_items.iter().map(|&byte| Integer::from(byte)).collect();
    let mut packed = pack_vector(unpacked, 248,8);
    
    let mut rng = RandState::new_mersenne_twister();
    rng.seed(&Integer::from(7));
    let mut commit_rand = rand_int(&mut rng);
    let commit_msg = vec![commit_rand, packed[0].clone()];
    let commit_ID_A = poseidon_hash(commit_msg);
    map.insert(format!("commit_ID_A"), integer_to_field(&commit_ID_A)); 

    map
}

pub fn prover_input_token_revocation(x: String) -> HashMap<String, Value> {
    let contents = read_lines(&x);
    let mut map = HashMap::<String, Value>::default();

    let left_leaf = &contents[0];
    map.insert(format!("left_leaf"), str_to_field(&left_leaf.clone()));

    let direction_left_trimmed = &contents[1][1..contents[1].len() - 1];
    let direction_left_items: Vec<&str> = direction_left_trimmed.split(',').collect();
    
    for (index, word) in direction_left_items.clone().iter().enumerate()  {
        map.insert(format!("direction_left.{}", index), str_to_field(&word.clone()));
    }

    let auth_path_left_trimmed = &contents[2][1..contents[2].len() - 1];
    let auth_path_left_items: Vec<&str> = auth_path_left_trimmed.split(',').collect();
    
    for (index, word) in auth_path_left_items.clone().iter().enumerate()  {
        map.insert(format!("auth_path_left.{}", index), str_to_field(&word.clone()));
    }

    let right_leaf = &contents[3];
    map.insert(format!("right_leaf"), str_to_field(&right_leaf.clone()));

    let direction_right_trimmed = &contents[4][1..contents[4].len() - 1];
    let direction_right_items: Vec<&str> = direction_right_trimmed.split(',').collect();
    
    for (index, word) in direction_right_items.clone().iter().enumerate()  {
        map.insert(format!("direction_right.{}", index), str_to_field(&word.clone()));
    }

    let auth_path_right_trimmed = &contents[5][1..contents[5].len() - 1];
    let auth_path_right_items: Vec<&str> = auth_path_right_trimmed.split(',').collect();
    
    for (index, word) in auth_path_right_items.clone().iter().enumerate()  {
        map.insert(format!("auth_path_right.{}", index), str_to_field(&word.clone()));
    }

    let sub_root= &contents[6];
    map.insert(format!("sub_root"), str_to_field(&sub_root.clone()));

    let ID_A_trimmed = &contents[7][1..contents[7].len() - 1];
    let ID_A_items: Vec<&str> = ID_A_trimmed.split(',').collect();
    
    for (index, word) in ID_A_items.clone().iter().enumerate()  {
        map.insert(format!("ID_A.{}", index), str_to_field(&word.clone()));
    }

    let direction_root_trimmed = &contents[8][1..contents[8].len() - 1];
    let direction_root_items: Vec<&str> = direction_root_trimmed.split(',').collect();
    
    for (index, word) in direction_root_items.clone().iter().enumerate()  {
        map.insert(format!("direction_root.{}", index), str_to_field(&word.clone()));
    }

    let auth_path_root_trimmed = &contents[9][1..contents[9].len() - 1];
    let auth_path_root_items: Vec<&str> = auth_path_root_trimmed.split(',').collect();
    
    for (index, word) in auth_path_root_items.clone().iter().enumerate()  {
        map.insert(format!("auth_path_root.{}", index), str_to_field(&word.clone()));
    }

    let root= &contents[10];
    map.insert(format!("root"), str_to_field(&root.clone()));

    let mut rng = RandState::new_mersenne_twister();
    //for proof gen testing - so deterministic for now:
    rng.seed(&Integer::from(7));
    let mut commit_rand = rand_int(&mut rng);
    map.insert(format!("commit_rand"), integer_to_field(&commit_rand));

    let u8_ID_A_items: Vec<u8> = ID_A_items.iter()
    .map(|s| s.trim().parse::<u8>().unwrap())
    .collect();

    let mut unpacked: Vec<Integer> = u8_ID_A_items.iter().map(|&byte| Integer::from(byte)).collect();
    let mut packed = pack_vector(unpacked, 248,8);
    let commit_msg = vec![commit_rand, packed[0].clone()];
    let commit_ID_A = poseidon_hash(commit_msg);
    map.insert(format!("commit_ID_A"), integer_to_field(&commit_ID_A));

    map 
}

/// generate verifier input for run_zk_example
pub fn verifier_input_token_revocation(x: String) -> HashMap<String, Value> {
    let contents = read_lines(&x);
    let mut map = HashMap::<String, Value>::default();

    let root= &contents[10];
    map.insert(format!("root"), str_to_field(&root.clone()));

    let mut rng = RandState::new_mersenne_twister();
    //for proof gen testing - so deterministic for now:
    rng.seed(&Integer::from(7));
    let mut commit_rand = rand_int(&mut rng);
    // map.insert(format!("commit_rand"), integer_to_field(&commit_rand));

    let ID_A_trimmed = &contents[7][1..contents[7].len() - 1];
    let ID_A_items: Vec<&str> = ID_A_trimmed.split(',').collect();

    let u8_ID_A_items: Vec<u8> = ID_A_items.iter()
    .map(|s| s.trim().parse::<u8>().unwrap())
    .collect();

    let mut unpacked: Vec<Integer> = u8_ID_A_items.iter().map(|&byte| Integer::from(byte)).collect();
    let mut packed = pack_vector(unpacked, 248,8);
    let commit_msg = vec![commit_rand, packed[0].clone()];
    let commit_ID_A = poseidon_hash(commit_msg);
    map.insert(format!("commit_ID_A"), integer_to_field(&commit_ID_A));

    map 
}
