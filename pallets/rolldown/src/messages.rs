#![allow(non_snake_case)]
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::Serialize;
use sp_core::{RuntimeDebug, H256, U256};
use sp_runtime::SaturatedConversion;
use sp_std::vec::Vec;

#[repr(u8)]
#[derive(Copy, Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, MaxEncodedLen, Serialize)]
pub enum L1 {
	Ethereum,
}

#[repr(u8)]
#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize)]
pub enum Origin{
	L1,
	L2
}

impl Default for Origin{
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
pub struct Range{
	pub start: u128,
	pub end: u128,
}

impl From<(u128, u128)> for Range {
	fn from((start, end): (u128, u128)) -> Range {
		Range{start, end}
	}
}

impl Into<eth_abi::Range> for Range {
	fn into(self) -> eth_abi::Range {
		eth_abi::Range {
			start: to_eth_u256(self.start.into()),
			end: to_eth_u256(self.end.into()),
		}
	}
}


#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize, Default)]
pub struct RequestId {
	pub origin: Origin,
	pub id: u128,
}

impl Into<eth_abi::RequestId> for RequestId {
	fn into(self) -> eth_abi::RequestId {
		eth_abi::RequestId {
			origin: self.origin.into(),
			id: to_eth_u256(U256::from(self.id)),
		}
	}
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize, Default)]
pub struct Deposit {
	pub requestId: RequestId,
	pub depositRecipient: [u8; 20],
	pub tokenAddress: [u8; 20],
	pub amount: sp_core::U256,
}

impl Into<eth_abi::Deposit> for Deposit {
	fn into(self) -> eth_abi::Deposit {
		eth_abi::Deposit {
			requestId: self.requestId.into(),
			depositRecipient: self.depositRecipient.into(),
			tokenAddress: self.tokenAddress.into(),
			amount: to_eth_u256(self.amount),
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
	pub l2UpdatesToRemove: Vec<sp_core::U256>,
}

impl Into<eth_abi::L2UpdatesToRemove> for L2UpdatesToRemove {
	fn into(self) -> eth_abi::L2UpdatesToRemove {
		eth_abi::L2UpdatesToRemove {
			requestId: self.requestId.into(),
			l2UpdatesToRemove: self
				.l2UpdatesToRemove
				.into_iter()
				.map(|req_id| to_eth_u256(req_id))
				.collect(),
		}
	}
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize)]
pub struct CancelResolution {
	pub requestId: RequestId,
	pub l2RequestId: sp_core::U256,
	pub cancelJustified: bool,
}

impl Into<eth_abi::CancelResolution> for CancelResolution {
	fn into(self) -> eth_abi::CancelResolution {
		eth_abi::CancelResolution {
			requestId: self.requestId.into(),
			l2RequestId: to_eth_u256(self.l2RequestId),
			cancelJustified: self.cancelJustified.into(),
		}
	}
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize)]
#[repr(u8)]
pub enum PendingRequestType {
	DEPOSIT,
	CANCEL_RESOLUTION,
	L2_UPDATES_TO_REMOVE,
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Default, Serialize)]
pub struct L1Update {
	pub pendingDeposits: Vec<Deposit>,
	pub pendingCancelResultions: Vec<CancelResolution>,
	pub pendingL2UpdatesToRemove: Vec<L2UpdatesToRemove>,
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, TypeInfo, Serialize)]
pub enum L1UpdateRequest {
	Deposit(Deposit),
	Cancel(CancelResolution),
	Remove(L2UpdatesToRemove),
}

impl L1UpdateRequest{
	pub fn id(&self) -> u128 {
		match self {
			L1UpdateRequest::Deposit(deposit) => deposit.requestId.id.clone(),
			L1UpdateRequest::Cancel(cancel) => cancel.requestId.id.clone(),
			L1UpdateRequest::Remove(remove) => remove.requestId.id.clone(),
		}
	}

	pub fn origin(&self) -> Origin {
		match self {
			L1UpdateRequest::Deposit(deposit) => deposit.requestId.origin.clone(),
			L1UpdateRequest::Cancel(cancel) => cancel.requestId.origin.clone(),
			L1UpdateRequest::Remove(remove) => remove.requestId.origin.clone(),
		}
	}

}

