const CLIENT_ID: &str = "";

#[tokio::main]
async fn main() {
    let client = trakt_rs::TraktApi::new(CLIENT_ID.parse().unwrap());

    // let response = client.execute(trakt_rs::api::movies::trending::Request::default()).await.unwrap();
    //
    // println!("{:#?}", response);
}
