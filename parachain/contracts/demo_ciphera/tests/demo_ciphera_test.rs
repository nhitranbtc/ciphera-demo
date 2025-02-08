use demo_ciphera::demo_ciphera::{IdentityProvider, IdentityProviderError};

use ink::env::test;
use ink::prelude::string::ToString;

#[ink::test]
fn test_add_user_success() {
    let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
    let mut contract = IdentityProvider::new(accounts.alice);

    let result = contract.add_user(
        "John".to_string(),
        "Doe".to_string(),
        "Smith".to_string(),
        "01.01.2000".to_string(),
        "123 Main St".to_string(),
        "1234567890".to_string(),
    );

    assert!(result.is_ok());
}

#[ink::test]
fn test_add_user_already_exists() {
    let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
    let mut contract = IdentityProvider::new(accounts.alice);

    let _ = contract.add_user(
        "John".to_string(),
        "Doe".to_string(),
        "Smith".to_string(),
        "01.01.2000".to_string(),
        "123 Main St".to_string(),
        "1234567890".to_string(),
    );

    let result = contract.add_user(
        "John".to_string(),
        "Doe".to_string(),
        "Smith".to_string(),
        "01.01.2000".to_string(),
        "123 Main St".to_string(),
        "1234567890".to_string(),
    );

    assert_eq!(result, Err(IdentityProviderError::UserAlreadyExists));
}

#[ink::test]
fn test_add_user_invalid_input() {
    let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
    let mut contract = IdentityProvider::new(accounts.alice);

    let result = contract.add_user(
        "".to_string(),
        "Doe".to_string(),
        "Smith".to_string(),
        "01.01.2000".to_string(),
        "123 Main St".to_string(),
        "1234567890".to_string(),
    );

    assert_eq!(result, Err(IdentityProviderError::InvalidInput));
}

#[ink::test]
fn test_add_user_invalid_name() {
    let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
    let mut contract = IdentityProvider::new(accounts.alice);

    let result = contract.add_user(
        "J".to_string(),
        "Doe".to_string(),
        "Smith".to_string(),
        "01.01.2000".to_string(),
        "123 Main St".to_string(),
        "1234567890".to_string(),
    );

    assert_eq!(result, Err(IdentityProviderError::InvalidName));
}

#[ink::test]
fn test_add_user_invalid_date_of_birth() {
    let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
    let mut contract = IdentityProvider::new(accounts.alice);

    let result = contract.add_user(
        "John".to_string(),
        "Doe".to_string(),
        "Smith".to_string(),
        "32.01.2000".to_string(),
        "123 Main St".to_string(),
        "1234567890".to_string(),
    );

    assert_eq!(result, Err(IdentityProviderError::InvalidDateOfBirth));
}

#[ink::test]
fn test_add_user_invalid_address() {
    let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
    let mut contract = IdentityProvider::new(accounts.alice);

    let result = contract.add_user(
        "John".to_string(),
        "Doe".to_string(),
        "Smith".to_string(),
        "01.01.2000".to_string(),
        "Main St".to_string(),
        "1234567890".to_string(),
    );

    assert_eq!(result, Err(IdentityProviderError::InvalidAddress));
}

#[ink::test]
fn test_add_user_invalid_phone_number() {
    let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
    let mut contract = IdentityProvider::new(accounts.alice);

    let result = contract.add_user(
        "John".to_string(),
        "Doe".to_string(),
        "Smith".to_string(),
        "01.01.2000".to_string(),
        "123 Main St".to_string(),
        "12345".to_string(),
    );

    assert_eq!(result, Err(IdentityProviderError::InvalidPhoneNumber));
}
