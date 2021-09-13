
//! Autogenerated weights for `pallet_bounties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/polkadex-node
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet=pallet_bounties
// --extrinsic=*
// --steps
// 50
// --repeat
// 20
// --output=benchout/pallet_bounties.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_bounties.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bounties::WeightInfo for WeightInfo<T> {
	// Storage: Treasury BountyCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Treasury BountyDescriptions (r:0 w:1)
	// Storage: Treasury Bounties (r:0 w:1)
	fn propose_bounty(d: u32, ) -> Weight {
		(44_080_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: Treasury BountyApprovals (r:1 w:1)
	fn approve_bounty() -> Weight {
		(11_581_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	fn propose_curator() -> Weight {
		(9_597_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unassign_curator() -> Weight {
		(41_003_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn accept_curator() -> Weight {
		(37_315_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	fn award_bounty() -> Weight {
		(25_317_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: Treasury BountyDescriptions (r:0 w:1)
	fn claim_bounty() -> Weight {
		(129_792_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Treasury BountyDescriptions (r:0 w:1)
	fn close_bounty_proposed() -> Weight {
		(39_618_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Treasury BountyDescriptions (r:0 w:1)
	fn close_bounty_active() -> Weight {
		(86_005_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Treasury Bounties (r:1 w:1)
	fn extend_bounty_expiry() -> Weight {
		(23_746_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Treasury BountyApprovals (r:1 w:1)
	// Storage: Treasury Bounties (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn spend_funds(b: u32, ) -> Weight {
		(210_640_000 as Weight)
			// Standard Error: 75_000
			.saturating_add((42_880_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
}
