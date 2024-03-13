#![allow(non_snake_case)]
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::Serialize;
use sp_core::{RuntimeDebug, H256, U256};
use sp_runtime::SaturatedConversion;
use sp_std::vec::Vec;

#[repr(u8)]
#[derive(
	Copy, Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, MaxEncodedLen, Serialize,
)]
pub enum L1 {
	Ethereum,
}

#[repr(u8)]
#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize, Copy)]
pub enum Origin {
	L1,
	L2,
}

impl Default for Origin {
	fn default() -> Self {
		Origin::L1
	}
}

impl Into<eth_abi::Origin> for Origin {
	fn into(self) -> eth_abi::Origin {
		match self {
			Origin::L1 => eth_abi::Origin::L1,
			Origin::L2 => eth_abi::Origin::L2,
		}
	}
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize)]
pub struct Range {
	pub start: u128,
	pub end: u128,
}

impl From<(u128, u128)> for Range {
	fn from((start, end): (u128, u128)) -> Range {
		Range { start, end }
	}
}

impl Into<eth_abi::Range> for Range {
	fn into(self) -> eth_abi::Range {
		eth_abi::Range { start: to_eth_u256(self.start.into()), end: to_eth_u256(self.end.into()) }
	}
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize, Default)]
pub struct RequestId {
	pub origin: Origin,
	pub id: u128,
}

impl RequestId {
	pub fn new(origin: Origin, id: u128) -> Self {
		Self { origin, id }
	}
}

impl From<(Origin, u128)> for RequestId {
	fn from((origin, id): (Origin, u128)) -> RequestId {
		RequestId { origin, id }
	}
}

impl Into<eth_abi::RequestId> for RequestId {
	fn into(self) -> eth_abi::RequestId {
		eth_abi::RequestId { origin: self.origin.into(), id: to_eth_u256(U256::from(self.id)) }
	}
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize, Default)]
pub struct Deposit {
	pub requestId: RequestId,
	pub depositRecipient: [u8; 20],
	pub tokenAddress: [u8; 20],
	pub amount: sp_core::U256,
	pub blockHash: sp_core::H256,
}

impl Into<eth_abi::Deposit> for Deposit {
	fn into(self) -> eth_abi::Deposit {
		eth_abi::Deposit {
			requestId: self.requestId.into(),
			depositRecipient: self.depositRecipient.into(),
			tokenAddress: self.tokenAddress.into(),
			amount: to_eth_u256(self.amount),
			blockHash: alloy_primitives::FixedBytes::<32>::from_slice(&self.blockHash[..]),
		}
	}
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Default, Serialize)]
#[allow(non_snake_case)]
pub struct Withdraw {
	pub depositRecipient: [u8; 20],
	pub tokenAddress: [u8; 20],
	pub amount: sp_core::U256,
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize)]
pub struct L2UpdatesToRemove {
	pub requestId: RequestId,
	pub l2UpdatesToRemove: Vec<u128>,
	pub blockHash: sp_core::H256,
}

impl Into<eth_abi::L2UpdatesToRemove> for L2UpdatesToRemove {
	fn into(self) -> eth_abi::L2UpdatesToRemove {
		eth_abi::L2UpdatesToRemove {
			requestId: self.requestId.into(),
			l2UpdatesToRemove: self
				.l2UpdatesToRemove
				.into_iter()
				.map(|rid| to_eth_u256(rid.into()))
				.collect(),
			blockHash: alloy_primitives::FixedBytes::<32>::from_slice(&self.blockHash[..]),
		}
	}
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize)]
pub struct CancelResolution {
	pub requestId: RequestId,
	pub l2RequestId: u128,
	pub cancelJustified: bool,
	pub blockHash: sp_core::H256,
}

impl Into<eth_abi::CancelResolution> for CancelResolution {
	fn into(self) -> eth_abi::CancelResolution {
		eth_abi::CancelResolution {
			requestId: self.requestId.into(),
			l2RequestId: to_eth_u256(self.l2RequestId.into()),
			cancelJustified: self.cancelJustified.into(),
			blockHash: alloy_primitives::FixedBytes::<32>::from_slice(&self.blockHash[..]),
		}
	}
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Default, Serialize)]
pub struct WithdrawalResolution {
	pub requestId: RequestId,
	pub l2RequestId: u128,
	pub status: bool,
	pub blockHash: H256,
}

impl Into<eth_abi::WithdrawalResolution> for WithdrawalResolution {
	fn into(self) -> eth_abi::WithdrawalResolution {
		eth_abi::WithdrawalResolution {
			requestId: self.requestId.into(),
			l2RequestId: to_eth_u256(self.l2RequestId.into()),
			status: self.status.into(),
			blockHash: alloy_primitives::FixedBytes::<32>::from_slice(&self.blockHash[..]),
		}
	}
}


