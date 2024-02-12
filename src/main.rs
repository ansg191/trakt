const CLIENT_ID: &str = "864ba5eafb3a02a474f43aa3422a181dfe34731b33c9f8fc845b9fbb9bc9b523";

#[tokio::main]
async fn main() {
    let client = trakt_rs::TraktApi::new(CLIENT_ID.parse().unwrap());

    // let response = client.execute(trakt_rs::api::movies::trending::Request::default()).await.unwrap();
    //
    // println!("{:#?}", response);
}
