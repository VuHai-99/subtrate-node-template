#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
pub use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	//use sp_runtime::generic::BlockId::Number;
	pub type Id = u32;

	#[derive(TypeInfo, Encode, Decode, Debug)]
	pub enum Gender{
		Male, Female,
	}
	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Student<T:Config>{
		name: Vec<u8>,
		age: u8,
		gender: Gender,
		account: T::AccountId
	}

	impl Default for Gender {
		fn default() -> Self {
			Gender::Female
		}
	}
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn student_id)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type StudentIds<T> = StorageValue<_, Id, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn students)]
	// Key :Id, Value: Student
	pub(super) type Students<T:Config> = StorageMap<_,Blake2_128Concat,Id, Student<T>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		CreatedStudent(Vec<u8>,u8),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		TooYoung,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000+ T::DbWeight::get().writes(1))]
		pub fn create_student(origin:OriginFor<T>,name: Vec<u8>,age:u8) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(age>20, Error::<T>::TooYoung);
			let gender = Self::gen_gender(name.clone())?;
			let student = Student{
				name: name.clone(),
				age: age,
				gender: gender,
				account: who,
			};
			// let current_id = Self::student_id();
			// let current_id = StudentIds::<T>::get();
			let mut current_id = <StudentIds<T>>::get();

			// Students::<T>::insert(current_id, student);
			<Students<T>>::insert(current_id, student);

			current_id = current_id + 1;

			StudentIds::<T>::put(current_id);
			Self::deposit_event(Event::CreatedStudent(name,age));
			Ok(())
		}

	}
}

//helper function
impl<T> Pallet<T>{
	fn gen_gender(name:Vec<u8>) -> Result<Gender,Error<T>>{
		let mut result = Gender::Female;
		if name.len() % 2 == 0 {
			result = Gender::Male
		}
		Ok(result)
	}
}
