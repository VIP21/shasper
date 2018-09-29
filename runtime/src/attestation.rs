use primitives::{H256, U256};
use rstd::prelude::*;

use bitfield::BitField;

#[derive(Clone, PartialEq, Eq, Decode, Encode, SszEncode, SszDecode)]
#[cfg_attr(feature = "std", derive(Debug))]
#[ssz_codec(sorted)]
pub struct AttestationRecord {
	pub slot: u64,
	pub shard_id: u16,
	pub oblique_parent_hashes: Vec<H256>,
	pub shard_block_hash: H256,
	pub attester_bitfield: BitField,
	pub justified_slot: u64,
	pub justified_block_hash: H256,
	pub aggregate_sig: Vec<U256>,
}

impl Default for AttestationRecord {
	fn default() -> Self {
		Self {
			slot: 0,
			shard_id: 0,
			oblique_parent_hashes: Vec::new(),
			shard_block_hash: H256::new(),
			attester_bitfield: BitField::new(0),
			justified_slot: 0,
			justified_block_hash: H256::new(),
			aggregate_sig: {
				let mut ret = Vec::new();
				ret.push(U256::zero());
				ret.push(U256::zero());
				ret
			},
		}
	}
}
