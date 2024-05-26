mod crypto;

use crypto::{encrypt, get_public_key};
use reqwest::{Client, ClientBuilder, Error, Response};

pub async fn login(
    client: &Client,
    service: &str,
    username: &str,
    password: &str,
) -> Result<Response, Error> {
    //TODO: reuse public key
    let pubkey = get_public_key(client).await?;
    client
        .post("https://zjuam.zju.edu.cn/cas/login?service=".to_string() + service)
        .form(&[
            ("username", username),
            (
                "password",
                &encrypt(password, &pubkey.exponent, &pubkey.modulus),
            ),
            //TODO: other fields
        ])
        .send()
        .await
}

#[tokio::main]
async fn main() {
    let client = ClientBuilder::new().no_proxy().build().unwrap();
    //TODO: test
    let result = login(&client, "", "username", "password").await;
    println!("{:?}", result);
}
