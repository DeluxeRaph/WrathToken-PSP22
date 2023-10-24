#![cfg_attr(not(feature = "std"), no_std, no_main)]

        
#[openbrush::implementation(PSP22, PSP22Ownable,PSP22Burnable,PSP22Mintable)]
#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct WrathToken {
    	#[storage_field]
		psp22: psp22::Data,
		#[storage_field]
		ownable: ownable::Data,
    }
    // The WrathToken takes in a i32 Balance as its initial supply
    impl WrathToken {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut _instance = Self::default();
			psp22::Internal::_mint(&mut _instance, Self::env().caller(), initial_supply).expect("Should mint"); 
			ownable::Internal::_init_with_owner(&mut _instance Self::env().caller());
			_instance
        }
    }
}