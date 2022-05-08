#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Content {
    pub raw: String,
}

pub fn get_raw(url: String) -> () {
    let async_client = RestClient::new(url).unwrap();

    async_client.get::<_, Content>(()).unwrap()
}