#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Default, Serialize)]
pub struct L1Update {
	pub pendingDeposits: Vec<Deposit>,
	pub pendingCancelResultions: Vec<CancelResolution>,
    pub pendingWithdrawalResolutions: Vec<WithdrawalResolution>,
	pub pendingL2UpdatesToRemove: Vec<L2UpdatesToRemove>,
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize)]
pub enum L1UpdateRequest {
	Deposit(Deposit),
	CancelResolution(CancelResolution),
	Remove(L2UpdatesToRemove),
	WithdrawalResolution(WithdrawalResolution)
}

impl L1UpdateRequest {
	pub fn request_id(&self) -> RequestId {
		match self {
			L1UpdateRequest::Deposit(deposit) => deposit.requestId.clone(),
			L1UpdateRequest::CancelResolution(cancel) => cancel.requestId.clone(),
			L1UpdateRequest::Remove(remove) => remove.requestId.clone(),
			L1UpdateRequest::WithdrawalResolution(withdrawal) => withdrawal.requestId.clone(),
		}
	}

	pub fn id(&self) -> u128 {
		match self {
			L1UpdateRequest::Deposit(deposit) => deposit.requestId.id.clone(),
			L1UpdateRequest::CancelResolution(cancel) => cancel.requestId.id.clone(),
			L1UpdateRequest::Remove(remove) => remove.requestId.id.clone(),
			L1UpdateRequest::WithdrawalResolution(withdrawal) => withdrawal.requestId.id.clone(),
		}
	}

	pub fn origin(&self) -> Origin {
		match self {
			L1UpdateRequest::Deposit(deposit) => deposit.requestId.origin.clone(),
			L1UpdateRequest::CancelResolution(cancel) => cancel.requestId.origin.clone(),
			L1UpdateRequest::Remove(remove) => remove.requestId.origin.clone(),
			L1UpdateRequest::WithdrawalResolution(withdrawal) => withdrawal.requestId.origin.clone(),
		}
	}
}

impl L1Update {
	pub fn range(&self) -> Option<Range> {
		let first = [
			self.pendingDeposits.first().map(|v| v.requestId.id),
			self.pendingCancelResultions.first().map(|v| v.requestId.id),
			self.pendingL2UpdatesToRemove.first().map(|v| v.requestId.id),
			self.pendingWithdrawalResolutions.first().map(|v| v.requestId.id),
		]
		.into_iter()
		.cloned()
		.filter_map(|v| v)
		.min();

		let last = [
			self.pendingDeposits.last().map(|v| v.requestId.id),
			self.pendingCancelResultions.last().map(|v| v.requestId.id),
			self.pendingL2UpdatesToRemove.last().map(|v| v.requestId.id),
			self.pendingWithdrawalResolutions.last().map(|v| v.requestId.id),
		]
		.into_iter()
		.cloned()
		.filter_map(|v| v)
		.max();
		if let (Some(first), Some(last)) = (first, last) {
			Some(Range { start: first, end: last })
		} else {
			None
		}
	}

	pub fn into_requests(self) -> Vec<L1UpdateRequest> {
		let mut result: Vec<L1UpdateRequest> = Default::default();

		let L1Update { pendingDeposits, pendingCancelResultions, pendingL2UpdatesToRemove , pendingWithdrawalResolutions} = self;

		let mut deposits_it = pendingDeposits.into_iter().peekable();
		let mut cancel_it = pendingCancelResultions.into_iter().peekable();
		let mut remove_it = pendingL2UpdatesToRemove.into_iter().peekable();
		let mut withdrawal_it = pendingWithdrawalResolutions.into_iter().peekable();

		loop {
			let min = [
				deposits_it.peek().map(|v| v.requestId.id),
				cancel_it.peek().map(|v| v.requestId.id),
				remove_it.peek().map(|v| v.requestId.id),
				withdrawal_it.peek().map(|v| v.requestId.id),
			]
			.into_iter()
			.cloned()
			.filter_map(|v| v)
			.min();

			match (deposits_it.peek(), cancel_it.peek(), remove_it.peek(), withdrawal_it.peek(), min) {
				(Some(deposit), _, _, _, Some(min)) if deposit.requestId.id == min => {
					if let Some(elem) = deposits_it.next() {
						result.push(L1UpdateRequest::Deposit(elem.clone()));
					}
				},
				(_, Some(cancel), _, _, Some(min)) if cancel.requestId.id == min => {
					if let Some(elem) = cancel_it.next() {
						result.push(L1UpdateRequest::CancelResolution(elem.clone()));
					}
				},
				(_, _, Some(update), _, Some(min)) if update.requestId.id == min => {
					if let Some(elem) = remove_it.next() {
						result.push(L1UpdateRequest::Remove(elem.clone()));
					}
				},
				(_, _, Some(update), _, Some(min)) if update.requestId.id == min => {
					if let Some(elem) = withdrawal_it.next() {
						result.push(L1UpdateRequest::WithdrawalResolution(elem.clone()));
					}
				},
				_ => break,
			}
		}
		result
	}
}

