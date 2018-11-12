use primitives::traits::{VerifiableDelayFunction, ScoredBlockChainHead, Payload, WitnessSelector, MultiSigScheme};
use super::{dag::DAG, message::Message};
use std::collections::{HashMap, HashSet};
use typed_arena::Arena;

const VDF_THRESHOLD_NOM: u32 = 1;
const VDF_THRESHOLD_DENOM: u32 = 2;

/// Payload that goes into txflow Message
#[derive(Hash)]
enum BeaconChainPayload<P: Payload, VDF: VerifiableDelayFunction, MS: MultiSigScheme, BCH: ScoredBlockChainHead> {
    PrevBlock(BCH),
    Regular(P),
    CommitWithVDF(VDF::OutputType, MS::CommitMessage),
    Commit(MS::CommitMessage),
    Signature(MS::SignatureMessage),
}

// A message is a commit if it either contains a VDF output, or approves some (configurable)
// percentage of VDFs. Seeing 2/3 of commits makes a message `is_commit_intermediate`
// Seeing 2/3 of intermediate messages increases the intermediate step until a predefined step
// is reached. Seeing 2/3 of those makes the message `is_signature`. Similar rules apply to
// `is_signature_intermediate` and `is_final`
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
enum Stage {
    None,
    Initiate,
    InitiateIntermediate(u32),
    PreCommit,
    Commit,
    CommitIntermediate(u32),
    Signature,
    SignatureIntermediate(u32),
    Final
}

struct MessageInfo<'a, BCH: 'a + ScoredBlockChainHead> {
    is_initialized: bool,
    stage: Stage,
    is_first_in_stage: bool,
    stage_validators: HashSet<u64>,
    user_stages: HashMap<u64, Stage>,
    prev_block: Option<&'a BCH>,
}

struct BeaconChainConsensus<'a, VDF: 'a + VerifiableDelayFunction, MS: 'a + MultiSigScheme, BCH: 'a + ScoredBlockChainHead, P: 'a + Payload, W: 'a + WitnessSelector> {
    txflow: DAG<'a, BeaconChainPayload<P, VDF, MS, BCH>, W>,
    num_steps: u32,
    message_infos: HashMap<u64, MessageInfo<'a, BCH>>,
}

impl<'a, BCH: 'a + ScoredBlockChainHead> MessageInfo<'a, BCH> {
    fn new() -> Self {
        Self {
            is_initialized: false,
            stage: Stage::None,
            is_first_in_stage: false,
            stage_validators: HashSet::new(),
            user_stages: HashMap::new(),
            prev_block: None,
        }
    }

    fn compute_vdf_input<VDF>(&self) -> VDF::InputType
        where VDF: VerifiableDelayFunction
    {

        match self.prev_block {
            Some(some) => VDF::from_hashable(some),
            None => VDF::from_hashable(0),
        }
    }

    fn ready_to_increment_stage(&self, num_steps: u32, num_witnesses: u32, is_representative: bool, precommit_callback: &Fn(&Self) -> bool) -> bool {
        let has_two_third_support_fn = || self.stage_validators.len() as u32 > num_witnesses * 2 / 3;
        let has_two_third_support_or_representative_fn = |step: u32| {
            if step < num_steps {
                has_two_third_support_fn()
            }
            else {
                is_representative
            }
        };

        match self.stage {
            Stage::None => true,
            Stage::Initiate => has_two_third_support_fn(),
            Stage::InitiateIntermediate(step) => has_two_third_support_or_representative_fn(step),
            Stage::PreCommit => precommit_callback(self),
            Stage::Commit => has_two_third_support_fn(),
            Stage::CommitIntermediate(step) => has_two_third_support_or_representative_fn(step),
            Stage::Signature => has_two_third_support_fn(),
            Stage::SignatureIntermediate(step) => has_two_third_support_or_representative_fn(step),
            Stage::Final => false,
        }
    }

    fn verify_first_message_in_stage<P, VDF, MS>(&self, payload: &BeaconChainPayload<P, VDF, MS, BCH>) -> Result<(), &'static str>
        where P: Payload, VDF: VerifiableDelayFunction, MS: MultiSigScheme
    {
        match self.stage {
            Stage::Initiate => {
                if let BeaconChainPayload::PrevBlock(_) = payload {
                    Ok(())
                } else {
                    Err("First Stage::Initiate message payload is not Payload::PrevBlock")
                }
            },
            Stage::Commit => {
                match payload {
                    BeaconChainPayload::Commit(_) => Ok(()),
                    BeaconChainPayload::CommitWithVDF(_, _) => Ok(()),
                    _ => Err("First Stage::Commit message payload is not Payload::Commit or Payload::CommitWithVDF")
                }
            },
            Stage::Signature => {
                if let BeaconChainPayload::Signature(_) = payload {
                    Ok(())
                } else {
                    Err("First Stage::Signature message payload is not Payload::Signature")
                }
            },
            _ =>  Ok(()),
        }
    }

    fn increment_stage(&mut self, num_steps: u32) {
        self.stage = match self.stage {
            Stage::None => Stage::Initiate,
            Stage::Initiate => Stage::InitiateIntermediate(1),
            Stage::InitiateIntermediate(step) => {
                if step < num_steps { Stage::InitiateIntermediate(step + 1) }
                else { Stage::PreCommit }
            },
            Stage::PreCommit => Stage::Commit,
            Stage::Commit => Stage::CommitIntermediate(1),
            Stage::CommitIntermediate(step) => {
                if step < num_steps { Stage::CommitIntermediate(step + 1) }
                else { Stage::Signature }
            },
            Stage::Signature => Stage::SignatureIntermediate(1),
            Stage::SignatureIntermediate(step) => {
                if step < num_steps { Stage::SignatureIntermediate(step + 1) }
                else { Stage::Final }
            },
            Stage::Final => Stage::Final,
        };
        self.is_first_in_stage = true;
    }

    fn merge_message_info_in(&mut self, other: &Self) {
        if self.prev_block.is_none() || !other.prev_block.is_none() && self.prev_block.unwrap().get_score() < other.prev_block.unwrap().get_score() {
            self.prev_block = other.prev_block;
        }

        if self.stage < other.stage {
            self.stage_validators = other.stage_validators.clone();
            self.stage = other.stage;
        }
        else if self.stage == other.stage {
            self.stage_validators.extend(&other.stage_validators);
        }

        for (k, v) in other.user_stages.iter() {
            if !self.user_stages.contains_key(&k) || self.user_stages.get(&k).unwrap() < &v {
                self.user_stages.insert(*k, *v);
            }
        }
    }
}

