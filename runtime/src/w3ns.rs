// Enables access to the runtime storage
use srml_support::{StorageMap, dispatch::Result};


// Enables access to account balances and interacting with signed messages
use {balances, system::ensure_signed};

pub trait Trait: balances::Trait {}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
	fn buy(origin, domain: u32) -> Result {
	  // Ensure we have a signed message, and derive the sender's account id from the signature
	  let sender = ensure_signed(origin)?;
	  
	  <Dns<T>>::insert(domain, sender.clone());

	  Ok(())
	}
  }
}

decl_storage! {
  trait Store for Module<T: Trait> as Demo {
		pub Dns get(domain_owner): map u32 => T::AccountId;
  }
}
