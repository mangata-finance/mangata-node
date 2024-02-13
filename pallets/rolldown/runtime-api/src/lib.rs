// Copyright (C) 2021 Mangata team
#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;
sp_api::decl_runtime_apis! {
	pub trait RolldownRuntimeApi{
		fn get_pending_updates_hash() -> sp_core::H256;
		fn get_pending_updates() -> Vec<u8>;
	}
}