impl<'a, VDF: VerifiableDelayFunction, MS: 'a + MultiSigScheme, BCH: 'a + ScoredBlockChainHead, P: 'a + Payload, W: 'a + WitnessSelector> BeaconChainConsensus<'a, VDF, MS, BCH, P, W> {
    fn new(arena: &'a Arena<Message<'a, BeaconChainPayload<P, VDF, MS, BCH>>>, owner_uid: u64, witness_selector: &'a W, num_steps: u32) -> Self {
        Self {
            txflow: DAG::new(
                arena,
                owner_uid,
                0,
                witness_selector,
            ),
            num_steps,
            message_infos: HashMap::new(),
        }
    }

    fn process_message(&mut self, msg: &'a Message<BeaconChainPayload<P, VDF, MS, BCH>>, witness_selector: &W) -> Result<(), &'static str> {
        let mut message_info = MessageInfo::new();
        let owner_uid = msg.data.body.owner_uid;
        let num_witnesses = witness_selector.epoch_witnesses(msg.computed_epoch).len() as u32;

        for parent in &msg.parents {
            message_info.merge_message_info_in(self.message_infos.get(&parent.computed_hash).unwrap())
        }

        debug_assert!(!message_info.user_stages.contains_key(&owner_uid) || &message_info.stage >= message_info.user_stages.get(&owner_uid).unwrap());
        message_info.user_stages.insert(owner_uid, message_info.stage);

        message_info.stage_validators.insert(owner_uid);

        let precommit_callback = |message_info: &MessageInfo<BCH>| {
            // A message is ready to move from Precommit to Commit if it either contains a VDF
            //    output, or approves more than a certain number of VDFs
            assert_eq!(message_info.stage, Stage::PreCommit);

            if let BeaconChainPayload::CommitWithVDF(output, _) = &msg.data.body.payload {
                VDF::verify(&message_info.compute_vdf_input::<VDF>(), output)
            } else {
                message_info.stage_validators.len() as u32 > num_witnesses * VDF_THRESHOLD_NOM / VDF_THRESHOLD_DENOM
            }
        };

        let mut expect_prev_block = false;

        if message_info.ready_to_increment_stage(self.num_steps, num_witnesses, msg.computed_is_representative, &precommit_callback) {
            message_info.increment_stage(self.num_steps);

            message_info.verify_first_message_in_stage(&msg.data.body.payload)?;
        }

        match &msg.data.body.payload {
            BeaconChainPayload::PrevBlock(bch) => {
                if message_info.prev_block.is_none() || bch.get_score() > message_info.prev_block.unwrap().get_score() {
                    message_info.prev_block = Some(bch);
                }
            }
            BeaconChainPayload::Regular(_) => {},
            BeaconChainPayload::CommitWithVDF(output, _) => {
                if !VDF::verify(&message_info.compute_vdf_input::<VDF>(), output) {
                    return Err("Incorrect VDF output");
                }
            },
            BeaconChainPayload::Commit(_) => {},
            BeaconChainPayload::Signature(_) => {},
        }

        message_info.is_initialized = true;

        self.message_infos.insert(msg.computed_hash, message_info);

        Ok({})
    }
}

impl<P: Payload, VDF: VerifiableDelayFunction, MS: MultiSigScheme, BCH: ScoredBlockChainHead> Payload for BeaconChainPayload<P, VDF, MS, BCH> {
    fn verify(&self) -> Result<(), &'static str> {
        if let BeaconChainPayload::Regular(sub_payload) = &self {
            sub_payload.verify()
        }
        else {
            Ok(())
        }
    }
}
