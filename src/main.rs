//use bitfield::UnvalidatedBitField;
//use encoding::{de, from_slice, ser, to_vec, Cbor, Error as EncodingError};
//use encoding::{de::Deserialize, serde_bytes, tuple::*, BytesDe};
use csv::ReaderBuilder;
//use serde_derive::{Deserialize, Serialize};
use std::io::{self, BufRead, BufReader, Cursor, Read};
extern crate base64;

//use bellperson::bls::Bls12;
use bellperson::groth16::aggregate::AggregateProof;
use blstrs::{Bls12, Compress};

fn main() {
    let reader = BufReader::new(io::stdin());
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let fields = line.split(" ").collect::<Vec<_>>();
        let (proof, exitcode) = (fields[0], fields[1]);
        let proof_decoded = base64::decode(proof).unwrap();
        match AggregateProof::<Bls12>::read(Cursor::new(&proof_decoded)) {
            Ok(_) => println!("proof {} (exit {}) parsed correctly", index + 1, exitcode),
            Err(e) => println!(
                "proof {} (exit {}) parsed INVALID: {}",
                index + 1,
                exitcode,
                e
            ),
        }
    }
}

#[cfg(test)]
mod test {
    //use actor::builtin::miner::types::ProveCommitAggregateParams;
    #[test]
    fn testou() {
        println!("this is true");
        assert!(true);
    }
}
