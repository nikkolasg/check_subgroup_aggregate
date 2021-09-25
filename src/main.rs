use bellperson::groth16::aggregate::AggregateProof;
use blstrs::Bls12;
use env_logger::Env;
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use std::io::{self, BufRead, BufReader, Cursor};
#[macro_use]
extern crate log;
extern crate base64;
extern crate num_cpus;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("error")).init();
    let reader = BufReader::new(io::stdin());
    let inc = 1000;
    let mut invalid = 0;

    invalid = reader
        .lines()
        .skip(1)
        .enumerate()
        .par_bridge()
        .map(|(index, line)| {
            let line = line.unwrap();
            let fields = line.split(",").collect::<Vec<_>>();
            let (proof, code) = (fields[0], fields[1]);
            let exit_code = match code.parse::<i32>() {
                Ok(e) => e,
                Err(err) => panic!("error unwrapping exit code {} -> {}", code, err),
            };
            let proof_decoded = base64::decode(proof).unwrap();
            match AggregateProof::<Bls12>::read(Cursor::new(&proof_decoded)) {
                Ok(_) => {
                    debug!("proof {} (exit {}) parsed correctly", index + 1, exit_code);
                    0
                }
                Err(e) => {
                    if exit_code == 0 {
                        error!("proof {} INVALID but exit 0!", index + 1);
                        1
                    } else {
                        debug!(
                            "proof {} (exit {}) parsed INVALID: {}",
                            index + 1,
                            exit_code,
                            e
                        );
                        0
                    }
                }
            }
        })
        .sum();
    println!(
        "Number of invalid 'compressed GT' proofs found = {}",
        invalid
    );
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
