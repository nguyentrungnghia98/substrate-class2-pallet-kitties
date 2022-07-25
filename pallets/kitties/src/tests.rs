use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::*;

#[test]
fn create_kitty_works_for_default_value() {
	new_test_ext().execute_with(|| {
		let origin = Origin::signed(1);
		let address = 1;

		log::warn!("MaxOwned 1: {:?}", <Test as Config>::MaxOwned::get());
		log::info!("count: {:?}", KittyCount::<Test>::get());
		log::error!("count own: {:?}", KittiesOwned::<Test>::get(address).len());

		assert_eq!(KittyCount::<Test>::get(), 0);

		assert_ok!(KittyModule::create_kitty(Origin::signed(1), 1));
		// assert_eq!(KittyCount::<Test>::get(), 1);
		// assert_eq!(Kitties::<Test>::iter_keys().count(), 1);
		// assert_eq!(KittiesOwned::<Test>::iter_keys().count(), 1);
		// assert_eq!(KittiesOwned::<Test>::get(address).len(), 1);

		// assert_ok!(KittyModule::create_kitty(origin, 1));
		// assert_eq!(KittyCount::<Test>::get(), 2);
		// assert_eq!(Kitties::<Test>::iter_keys().count(), 2);
		// assert_eq!(KittiesOwned::<Test>::iter_keys().count(), 1);
		// assert_eq!(KittiesOwned::<Test>::get(address).len(), 2);
	});
}

// #[test]
// fn correct_error_for_invalid_kitty_price() {
// 	new_test_ext().execute_with(|| {
// 		assert_noop!(KittyModule::create_kitty(Origin::signed(1), -1), Error::<Test>::PriceMustGreaterThanZero);
// 	});
// }

//? test max kitty

fn get_kitty(origin: Origin, address: u64) -> Result<Kitty<Test>, ()> {
	if (KittiesOwned::<Test>::get(address).len() as u32) == 0 {
		KittyModule::create_kitty(origin, 0).unwrap();
	}
	let kitty_address = KittyModule::kitties_owned(address)[0];

	let kitty_option = KittyModule::kitties(kitty_address);
	match kitty_option {
		Some(kitty) => Ok(kitty),
		None => Err(()),
	}

}

#[test]
fn transfer_kitty_works_for_default_value() {
	new_test_ext().execute_with(|| {
		let from = Origin::signed(1);
		let from_address = 1;
		let to_address = 2;

		let old_kitty_count = KittyCount::<Test>::get()  as u32;
		let old_kitties_len = KittiesOwned::<Test>::iter_keys().count()  as u32;
		let old_kitties_owned_from_len = KittiesOwned::<Test>::get(from_address).len()  as u32;
		let old_kitties_owned_to_len = KittiesOwned::<Test>::get(to_address).len()  as u32;

		// if (old_kitties_owned_from_len == 0) {
		// 	KittyModule::create_kitty(Origin::signed(1), 0);
		// }
		// let transfer_kitty = <KittiesOwned::<Test>::get(from)[0];
		let transfer_kitty = get_kitty(from.clone(), from_address).unwrap();

		assert_ok!(KittyModule::transfer(from, to_address, transfer_kitty.dna));

		let new_kitty_count = KittyCount::<Test>::get()  as u32;
		let new_kitties_len = KittiesOwned::<Test>::iter_keys().count()  as u32;
		let new_kitties_owned_from_len = KittiesOwned::<Test>::get(from_address).len()  as u32;
		let new_kitties_owned_to_len = KittiesOwned::<Test>::get(to_address).len()  as u32;

		assert_eq!(old_kitty_count, new_kitty_count);
		assert_eq!(old_kitties_len, new_kitties_len);
		assert_eq!(old_kitties_owned_from_len, new_kitties_owned_from_len + 1);
		assert_eq!(old_kitties_owned_to_len, new_kitties_owned_to_len - 1);
	});
}

#[test]
fn correct_error_for_transfer_invalid_kitty_owner() {
	new_test_ext().execute_with(|| {
		let from = Origin::signed(1);
		let from_address = 1;
		let to = Origin::signed(2);
		let to_address = 2;

		let transfer_kitty = get_kitty(from.clone(), from_address).unwrap();
		assert_noop!(KittyModule::transfer(from, to_address, transfer_kitty.dna), Error::<Test>::NotKittyOwner);
	});
}

#[test]
fn correct_error_for_transfer_invalid_transfer_to_yourself() {
	new_test_ext().execute_with(|| {
		let from = Origin::signed(1);
		let from_address = 1;

		let transfer_kitty = get_kitty(from.clone(), from_address).unwrap();

		assert_noop!(KittyModule::transfer(from, from_address, transfer_kitty.dna), Error::<Test>::TransferToYourself);
	});
}

//? test max kitty
