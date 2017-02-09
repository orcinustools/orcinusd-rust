use rustc_serialize::json::Json;

pub fn to_json(data: &str) -> Json {
    Json::from_str(&data).unwrap()
}
