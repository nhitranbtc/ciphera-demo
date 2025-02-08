#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod demo_ciphera {
    use access_control::AccessControlError;
    use access_control::AccessControlRef;
    use ink::env::call::FromAccountId;
    use ink::prelude::{string::String, vec::Vec};
    use ink::storage::Mapping;

    /// Represents a user in the identity provider system.
    #[derive(Debug)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct User {
        pub first_name: String,
        pub middle_name: String,
        pub last_name: String,
        pub date_of_birth: String,
        pub address: String,
        pub phone_number: String,
    }

    /// The main contract struct that holds the storage for the identity provider.
    #[ink(storage)]
    pub struct IdentityProvider {
        users: Mapping<AccountId, User>,
        user_list: Vec<AccountId>,
        admin: AccountId,
        access_control: AccountId,
    }

    /// Define possible errors that can occur in the identity provider contract.
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum IdentityProviderError {
        UserAlreadyExists,
        UserDoesNotExist,
        InvalidInput,
        InvalidName,
        InvalidPhoneNumber,
        InvalidAddress,
        InvalidDateOfBirth,
        AccessDenied,
        NoDataShared,
    }

    /// Type alias for results returned by the contract functions.
    pub type Result<T> = core::result::Result<T, IdentityProviderError>;

    impl IdentityProvider {
        /// Constructor that initializes the contract with the caller as the admin.
        #[ink(constructor)]
        pub fn new(access_control_addr: AccountId) -> Self {
            let caller = Self::env().caller();
            Self {
                users: Mapping::default(),
                user_list: Vec::new(),
                admin: caller,
                access_control: access_control_addr,
            }
        }

        #[ink(constructor)]
        pub fn new_default() -> Self {
            let default_account_id = AccountId::from([0x00; 32]);
            Self::new(default_account_id)
        }

        /// Checks if the given account is the admin.
        fn is_admin(&self, account: AccountId) -> bool {
            account == self.admin
        }

        /// Adds a new user to the identity provider.
        #[ink(message)]
        pub fn add_user(
            &mut self,
            first_name: String,
            middle_name: String,
            last_name: String,
            date_of_birth: String,
            address: String,
            phone_number: String,
        ) -> Result<()> {
            let caller = self.env().caller();
            if self.users.contains(caller) {
                return Err(IdentityProviderError::UserAlreadyExists);
            }
            if first_name.trim().is_empty()
                || last_name.trim().is_empty()
                || date_of_birth.trim().is_empty()
                || address.trim().is_empty()
                || phone_number.trim().is_empty()
            {
                return Err(IdentityProviderError::InvalidInput);
            }

            if !Self::is_valid_name(&first_name, false)
                || !Self::is_valid_name(&middle_name, true)
                || !Self::is_valid_name(&last_name, false)
            {
                return Err(IdentityProviderError::InvalidName);
            }

            if !Self::is_valid_date(&date_of_birth) {
                return Err(IdentityProviderError::InvalidDateOfBirth);
            }

            if !Self::is_valid_address(&address) {
                return Err(IdentityProviderError::InvalidAddress);
            }

            if !Self::is_valid_phone_number(&phone_number) {
                return Err(IdentityProviderError::InvalidPhoneNumber);
            }

            let user = User {
                first_name,
                middle_name,
                last_name,
                date_of_birth,
                address,
                phone_number,
            };

            self.users.insert(caller, &user);
            self.user_list.push(caller);
            Ok(())
        }

        /// Retrieves the caller's data.
        #[ink(message)]
        pub fn get_my_data(&self) -> Result<User> {
            let caller = self.env().caller();
            self.users
                .get(caller)
                .ok_or(IdentityProviderError::UserDoesNotExist)
        }

        /// Retrieves the claims for a user.
        #[ink(message)]
        pub fn get_claims(&self, user: AccountId) -> Result<Vec<(String, String)>> {
            let user_data = match self.users.get(user) {
                Some(u) => u,
                None => return Err(IdentityProviderError::UserDoesNotExist),
            };

            let caller = self.env().caller();
            let ac = AccessControlRef::from_account_id(self.access_control);

            let allowed_claims: Vec<String> =
                ac.get_allowed_claims(user, caller)
                    .map_err(|err| match err {
                        AccessControlError::AccessDenied => IdentityProviderError::AccessDenied,
                        AccessControlError::NoSuchEntry => IdentityProviderError::NoDataShared,
                    })?;

            let mut result = Vec::new();
            for claim in allowed_claims {
                let value = match claim.as_str() {
                    "first_name" => user_data.first_name.clone(),
                    "middle_name" => user_data.middle_name.clone(),
                    "last_name" => user_data.last_name.clone(),
                    "date_of_birth" => user_data.date_of_birth.clone(),
                    "address" => user_data.address.clone(),
                    "phone_number" => user_data.phone_number.clone(),
                    _ => continue,
                };
                result.push((claim, value));
            }
            Ok(result)
        }

        /// Updates the caller's data.
        #[ink(message)]
        pub fn update_my_data(
            &mut self,
            first_name: Option<String>,
            middle_name: Option<String>,
            last_name: Option<String>,
            date_of_birth: Option<String>,
            address: Option<String>,
            phone_number: Option<String>,
        ) -> Result<()> {
            let caller = self.env().caller();

            if let Some(mut user) = self.users.get(caller) {
                if let Some(new_first_name) = first_name {
                    if new_first_name.trim().is_empty()
                        || !Self::is_valid_name(&new_first_name, false)
                    {
                        return Err(IdentityProviderError::InvalidName);
                    }
                    user.first_name = new_first_name;
                }
                if let Some(new_middle_name) = middle_name {
                    if !Self::is_valid_name(&new_middle_name, true) {
                        return Err(IdentityProviderError::InvalidName);
                    }
                    user.middle_name = new_middle_name;
                }
                if let Some(new_last_name) = last_name {
                    if new_last_name.trim().is_empty()
                        || !Self::is_valid_name(&new_last_name, false)
                    {
                        return Err(IdentityProviderError::InvalidName);
                    }
                    user.last_name = new_last_name;
                }
                if let Some(new_dob) = date_of_birth {
                    if new_dob.trim().is_empty() || !Self::is_valid_date(&new_dob) {
                        return Err(IdentityProviderError::InvalidDateOfBirth);
                    }
                    user.date_of_birth = new_dob;
                }
                if let Some(new_address) = address {
                    if new_address.trim().is_empty() || !Self::is_valid_address(&new_address) {
                        return Err(IdentityProviderError::InvalidAddress);
                    }
                    user.address = new_address;
                }
                if let Some(new_phone) = phone_number {
                    if new_phone.trim().is_empty() || !Self::is_valid_phone_number(&new_phone) {
                        return Err(IdentityProviderError::InvalidPhoneNumber);
                    }
                    user.phone_number = new_phone;
                }

                self.users.insert(caller, &user);
                Ok(())
            } else {
                Err(IdentityProviderError::UserDoesNotExist)
            }
        }

        /// Deletes the caller's data.
        #[ink(message)]
        pub fn delete_my_data(&mut self) -> Result<()> {
            let caller = self.env().caller();

            if self.users.contains(caller) {
                self.users.remove(caller);
                self.user_list.retain(|&x| x != caller);
                Ok(())
            } else {
                Err(IdentityProviderError::UserDoesNotExist)
            }
        }

        /// Retrieves a user's data by account ID. Only accessible by the admin.
        #[ink(message)]
        pub fn get_user(&self, account_id: AccountId) -> Result<User> {
            let caller = self.env().caller();
            if self.is_admin(caller) {
                self.users
                    .get(account_id)
                    .ok_or(IdentityProviderError::UserDoesNotExist)
            } else {
                Err(IdentityProviderError::AccessDenied)
            }
        }

        /// Lists all users. Only accessible by the admin.
        #[ink(message)]
        pub fn list_users(&self) -> Result<Vec<AccountId>> {
            let caller = self.env().caller();
            if self.is_admin(caller) {
                Ok(self.user_list.clone())
            } else {
                Err(IdentityProviderError::AccessDenied)
            }
        }

        /// Retrieves the account ID of the admin.
        #[ink(message)]
        pub fn get_admin(&self) -> AccountId {
            self.admin
        }

        /// Changes the admin to a new account ID. Only accessible by the current admin.
        #[ink(message)]
        pub fn change_admin(&mut self, new_admin: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if self.is_admin(caller) {
                self.admin = new_admin;
                Ok(())
            } else {
                Err(IdentityProviderError::AccessDenied)
            }
        }

        /// Validates a name based on specific criteria.
        fn is_valid_name(name: &str, allow_empty: bool) -> bool {
            if name.trim().is_empty() {
                return allow_empty;
            }

            let length_valid = name.len() >= 2 && name.len() <= 20;
            let valid_chars = name
                .chars()
                .all(|c| c.is_alphabetic() || c == '-' || c == ' ');
            let not_start_or_end_with_hyphen = !name.starts_with('-') && !name.ends_with('-');
            let not_start_or_end_with_space = !name.starts_with(' ') && !name.ends_with(' ');
            let no_consecutive_hyphens = !name.contains("--");
            let no_consecutive_spaces = !name.contains("  "); // Fix: Check for consecutive spaces

            length_valid
                && valid_chars
                && not_start_or_end_with_hyphen
                && not_start_or_end_with_space
                && no_consecutive_hyphens
                && no_consecutive_spaces
        }

        /// Validates a phone number based on specific criteria.
        fn is_valid_phone_number(phone_number: &str) -> bool {
            let digits_only = phone_number.chars().all(|c| c.is_digit(10));
            let length_valid = phone_number.len() >= 7 && phone_number.len() <= 15;
            digits_only && length_valid
        }

        /// Validates an address based on specific criteria.
        fn is_valid_address(address: &str) -> bool {
            let length_valid = address.len() >= 6 && address.len() <= 50;
            let includes_digit = address.chars().any(|c| c.is_digit(10));
            length_valid && includes_digit
        }

        /// Validates a date based on specific criteria.
        fn is_valid_date(date: &str) -> bool {
            let parts: Vec<&str> = date.split('.').collect();
            if parts.len() != 3 {
                return false;
            }

            let day = parts[0].parse::<u32>();
            let month = parts[1].parse::<u32>();
            let year = parts[2].parse::<u32>();

            if day.is_err() || month.is_err() || year.is_err() {
                return false;
            }

            let day = day.unwrap();
            let month = month.unwrap();
            let year = year.unwrap();

            day >= 1 && day <= 31 && month >= 1 && month <= 12 && year >= 1900 && year <= 2100
        }
    }
}
