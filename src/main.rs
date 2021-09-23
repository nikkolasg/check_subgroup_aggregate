use bitfield::UnvalidatedBitField;
use encoding::{serde_bytes, tuple::*, BytesDe};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct ProveCommitAggregateParams {
    pub sector_numbers: UnvalidatedBitField,
    #[serde(with = "serde_bytes")]
    pub aggregate_proof: Vec<u8>,
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
