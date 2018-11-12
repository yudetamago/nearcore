use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::PartialOrd;

use super::types;

pub trait Verifier {
    fn compute_state(&mut self, transactions: &[types::StatedTransaction]) -> types::State;
}

pub trait WitnessSelector {
    fn epoch_witnesses(&self, epoch: u64) -> &HashSet<u64>;
    fn epoch_leader(&self, epoch: u64) -> u64;
}

pub trait Payload: Hash {
    fn verify(&self) -> Result<(), &'static str>;
}

pub trait VerifiableDelayFunction: Hash {
    type InputType: Hash + Copy;
    type OutputType: Hash + Copy;

    fn compute(inp: &Self::InputType) -> Self::OutputType;
    fn verify(inp: &Self::InputType, outp: &Self::OutputType) -> bool;

    fn from_hashable(hashable: impl Hash) -> Self::InputType;
}

pub trait ScoredBlockChainHead: Hash {
    type Score: PartialOrd + Copy;

    fn get_score(&self) -> Self::Score;
}

pub trait MultiSigScheme: Hash {
    type CommitMessage: Hash;
    type SignatureMessage: Hash;
}