pub fn to_eth_u256(value: U256) -> alloy_primitives::U256 {
	let mut bytes = [0u8; 32];
	value.to_big_endian(&mut bytes);
	alloy_primitives::U256::from_be_bytes(bytes)
}

impl Into<eth_abi::L1Update> for L1Update {
	fn into(self) -> eth_abi::L1Update {
		eth_abi::L1Update {
			pendingDeposits: self.pendingDeposits.into_iter().map(Into::into).collect::<Vec<_>>(),
			pendingCancelResultions: self
				.pendingCancelResultions
				.into_iter()
				.map(Into::into)
				.collect::<Vec<_>>(),
			pendingL2UpdatesToRemove: self
				.pendingL2UpdatesToRemove
				.into_iter()
				.map(Into::into)
				.collect::<Vec<_>>(),
			pendingWithdrawalResolutions: self
				.pendingWithdrawalResolutions
				.into_iter()
				.map(Into::into)
				.collect::<Vec<_>>(),
		}
	}
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Default, Serialize)]
pub struct Cancel<AccountId> {
	pub updater: AccountId,
	pub canceler: AccountId,
	pub lastProccessedRequestOnL1: U256,
	pub lastAcceptedRequestOnL1: U256,
	pub hash: H256,
}

pub use eth_abi::{L2Update, UpdateType};

pub mod eth_abi {
	use alloy_sol_types::sol;
	use codec::{Decode, Encode};
	use scale_info::TypeInfo;
	sol! {
		// L1 to L2
		#[derive(Debug)]
		struct Deposit {
			RequestId requestId;
			address depositRecipient;
			address tokenAddress;
			uint256 amount;
			bytes32 blockHash;
		}

		#[derive(Debug)]
		struct L2UpdatesToRemove {
			RequestId requestId;
			uint256[] l2UpdatesToRemove;
			bytes32 blockHash;
		}

		#[derive(Debug)]
		struct WithdrawalResolution {
			RequestId requestId;
			uint256 l2RequestId;
			bool status;
			bytes32 blockHash;
		}

		#[derive(Debug)]
		struct CancelResolution {
			RequestId requestId;
			uint256 l2RequestId;
			bool cancelJustified;
			bytes32 blockHash;
		}

		#[derive(Debug)]
		struct L1Update {
			Deposit[] pendingDeposits;
			CancelResolution[] pendingCancelResultions;
			WithdrawalResolution[] pendingWithdrawalResolutions;
			L2UpdatesToRemove[] pendingL2UpdatesToRemove;
		}


		#[derive(Debug, Eq, PartialEq, Encode, Decode, TypeInfo)]
		enum UpdateType{
			DEPOSIT,
			WITHDRAWAL,
			WITHDRAWAL_RESOLUTION,
			INDEX_UPDATE,
			CANCEL,
			CANCEL_RESOLUTION
		}

		#[derive(Debug, Eq, PartialEq, Encode, Decode, TypeInfo)]
		enum Origin{ L1, L2 }

		#[derive(Debug, Eq, PartialEq)]
		struct RequestId {
			Origin origin;
			uint256 id;
		}

		// L2 to L1
		#[derive(Debug, PartialEq)]
		struct RequestResult {
			RequestId requestId;
			uint256 originRequestId;
			UpdateType updateType;
			bool status;
		}

		#[derive(Debug, PartialEq)]
		struct Withdrawal {
			RequestId requestId;
			address withdrawalRecipient;
			address tokenAddress;
			uint256 amount;
		}

		#[derive(Debug, PartialEq)]
		struct Range{
			uint256 start;
			uint256 end;
		}

		#[derive(Debug, PartialEq)]
		struct Cancel {
			RequestId requestId;
			Range range;
			bytes32 hash;
		}

		#[derive(Debug, PartialEq)]
		struct L2Update {
			Cancel[] cancels;
			Withdrawal[] withdrawals;
			RequestResult[] results;
		}
	}
}
