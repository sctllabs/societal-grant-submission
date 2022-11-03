#![cfg_attr(not(feature = "std"), no_std)]

use codec::MaxEncodedLen;
use frame_support::codec::{Decode, Encode};
pub use node_primitives::Balance;

use scale_info::TypeInfo;
use serde::{self, Deserialize, Deserializer, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

#[derive(
	Encode, Decode, Default, Clone, PartialEq, TypeInfo, RuntimeDebug, Serialize, Deserialize,
)]
pub struct DaoTokenMetadata {
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub name: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub symbol: Vec<u8>,
	pub decimals: u8,
}

#[derive(
	Encode, Decode, Default, Clone, PartialEq, TypeInfo, RuntimeDebug, Serialize, Deserialize,
)]
pub struct DaoGovernanceToken {
	pub token_id: u32,
	pub metadata: DaoTokenMetadata,
	#[serde(deserialize_with = "de_string_to_u128")]
	pub min_balance: u128,
}

#[derive(
	Encode, Decode, Default, Clone, PartialEq, TypeInfo, RuntimeDebug, Serialize, Deserialize,
)]
pub struct DaoPolicyPayload {
	pub proposal_bond: u32,
	pub proposal_bond_min: u128,
	pub proposal_period: u32,
}

#[derive(
	Encode, Decode, Default, Clone, PartialEq, TypeInfo, RuntimeDebug, Serialize, Deserialize,
)]
pub struct DaoPayload {
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub name: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub purpose: Vec<u8>,
	#[serde(deserialize_with = "de_string_to_bytes")]
	pub metadata: Vec<u8>,
	pub token: Option<DaoGovernanceToken>,
	pub token_id: Option<u32>,
	pub policy: DaoPolicyPayload,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct DaoConfig<BoundedString, BoundedMetadata> {
	/// Name of the DAO.
	pub name: BoundedString,
	/// Purpose of this DAO.
	pub purpose: BoundedString,
	/// Generic metadata. Can be used to store additional data.
	pub metadata: BoundedMetadata,
}

#[derive(
	Encode,
	Decode,
	Default,
	Clone,
	PartialEq,
	TypeInfo,
	RuntimeDebug,
	Serialize,
	Deserialize,
	MaxEncodedLen,
)]
pub struct DaoPolicy<AccountId> {
	/// Fraction of a proposal's value that should be bonded in order to place the proposal.
	/// An accepted proposal gets these back. A rejected proposal does not.
	pub proposal_bond: u32, //TODO: static value or percentage???
	/// Minimum amount of funds that should be placed in a deposit for making a proposal.
	pub proposal_bond_min: u128,
	/// Maximum amount of funds that should be placed in a deposit for making a proposal.
	pub proposal_bond_max: Option<u128>,
	/// In millis
	pub proposal_period: u32,
	//TODO: ??
	pub prime_account: AccountId,
	// TODO: use max members for account length
	pub approve_origin: (u32, u32),
	pub reject_origin: (u32, u32),
}

#[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
pub struct Dao<AccountId, TokenId, BoundedString, BoundedMetadata> {
	pub founder: AccountId,
	pub account_id: AccountId,
	pub token_id: TokenId,
	pub config: DaoConfig<BoundedString, BoundedMetadata>,
}

pub trait DaoProvider {
	type Id;
	type AccountId;
	type Policy;

	fn exists(id: Self::Id) -> bool;
	fn dao_account_id(id: Self::Id) -> Self::AccountId;
	fn policy(id: Self::Id) -> Option<Self::Policy>;
	fn count() -> u32;
}

pub trait CouncilProvider<DaoId, AccountId> {
	fn initialize_members(dao_id: DaoId, members: &[AccountId]);
}

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: &str = Deserialize::deserialize(de)?;
	Ok(s.as_bytes().to_vec())
}

pub fn de_string_to_u128<'de, D>(de: D) -> Result<u128, D::Error>
where
	D: Deserializer<'de>,
{
	let s: &str = Deserialize::deserialize(de)?;
	Ok(s.parse::<u128>().unwrap())
}
