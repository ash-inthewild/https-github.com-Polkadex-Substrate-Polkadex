
//! Autogenerated weights for `pallet_collective`
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
// --pallet=pallet_collective
// --extrinsic=*
// --steps
// 50
// --repeat
// 20
// --output=benchout/pallet_collective.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_collective.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	// Storage: Instance1Collective Members (r:1 w:1)
	// Storage: Instance1Collective Proposals (r:1 w:0)
	// Storage: Instance1Collective Voting (r:100 w:100)
	// Storage: Instance1Collective Prime (r:0 w:1)
	fn set_members(m: u32, n: u32, p: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 14_000
			.saturating_add((11_885_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 14_000
			.saturating_add((203_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 14_000
			.saturating_add((16_387_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Instance1Collective Members (r:1 w:0)
	fn execute(b: u32, m: u32, ) -> Weight {
		(16_601_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((82_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective ProposalOf (r:1 w:0)
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		(19_953_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((163_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective ProposalOf (r:1 w:1)
	// Storage: Instance1Collective Proposals (r:1 w:1)
	// Storage: Instance1Collective ProposalCount (r:1 w:1)
	// Storage: Instance1Collective Voting (r:0 w:1)
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		(26_493_000 as Weight)
			// Standard Error: 0
			.saturating_add((8_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_000
			.saturating_add((106_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((343_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective Voting (r:1 w:1)
	fn vote(m: u32, ) -> Weight {
		(29_509_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((207_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance1Collective Voting (r:1 w:1)
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective Proposals (r:1 w:1)
	// Storage: Instance1Collective ProposalOf (r:0 w:1)
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		(32_584_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((181_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((292_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Collective Voting (r:1 w:1)
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective ProposalOf (r:1 w:1)
	// Storage: Instance1Collective Proposals (r:1 w:1)
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(39_208_000 as Weight)
			// Standard Error: 0
			.saturating_add((6_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_000
			.saturating_add((188_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((335_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Collective Voting (r:1 w:1)
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective Prime (r:1 w:0)
	// Storage: Instance1Collective Proposals (r:1 w:1)
	// Storage: Instance1Collective ProposalOf (r:0 w:1)
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		(35_741_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((179_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((301_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Collective Voting (r:1 w:1)
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective Prime (r:1 w:0)
	// Storage: Instance1Collective ProposalOf (r:1 w:1)
	// Storage: Instance1Collective Proposals (r:1 w:1)
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(41_836_000 as Weight)
			// Standard Error: 0
			.saturating_add((5_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_000
			.saturating_add((201_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((341_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Collective Proposals (r:1 w:1)
	// Storage: Instance1Collective Voting (r:0 w:1)
	// Storage: Instance1Collective ProposalOf (r:0 w:1)
	fn disapprove_proposal(p: u32, ) -> Weight {
		(20_040_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((326_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
