//! Benchmarking setup for pallet-template
use mock::PDEX;

use super::*;

use frame_system::RawOrigin;
use frame_support::{assert_noop, assert_ok, error::BadOrigin};
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};

// use frame_benchmarking::({account, benchmarks, impl_benchmark_test_suite, whitelist_account});
use frame_support::traits::{EnsureOrigin, Get, UnfilteredDispatchable};
// use frame_system::{self, EventRecord, RawOrigin};
// use orml_tokens::{AccountData, Accounts};
// use sp_runtime::traits::Bounded;

use frame_benchmarking::account;
use crate::pallet::Pallet;
use frame_system::Call;
// use frame_system::Pallet;
use sp_runtime::testing::H256;
use frame_system::Origin;
// use crate::mock::PDEX;
use crate::pallet::Pallet as PDEXMigration;
// use runtime::PDEX;
#[allow(unused)]
// use crate::Pallet as Template;
use crate::pallet::Config;

benchmarks! {
    set_migration_operational_status {

    }: _(RawOrigin::Root, true)

    set_relayer_status {
        let relayer = account("relayer",0,0);
    }: _ (RawOrigin::Root, relayer, true)

    mint {
        let relayer1 = account("relayer1",0,0);
        let relayer2  = account("relayer2",0,0);
        let relayer3 = account("relayer3",0,0);
        let beneficiary = whitelisted_caller();
        assert_ok!(PDEXMigration::set_migration_operational_status(Origin::root(),true));
        // Register relayers
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer1,true));
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer2,true));
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer3,true));

        assert_ok!(PDEXMigration::mint(Origin::signed(relayer1), beneficiary,100*PDEX,H256::zero()));
        assert_ok!(PDEXMigration::mint(Origin::signed(relayer2), beneficiary,100*PDEX,H256::zero()));

        let beneficiary: T::AccountId = whitelisted_caller();
        let eth_hash: T::Hash = T::Hash::default();
        let relayer3: T::AccountId = account("relayer3",0,0);
        let amount: T::Balance = <T as pallet_balances::Config>::ExistentialDeposit::get().saturating_mul(100u32.into());
    }: _(RawOrigin::Signed(relayer3),beneficiary,amount,eth_hash)


    unlock {
        let relayer1 = account("relayer1",0,0);
        let relayer2  = account("relayer2",0,0);
        let relayer3 = account("relayer3",0,0);
        let beneficiary = whitelisted_caller();
        assert_ok!(PDEXMigration::set_migration_operational_status(Origin::root(),true));
        // Register relayers
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer1,true));
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer2,true));
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer3,true));

        assert_ok!(PDEXMigration::mint(Origin::signed(relayer1), beneficiary,100*PDEX,H256::zero()));
        assert_ok!(PDEXMigration::mint(Origin::signed(relayer2), beneficiary,100*PDEX,H256::zero()));
        assert_ok!(PDEXMigration::mint(Origin::signed(relayer3), beneficiary,100*PDEX,H256::zero()));

        // frame_system::Pallet::<T>::set_block_number(frame_system::Pallet::<T>::current_block_number()+T::LockPeriod::get());

        let beneficiary: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(beneficiary))

    remove_minted_tokens {
        let relayer1 = account("relayer1",0,0);
        let relayer2  = account("relayer2",0,0);
        let relayer3 = account("relayer3",0,0);
        let beneficiary = whitelisted_caller();
        assert_ok!(PDEXMigration::set_migration_operational_status(Origin::root(),true));
        // Register relayers
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer1,true));
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer2,true));
        assert_ok!(PDEXMigration::set_relayer_status(Origin::root(),relayer3,true));

        assert_ok!(PDEXMigration::mint(Origin::signed(relayer1), beneficiary,100*PDEX,H256::zero()));
        assert_ok!(PDEXMigration::mint(Origin::signed(relayer2), beneficiary,100*PDEX,H256::zero()));
        assert_ok!(PDEXMigration::mint(Origin::signed(relayer3), beneficiary,100*PDEX,H256::zero()));

        let beneficiary: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Root,beneficiary)
}

impl_benchmark_test_suite!(
	Template,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);
