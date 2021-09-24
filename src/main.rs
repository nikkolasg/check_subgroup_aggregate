use bellperson::groth16::aggregate::AggregateProof;
use blstrs::Bls12;
use env_logger::Env;
use std::io::{self, BufRead, BufReader, Cursor};
#[macro_use]
extern crate log;
extern crate base64;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("error")).init();
    let reader = BufReader::new(io::stdin());
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let fields = line.split(" ").collect::<Vec<_>>();
        let (proof, code) = (fields[0], fields[1]);
        let exit_code = code.parse::<i32>().unwrap();
        let proof_decoded = base64::decode(proof).unwrap();
        match AggregateProof::<Bls12>::read(Cursor::new(&proof_decoded)) {
            Ok(_) => debug!("proof {} (exit {}) parsed correctly", index + 1, exit_code),
            Err(e) => {
                if exit_code == 0 {
                    error!("proof {} INVALID but exit 0!", index + 1);
                } else {
                    debug!(
                        "proof {} (exit {}) parsed INVALID: {}",
                        index + 1,
                        exit_code,
                        e
                    )
                }
            }
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
