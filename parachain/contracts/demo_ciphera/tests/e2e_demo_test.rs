use demo_ciphera::demo_ciphera::{IdentityProvider, IdentityProviderRef, IdentityProviderError};

use ink::env::test;
use ink::prelude::string::ToString;

//#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests {
    use super::*;
    use ink_e2e::{ContractsBackend, E2EBackend};
    use ink::{
        env::{test::default_accounts, DefaultEnvironment},
        primitives::AccountId,
    };
    use ink::env::debug_println;

    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn add_user_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
        // given
        let alice: AccountId = default_accounts::<DefaultEnvironment>().alice;
        let mut constructor = IdentityProviderRef::new(alice);

        let contract = client
            .instantiate("demo_ciphera", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed");
        let mut call_builder = contract.call_builder::<IdentityProvider>();

        // when
        let add_user = call_builder.add_user(
            "John".into(),
            "Doe".into(),
            "Smith".into(),
            "01.01.2000".into(),
            "123 Main St".into(),
            "1234567890".into(),
        );
        client
            .call(&ink_e2e::alice(), &add_user)
            .submit()
            .await
            .expect("Calling `add_user` failed");

        // then
        let value = client
            .call(&ink_e2e::alice(), &call_builder.get_my_data())
            .dry_run()
            .await
            .expect("get_my_data trapped when it shouldn't")
            .return_value();
        assert_eq!(value.unwrap().first_name, "John");

        Ok(())
    }

    #[ink_e2e::test]
    async fn update_user_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
        // given
        let alice: AccountId = default_accounts::<DefaultEnvironment>().alice;
        let mut constructor = IdentityProviderRef::new(alice);

        let contract = client
            .instantiate("demo_ciphera", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed");
        let mut call_builder = contract.call_builder::<IdentityProvider>();

        // add user
        let add_user = call_builder.add_user(
            "John".into(),
            "Doe".into(),
            "Smith".into(),
            "01.01.2000".into(),
            "123 Main St".into(),
            "1234567890".into(),
        );
        client
            .call(&ink_e2e::alice(), &add_user)
            .submit()
            .await
            .expect("Calling `add_user` failed");

        // when
        let update_user = call_builder.update_my_data(
            Some("Jane".into()),
            None,
            None,
            None,
            None,
            None,
        );
        client
            .call(&ink_e2e::alice(), &update_user)
            .submit()
            .await
            .expect("Calling `update_my_data` failed");

        // then
        let value = client
            .call(&ink_e2e::alice(), &call_builder.get_my_data())
            .dry_run()
            .await
            .expect("get_my_data trapped when it shouldn't")
            .return_value();
        assert_eq!(value.unwrap().first_name, "Jane");

        Ok(())
    }

    #[ink_e2e::test]
    async fn delete_user_works<Client: E2EBackend>(mut client: Client) -> E2EResult<()> {
        // given
        let bob: AccountId = default_accounts::<DefaultEnvironment>().bob;
        let mut constructor = IdentityProviderRef::new(bob);

        let contract = client
            .instantiate("demo_ciphera", &ink_e2e::bob(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed");
        let mut call_builder = contract.call_builder::<IdentityProvider>();

        // add user
        let add_user = call_builder.add_user(
            "John".into(),
            "Doe".into(),
            "Smith".into(),
            "01.01.2000".into(),
            "123 Main St".into(),
            "1234567890".into(),
        );
        client
            .call(&ink_e2e::bob(), &add_user)
            .submit()
            .await
            .expect("Calling `add_user` failed");

        // when
        let delete_user = call_builder.delete_my_data();
        let call_res = client
            .call(&ink_e2e::bob(), &delete_user)
            .submit()
            .await
            .expect("Calling `delete_my_data` failed");


        //assert!(call_res.is_ok(), "Calling `delete_my_data` failed: {:?}", call_res);


        let value = client
            .call(&ink_e2e::bob(), &call_builder.get_my_data())
            .dry_run()
            .await
            .expect("get_my_data trapped when it shouldn't")
            .return_value();
        assert!(value.is_err());

        Ok(())
    }
}
