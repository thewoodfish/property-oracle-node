#![cfg_attr(not(feature = "std"), no_std)]

/// This oracle pallet serves as the pallet that contains all the business logic of property oracle.
/// It handles account creation, creation of property type, witnesses and attestations etc.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// use core::ops::Bound;

	use frame_support::traits::UnixTime;
	use frame_support::{pallet_prelude::*, /*traits::Bounded,*/ BoundedVec};
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;
	use sp_core::H256;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type TimeProvider: UnixTime;

		#[pallet::constant]
		type MaxUriLength: Get<u32>;

		#[pallet::constant]
		type MaxAsciiTextLength: Get<u32>;

		#[pallet::constant]
		type MaxCount: Get<u32>;
	}

	// property type
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct PropertyType<T: Config> {
		pub title: BoundedVec<u8, T::MaxAsciiTextLength>,
		pub registrar: T::AccountId,
		pub attributes: BoundedVec<u8, T::MaxAsciiTextLength>,
		pub cid: BoundedVec<u8, T::MaxUriLength>,
		pub timestamp: u64,
	}

	// property type
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Credential<T: Config> {
		pub owners: BoundedVec<T::AccountId, T::MaxCount>,
		pub verifiers: BoundedVec<T::AccountId, T::MaxCount>,
		pub cid: BoundedVec<u8, T::MaxUriLength>,
		pub timestamp: u64,
	}

	#[pallet::storage]
	#[pallet::getter(fn user_reg)]
	pub(super) type UserRegistry<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<u8, T::MaxUriLength>>;

	#[pallet::storage]
	#[pallet::getter(fn ptype_reg)]
	pub(super) type PropertyTypeRegistry<T: Config> =
		StorageMap<_, Twox64Concat, H256, PropertyType<T>>;

	#[pallet::storage]
	#[pallet::getter(fn cred_reg)]
	pub(super) type CredentialRegistry<T: Config> =
		StorageMap<_, Twox64Concat, H256, BoundedVec<Credential<T>, T::MaxCount>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// a new user has been added
		NewUserRecorded { cid: Vec<u8> },
		/// a new property record has been added
		NewPropertyTypeRecorded { cid: Vec<u8> },
		/// a new unverified credential has been created
		NewPropertyCredentialCreated { cid: Vec<u8> },
		/// property transferred from one entity to the other
		PropertyTransferred { sender: T::AccountId, recipient: T::AccountId, property_id: H256 },
		/// a property claim has just been signed
		PropertyClaimAttested { verifier: T::AccountId, property_id: H256 },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// the Uri is too long
		UriOverflow,
		/// the count upper bound was exceeded
		CountOverflow,
		/// property entry not found, possibly because of wrong property hash(id) given
		PropertyEntryNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// record a new user on the blockchain
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn record_user(origin: OriginFor<T>, cid: Vec<u8>) -> DispatchResult {
			// get sender
			let who = ensure_signed(origin)?;
			let mut uri: BoundedVec<_, T::MaxUriLength> = Default::default();

			for i in cid.clone() {
				uri.try_push(i).map_err(|_| Error::<T>::UriOverflow)?
			}
			UserRegistry::<T>::insert(&who, uri.clone());

			Self::deposit_event(Event::NewUserRecorded { cid });
			Ok(())
		}

		/// record a new property type on the blockchain
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn record_ptype(
			origin: OriginFor<T>,
			hash: H256,
			name: Vec<u8>,
			cid: Vec<u8>,
			props: Vec<u8>,
		) -> DispatchResult {
			// get sender
			let who = ensure_signed(origin)?;
			let mut uri: BoundedVec<_, T::MaxUriLength> = Default::default();
			let mut title: BoundedVec<_, T::MaxAsciiTextLength> = Default::default();
			let mut attr: BoundedVec<_, T::MaxAsciiTextLength> = Default::default();

			for i in cid.clone() {
				uri.try_push(i).map_err(|_| Error::<T>::UriOverflow)?
			}

			for j in name.clone() {
				title.try_push(j).map_err(|_| Error::<T>::UriOverflow)?
			}

			for k in props.clone() {
				attr.try_push(k).map_err(|_| Error::<T>::UriOverflow)?
			}

			// create struct
			let record = PropertyType {
				title,
				registrar: who,
				attributes: attr,
				cid: uri,
				timestamp: T::TimeProvider::now().as_secs(),
			};

			PropertyTypeRegistry::<T>::insert(hash, record);

			Self::deposit_event(Event::NewPropertyTypeRecorded { cid });
			Ok(())
		}

		/// record the creation of a property credential
		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn record_credential(origin: OriginFor<T>, hash: H256, cid: Vec<u8>) -> DispatchResult {
			// get sender
			let who = ensure_signed(origin)?;
			let mut uri: BoundedVec<_, T::MaxUriLength> = Default::default();

			for i in cid.clone() {
				uri.try_push(i).map_err(|_| Error::<T>::UriOverflow)?
			}

			let mut owners: BoundedVec<T::AccountId, T::MaxCount> = Default::default();
			owners.try_push(who).map_err(|_| Error::<T>::CountOverflow)?;

			// set up credential
			let cred = Credential { owners, verifiers: Default::default(), cid: uri, timestamp: 0 };

			// try to get credential
			if let Some(mut credentials) = CredentialRegistry::<T>::get(hash) {
				credentials.try_push(cred).map_err(|_| Error::<T>::CountOverflow)?;
				CredentialRegistry::<T>::insert(hash, credentials);
			} else {
				let mut credentials: BoundedVec<Credential<T>, T::MaxCount> = Default::default();
				credentials.try_push(cred).map_err(|_| Error::<T>::CountOverflow)?;
				CredentialRegistry::<T>::insert(hash, credentials);
			}

			Self::deposit_event(Event::NewPropertyCredentialCreated { cid });
			Ok(())
		}

		/// transfer a property from one entity to another
		#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn transfer_property(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			property_id: H256,
			original_document_cid: Vec<u8>,
			new_sender_credential_cid: Vec<u8>,
			recipient_credential_cid: Vec<u8>,
			transfer_all: bool,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let mut is_owned_by_sender = false;
			let mut od_cid: BoundedVec<_, T::MaxUriLength> = Default::default();
			let mut nsc_cid: BoundedVec<_, T::MaxUriLength> = Default::default();
			let mut rc_cid: BoundedVec<_, T::MaxUriLength> = Default::default();
			let mut create_new_crdential = false;

			for i in original_document_cid.clone() {
				od_cid.try_push(i).map_err(|_| Error::<T>::UriOverflow)?
			}

			for j in new_sender_credential_cid.clone() {
				nsc_cid.try_push(j).map_err(|_| Error::<T>::UriOverflow)?
			}

			for k in recipient_credential_cid.clone() {
				rc_cid.try_push(k).map_err(|_| Error::<T>::UriOverflow)?
			}

			if let Some(mut properties) = CredentialRegistry::<T>::get(&property_id) {
				for p in &mut properties {
					let owners = p.owners.to_vec();
					let curr_owner = &owners[owners.len() - 1];

					if *curr_owner == sender.clone() && p.cid == od_cid {
						if transfer_all {
							// just change the owner of the document
							p.owners
								.try_push(recipient.clone())
								.map_err(|_| Error::<T>::CountOverflow)?;
						} else {
							// create new credential and update the old CID to reflect the transfer and creation of the new document
							p.cid = nsc_cid;
							create_new_crdential = true;
						}

						is_owned_by_sender = true;
						break;
					}
				}

				if !is_owned_by_sender {
					return Err(Error::<T>::PropertyEntryNotFound.into());
				}

				if create_new_crdential {
					let mut owners: BoundedVec<T::AccountId, T::MaxCount> = Default::default();
					owners.try_push(recipient.clone()).map_err(|_| Error::<T>::CountOverflow)?;

					// new credential
					let cred = Credential {
						owners,
						verifiers: Default::default(),
						cid: rc_cid,
						timestamp: 0,
					};

					properties.try_push(cred).map_err(|_| Error::<T>::CountOverflow)?;
				}

				// save the new properties
				CredentialRegistry::<T>::insert(&property_id, properties);

			} else {
				// throw
				return Err(Error::<T>::PropertyEntryNotFound.into());
			}

			Self::deposit_event(Event::PropertyTransferred { sender, recipient, property_id });
			Ok(())
		}

		/// record property claim attestation
		#[pallet::call_index(4)]
		#[pallet::weight(10_000)]
		pub fn attest_claim(
			origin: OriginFor<T>,
			property_id: H256,
			cid: Vec<u8>,
			is_canonical_verifier: bool,
		) -> DispatchResult {
			let verifier = ensure_signed(origin)?;
			let mut uri: BoundedVec<_, T::MaxUriLength> = Default::default();

			for i in cid.clone() {
				uri.try_push(i).map_err(|_| Error::<T>::UriOverflow)?
			}

			if let Some(mut properties) = CredentialRegistry::<T>::get(&property_id) {
				for p in &mut properties {
					if p.cid == uri {
						// modify the verifiers
						p.verifiers
							.try_push(verifier.clone())
							.map_err(|_| Error::<T>::CountOverflow)?;

						if is_canonical_verifier {
							p.timestamp = T::TimeProvider::now().as_secs(); // take only the time that cononical verifier signs the document
						}
					}
				}

				// save the new properties
				CredentialRegistry::<T>::insert(&property_id, properties);
			} else {
				// throw
				return Err(Error::<T>::PropertyEntryNotFound.into());
			}

			Self::deposit_event(Event::PropertyClaimAttested { verifier, property_id });
			Ok(())
		}
	}
}
