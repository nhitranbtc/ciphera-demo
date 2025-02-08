#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::access_control::{AccessControl, AccessControlError, AccessControlRef};

#[ink::contract]
mod access_control {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum AccessControlError {
        AccessDenied,
        NoSuchEntry,
    }

    pub type Result<T> = core::result::Result<T, AccessControlError>;

    #[ink(storage)]
    pub struct AccessControl {
        allowed_claims: Mapping<(AccountId, AccountId), Vec<String>>,
        admin: AccountId,
        trusted_contracts: Mapping<AccountId, bool>,
    }

    /// Defines the `AccessControl` struct, which holds the contract's storage.
    /// - `allowed_claims`: A mapping of allowed claims between users and relying parties.
    /// - `admin`: The account ID of the contract's admin.
    /// - `trusted_contracts`: A mapping of trusted contract addresses.

    impl AccessControl {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                allowed_claims: Mapping::default(),
                admin: caller,
                trusted_contracts: Mapping::default(),
            }
        }

        /// Constructor that initializes the contract with the caller as the admin.
        fn is_admin(&self) -> bool {
            self.env().caller() == self.admin
        }

        /// Checks if the caller is the admin.
        #[ink(message)]
        pub fn grant_access(
            &mut self,
            relying_party: AccountId,
            claims: Vec<String>,
        ) -> Result<()> {
            let caller = self.env().caller();
            self.allowed_claims.insert((caller, relying_party), &claims);
            Ok(())
        }

        /// Grants access by adding claims for a relying party.
        /// - `relying_party`: The account ID of the relying party.
        /// - `claims`: A vector of claims to be granted.
        #[ink(message)]
        pub fn revoke_access(&mut self, relying_party: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if self.allowed_claims.contains((caller, relying_party)) {
                self.allowed_claims.remove((caller, relying_party));
                Ok(())
            } else {
                Err(AccessControlError::NoSuchEntry)
            }
        }

        #[ink(message)]
        pub fn get_allowed_claims(
            &self,
            user: AccountId,
            relying_party: AccountId,
        ) -> Result<Vec<String>> {
            let caller = self.env().caller();
            let is_user = caller == user;
            let is_relying_party = caller == relying_party;
            let is_trusted_contract = self.trusted_contracts.contains(caller);

            if !(is_user || is_relying_party || is_trusted_contract) {
                return Err(AccessControlError::AccessDenied);
            }

            let allowed_claims = self
                .allowed_claims
                .get((user, relying_party))
                .unwrap_or_default();

            if allowed_claims.is_empty() {
                return Err(AccessControlError::NoSuchEntry);
            }
            Ok(allowed_claims)
        }

        /// Retrieves the allowed claims for a user and relying party.
        /// - `user`: The account ID of the user.
        /// - `relying_party`: The account ID of the relying party.
        #[ink(message)]
        pub fn admin_revoke(&mut self, user: AccountId, relying_party: AccountId) -> Result<()> {
            if !self.is_admin() {
                return Err(AccessControlError::AccessDenied);
            }

            if self.allowed_claims.contains((user, relying_party)) {
                self.allowed_claims.remove((user, relying_party));
                Ok(())
            } else {
                Err(AccessControlError::NoSuchEntry)
            }
        }

        /// Allows the admin to revoke access by removing claims for a user and relying party.
        /// - `user`: The account ID of the user.
        /// - `relying_party`: The account ID of the relying party.
        #[ink(message)]
        pub fn get_admin(&self) -> AccountId {
            self.admin
        }
        /// Retrieves the account ID of the admin.

        #[ink(message)]
        pub fn set_admin(&mut self, new_admin: AccountId) -> Result<()> {
            if !self.is_admin() {
                return Err(AccessControlError::AccessDenied);
            }
            self.admin = new_admin;
            Ok(())
        }

        /// Sets a new admin.
        /// - `new_admin`: The account ID of the new admin.
        #[ink(message)]
        pub fn add_trusted_contract(&mut self, contract: AccountId) -> Result<()> {
            if !self.is_admin() {
                return Err(AccessControlError::AccessDenied);
            }
            self.trusted_contracts.insert(contract, &true);
            Ok(())
        }

        /// Adds a trusted contract.
        /// - `contract`: The account ID of the contract to be trusted.
        #[ink(message)]
        pub fn remove_trusted_contract(&mut self, contract: AccountId) -> Result<()> {
            if !self.is_admin() {
                return Err(AccessControlError::AccessDenied);
            }
            self.trusted_contracts.remove(contract);
            Ok(())
        }
    }
}

impl Default for AccessControl {
    fn default() -> Self {
        Self::new()
    }
}