impl Into<eth_abi::PendingRequestType> for PendingRequestType {
	fn into(self) -> eth_abi::PendingRequestType {
		match self {
			PendingRequestType::DEPOSIT => eth_abi::PendingRequestType::DEPOSIT,
			PendingRequestType::CANCEL_RESOLUTION => eth_abi::PendingRequestType::CANCEL_RESOLUTION,
			PendingRequestType::L2_UPDATES_TO_REMOVE =>
				eth_abi::PendingRequestType::L2_UPDATES_TO_REMOVE,
		}
	}
}

impl L1Update {
	pub fn range(&self) -> Option<Range> {
		let first = [
			self.pendingDeposits.first().map(|v| v.requestId.id),
			self.pendingCancelResultions.first().map(|v| v.requestId.id),
			self.pendingL2UpdatesToRemove.first().map(|v| v.requestId.id)
			]
			.into_iter()
			.cloned()
			.filter_map(|v| v)
		.min();

		let last = [
			self.pendingDeposits.last().map(|v| v.requestId.id),
			self.pendingCancelResultions.last().map(|v| v.requestId.id),
			self.pendingL2UpdatesToRemove.last().map(|v| v.requestId.id)
			]
			.into_iter()
			.cloned()
			.filter_map(|v| v)
		.max();
		if let (Some(first), Some(last)) = (first, last){
			Some(Range{start: first, end: last})
		}else{
			None
		}

	}
	pub fn into_requests(self) -> Vec<L1UpdateRequest> {
		let mut result = vec![];

		let L1Update {
			mut pendingDeposits,
			mut pendingCancelResultions,
			mut pendingL2UpdatesToRemove,
		} = self;

		loop {

			let all = [
				pendingDeposits.first().map(|v| v.requestId.id),
				pendingCancelResultions.first().map(|v| v.requestId.id),
				pendingL2UpdatesToRemove.first().map(|v| v.requestId.id)
			]
			.into_iter()
			.cloned()
			.filter_map(|v| v)
			.collect::<Vec<_>>();

			println!("all: {:?}", all);

			let min = [
				pendingDeposits.first().map(|v| v.requestId.id),
				pendingCancelResultions.first().map(|v| v.requestId.id),
				pendingL2UpdatesToRemove.first().map(|v| v.requestId.id)
			]
			.into_iter()
			.cloned()
			.filter_map(|v| v)
			.min();

			println!("min: {:?}", min);
			println!("pendingDeposits: {:?}", pendingDeposits.first());
			println!("pendingCancelResultions: {:?}", pendingCancelResultions.first());
			println!("pendingL2UpdatesToRemove: {:?}", pendingL2UpdatesToRemove.first());

			match ((pendingDeposits.first()), pendingCancelResultions.first(), pendingL2UpdatesToRemove.first(), min) {
				(Some(deposit), _, _, Some(min)) if deposit.requestId.id == min => {
					if let Some(elem) = pendingDeposits.pop(){
						result.push(L1UpdateRequest::Deposit(elem));
					}
				},
				(_, Some(cancel),  _, Some(min)) if cancel.requestId.id == min => {
					if let Some(elem) = pendingCancelResultions.pop(){
						result.push(L1UpdateRequest::Cancel(elem));
					}
				},
				(_, _, Some(update), Some(min)) if update.requestId.id == min => {
					if let Some(elem) = pendingL2UpdatesToRemove.pop(){
						result.push(L1UpdateRequest::Remove(elem));
					}
				},
				_ => { break; }
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
			pendingDeposits: self.pendingDeposits
				.into_iter()
				.map(Into::into)
				.collect::<Vec<_>>(),
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
		struct Deposit {
			RequestId requestId;
			address depositRecipient;
			address tokenAddress;
			uint256 amount;
		}

		struct L2UpdatesToRemove {
			RequestId requestId;
			uint256[] l2UpdatesToRemove;
		}

		struct CancelResolution {
			RequestId requestId;
			uint256 l2RequestId;
			bool cancelJustified;
		}

		enum PendingRequestType{ DEPOSIT, CANCEL_RESOLUTION, L2_UPDATES_TO_REMOVE}

		struct L1Update {
			Deposit[] pendingDeposits;
			CancelResolution[] pendingCancelResultions;
			L2UpdatesToRemove[] pendingL2UpdatesToRemove;
		}


		#[derive(Debug, Eq, PartialEq, Encode, Decode, TypeInfo)]
		enum UpdateType{ DEPOSIT, WITHDRAWAL, INDEX_UPDATE, CANCEL_RESOLUTION}

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
