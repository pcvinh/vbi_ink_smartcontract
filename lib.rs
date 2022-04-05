#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract]
mod testcontract {
    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Testcontract {
        /// Stores a single `bool` value on the storage.
        count: u32,
        table: Mapping<AccountId, u32>
        
    }

    impl Testcontract {
        /// Constructor that initializes the contract
        #[ink(constructor)]
        pub fn new(val: u32) -> Self {
            //Self { value: init_value }
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.table.insert(&caller, &val);
            })
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_number(&self) -> u32 {
            let caller = Self::env().caller();
            self.table.get(caller).unwrap_or_default()
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn set_number(&mut self, value: u32) {
            let caller = Self::env().caller();
            self.count = self.count + 1;
            self.table.insert(&caller, &value);
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_count(&self) -> u32 {
            self.count
        }
    }

    

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let testcontract = Testcontract::default();
            assert_eq!(testcontract.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut testcontract = Testcontract::new(false);
            assert_eq!(testcontract.get(), false);
            testcontract.flip();
            assert_eq!(testcontract.get(), true);
        }
    }
}
