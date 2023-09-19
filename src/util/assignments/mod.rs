use crate::rsa::{fermat, get_rsa_data, decode, encode};

use super::{ttablegen::generate_truth_table, notation::{LOGICAL_AND, NOT, LOGICAL_OR}};



pub(crate) fn assignment1() {
    generate_truth_table(
        vec!["P".to_string(), "Q".to_string(), "R".to_string()],
        vec![
            (format!("P {} Q", LOGICAL_AND), Box::new(|x| x[0] && x[1])),
            (format!("P {} R", LOGICAL_AND), Box::new(|x| x[0] && x[2])),
            (format!("Q {} R", LOGICAL_AND), Box::new(|x| x[1] && x[2])),
            (format!("P {} Q {} R", LOGICAL_AND, LOGICAL_AND), Box::new(|x| x[0] && x[1] && x[2])),
            (format!("P {} (Q {} R)", LOGICAL_AND, LOGICAL_AND), Box::new(|x| x[0] && (x[1] && x[2]))),
            (format!("(P {} Q) {} R", LOGICAL_AND, LOGICAL_AND), Box::new(|x| (x[0] && x[1]) && x[2])),

        ]
    );
    // disprove affirming a disjunct
    generate_truth_table(
        vec!["P".to_string(), "Q".to_string()], 
        vec![
            (format!("P {} Q", LOGICAL_OR), Box::new(|x| x[0] || x[1])),
            (format!("P {} ({} Q)", LOGICAL_AND, NOT), Box::new(|x| !x[1])),
        ]
    )
}

pub(crate) fn assignment2() { 

    let rsa_data = get_rsa_data(103u64, 409u64);
    let to_decrypt = vec![
        1208, 4069, 4694, 34313, 474, 19705, 31805, 3779, 21340, 40789, 34313, 15073, 19705, 19121, 4069, 41336, 8677u64
    ];
    let decrypted = decode(to_decrypt, rsa_data.privk);
    dbg!(decrypted);

    let to_encrypt = String::from("christmas");
    let encrypted = encode(to_encrypt, (7, 60491));
    dbg!(encrypted);

    println!("proof: the universe spoke to me directly");
    println!("i know this probably doesnt fit everything thats desired out of this assignment but my brain is really foggy and i cant think much right now");
}