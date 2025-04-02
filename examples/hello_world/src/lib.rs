
extern crate mls_rs_crypto_webcrypto;

use wasm_bindgen::prelude::*;
use rand::Rng;
use wasm_bindgen::prelude::*;
use mls_rs::{
    client_builder::{ClientBuilder, MlsConfig},
    error::MlsError,
    identity::{
        basic::{BasicCredential, BasicIdentityProvider},
        SigningIdentity,
    },
    mls_rules::{CommitOptions, DefaultMlsRules},
    CipherSuite, CipherSuiteProvider, Client, CryptoProvider, ExtensionList,
    
};
use mls_rs::ProtocolVersion;

use mls_rs_crypto_webcrypto::WebCryptoProvider;

const CIPHERSUITE: CipherSuite = CipherSuite::P256_AES128;


async fn make_client<P: CryptoProvider + Clone>(
    crypto_provider: P,
    name: &str,
) -> Result<Client<impl MlsConfig>, MlsError> {
    let cipher_suite = crypto_provider.cipher_suite_provider(CIPHERSUITE).unwrap();
    // Generate a signature key pair.
    let (secret, public) = cipher_suite.signature_key_generate().await.unwrap();

    // Create a basic credential for the session.
    // NOTE: BasicCredential is for demonstration purposes and not recommended for production.
    // X.509 credentials are recommended.
    let basic_identity = BasicCredential::new(name.as_bytes().to_vec());
    let signing_identity = SigningIdentity::new(basic_identity.into_credential(), public);
    let mls_rules =
        DefaultMlsRules::default().with_commit_options(None.unwrap_or_default());

    Ok(ClientBuilder::new()
        .identity_provider(BasicIdentityProvider::new())
        .crypto_provider(crypto_provider.clone())
        .signing_identity(signing_identity, secret, CIPHERSUITE)
        .build())


        // let mut builder = ClientBuilder::new()
        // .crypto_provider(crypto.clone())
        // .identity_provider(BasicIdentityProvider::new())
        // .mls_rules(mls_rules)
        // .psk(
        //     ExternalPskId::new(TEST_EXT_PSK_ID.to_vec()),
        //     make_test_ext_psk().into(),
        // )
        // .used_protocol_version(protocol_version)
        // .signing_identity(identity, secret_key, cipher_suite);


}

pub async fn basic_mlsrs_usage() -> Result<(), MlsError> {
    let crypto_provider = mls_rs_crypto_webcrypto::WebCryptoProvider::default();

    // Create clients for Alice and Bob
    let alice = make_client(crypto_provider.clone(), "alice").await?;
    let bob = make_client(crypto_provider.clone(), "bob").await?;

    // Alice creates a new group.
    let bob_key_pkg = bob
    .generate_key_package_message(Default::default(), Default::default())
    .await
    .unwrap();


    let mut alice_group = alice
    .create_group_with_id(b"group".to_vec(), Default::default(), Default::default())
    .await
    .unwrap();

    let welcome = &alice_group
        .commit_builder()
        .add_member(bob_key_pkg)
        .unwrap()
        .build()
        .await
        .unwrap()
        .welcome_messages[0];

    // Upon server confirmation, alice applies the commit to her own state
    alice_group.apply_pending_commit().await.unwrap();

    // Bob receives the welcome message and joins the group
    let (bob_group, _) = bob.join_group(None, welcome).await.unwrap();

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}


#[wasm_bindgen]
pub async fn basic_example() {
    let res = basic_mlsrs_usage().await;
    println!("Return value of basic_mlsrs_usage is: {:?}", res);
}