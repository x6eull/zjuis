mod crypto;

use std::env;

use crypto::{encrypt, get_public_key};
use reqwest::{redirect::Policy, ClientBuilder, Error, Response};

pub async fn login(service: &str, username: &str, password: &str) -> Result<Response, Error> {
    // let auth="https://zjuam.zju.edu.cn/cas/oauth2.0/authorize?client_id=TODO&redirect_uri=TODO&response_type=code";
    let client = ClientBuilder::new()
        .cookie_store(true)
        .no_proxy()
        .redirect(Policy::none())
        .build()?;
    let page = client
        .get("https://zjuam.zju.edu.cn/cas/login")
        .query(&[("service", service)])
        .send()
        .await?
        .text()
        .await?;
    let find_anchor = "<input type=\"hidden\" name=\"execution\" value=\"";
    let find_anchor_len = find_anchor.len();
    let start = page.find(find_anchor).unwrap();
    let end = start + find_anchor_len + page[start + find_anchor_len..].find("\"").unwrap();
    let execution = &page[start + find_anchor_len..end];
    //TODO: reuse public key
    let pubkey = get_public_key(&client).await?;
    client
        .post("https://zjuam.zju.edu.cn/cas/login")
        .query(&[("service", service)])
        .form(&[
            ("username", username),
            (
                "password",
                &encrypt(password, &pubkey.exponent, &pubkey.modulus),
            ),
            ("authcode", ""),
            ("execution", execution),
            ("_eventId", "submit"),
        ])
        .send()
        .await
}

#[tokio::main]
async fn main() {
    //TODO: fix 500 error
    let result = login(
        "http://eta.zju.edu.cn/zftal-xgxt-web/teacher/xtgl/index/check.zf",
        &env::var("username").unwrap_or("__username".to_string()),
        &env::var("password").unwrap_or("__password".to_string()),
    )
    .await
    .unwrap();
    let body = result;
    println!("{:?}", body);
}
