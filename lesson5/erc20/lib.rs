#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
	use ink_storage::collections::HashMap as StorageHashMap;

	#[ink(storage)]
	pub struct Erc20 {
		total_supply: Balance,
		balances: StorageHashMap<AccountId, Balance>,
		allowance: StorageHashMap<(AccountId, AccountId), Balance>,
	}

	#[ink(event)]
	pub struct Transfer {
		#[ink(topic)]
		from: AccountId,
		#[ink(topic)]
		to: AccountId,
		value: Balance,
	}

	#[ink(event)]
	pub struct Approval {
		#[ink(topic)]
		owner: AccountId,
		#[ink(topic)]
		spender: AccountId,
		value: Balance,
    }

    #[ink(event)]
	pub struct Mint {
		#[ink(topic)]
		who: AccountId,
		#[ink(topic)]
		to: AccountId,
		value: Balance,
    }

    #[ink(event)]
	pub struct Burn {
		#[ink(topic)]
		who: AccountId,
		#[ink(topic)]
		to: AccountId,
		value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature="std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        AllowanceNotEnough,
    }

    pub type Result<T> = core::result::Result<T, Error>;

	impl Erc20 {
		#[ink(constructor)]
		pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = StorageHashMap::new();
            balances.insert(caller, initial_supply);
            let instance = Self {
                total_supply: initial_supply,
                balances,
                allowance: StorageHashMap::new(),
            };
            
            instance
		}

		#[ink(message)]
		pub fn total_supply(&self) -> Balance {
			self.total_supply
		}

		#[ink(message)]
		pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.get_balance_or_zero(&owner)
        }
        
        #[ink(message)]
		pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
			self.get_allowance_or_zero(&owner, &spender)
		}

		#[ink(message)]
		pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let who = self.env().caller();

			self.do_transfer(who, to, value)?;

			Ok(())
		}

		#[ink(message)]
		pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
			let owner = self.env().caller();
            self.allowance.insert((owner, spender), value);

			self.env().emit_event(Approval {
				owner,
				spender,
				value,
			});

			Ok(())
		}

		#[ink(message)]
		pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
			let spender = self.env().caller();
			let allowance = self.get_allowance_or_zero(&from, &spender);
			if allowance < value {
                return Err(Error::AllowanceNotEnough);
			}

			self.do_transfer(from, to, value)?;
			self.allowance.insert((from, spender), allowance - value);

			Ok(())
        }
        
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let who = self.env().caller();

            let to_balance = self.get_balance_or_zero(&to);
            self.balances.insert(to, to_balance + value);
            self.total_supply = self.total_supply + value;

            self.env().emit_event(Mint {
				who,
				to,
				value,
            });
            
            Ok(())
        }

        #[ink(message)]
        pub fn burn(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let who = self.env().caller();

            let to_balance = self.get_balance_or_zero(&to);
            self.balances.insert(to, to_balance - value);
            self.total_supply = self.total_supply - value;

            self.env().emit_event(Burn {
				who,
				to,
				value,
            });
            
            Ok(())
        }

		fn do_transfer(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
			let from_balance = self.get_balance_or_zero(&from);
			if from_balance < value {
				return Err(Error::InsufficientBalance);
			}

            self.balances.insert(from, from_balance - value);
            let to_balance = self.get_balance_or_zero(&to);
            self.balances.insert(to, to_balance + value);

			self.env().emit_event(Transfer {
				from,
				to,
				value,
			});

			Ok(())
		}

		fn get_balance_or_zero(&self, account: &AccountId) -> Balance {
			*self.balances.get(account).unwrap_or(&0)
		}

		fn get_allowance_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
			*self.allowance.get(&(*owner, *spender)).unwrap_or(&0)
		}
	}

	#[cfg(test)]
	mod tests {
        use super::*;
        use ink_lang as ink;
        // use ink_env::{
        //     hash::{
        //         Blake2x256,
        //         CryptoHash,
        //         HashOutput,
        //     },
        //     Clear,
        // };

		#[ink::test]
		fn new_works() {
			let erc20 = Erc20::new(100);
			assert_eq!(erc20.total_supply(), 100);
		}

		#[ink::test]
		fn balance_of_works() {
			let erc20 = Erc20::new(100);
			assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 100);
			assert_eq!(erc20.balance_of(AccountId::from([0x2; 32])), 0);
		}

		#[ink::test]
		fn transfer_works() {
			let mut erc20 = Erc20::new(100);
			assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 100);

			let to_addr = AccountId::from([0x2; 32]);
			erc20.transfer(to_addr, 50).unwrap();

			assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 50);
			assert_eq!(erc20.balance_of(to_addr), 50);
        }
        
        // #[ink::test]
		// fn transfer_from_works() {
        //     let mut erc20 = Erc20::new(100);
		// 	assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 100);

		// 	erc20.approve(AccountId::from([0x2; 32]), 50);
		// 	assert_eq!(erc20.allowance(AccountId::from([0x1; 32]), AccountId::from([0x2; 32])), 50);

        //     let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
        //     .unwrap_or([0x2; 32].into());

		// 	let to_addr = AccountId::from([0x3; 32]);
		// 	erc20.transfer_from(AccountId::from([0x1; 32]), to_addr, 50);

		// 	assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 50);
		// 	assert_eq!(erc20.balance_of(to_addr), 50);
		// }

		#[ink::test]
		fn approve_works() {
			let mut erc20 = Erc20::new(100);
			assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 100);

			erc20.approve(AccountId::from([0x2; 32]), 50).unwrap();
			assert_eq!(erc20.allowance(AccountId::from([0x1; 32]), AccountId::from([0x2; 32])), 50);
        }
        
        #[ink::test]
        fn mint_works() {
            let mut erc20 = Erc20::new(100);
            assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 100);
            
            erc20.mint(AccountId::from([0x2; 32]), 100).unwrap();
            assert_eq!(erc20.balance_of(AccountId::from([0x2; 32])), 100);
            assert_eq!(erc20.total_supply(), 200);
        }

        #[ink::test]
        fn burn_works() {
            let mut erc20 = Erc20::new(100);
            assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 100);
            
            erc20.burn(AccountId::from([0x1; 32]), 50).unwrap();
            assert_eq!(erc20.balance_of(AccountId::from([0x1; 32])), 50);
            assert_eq!(erc20.total_supply(), 50);
        }
	}
}
