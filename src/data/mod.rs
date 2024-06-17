pub mod start_time;

pub async fn connect_to() {
    let connection_string = format!("mongodb://localhost:{}", 27017);
    let client = mongodb::Client::with_uri_str(connection_string)
        .await
        .unwrap();
    let db = client.database("Scenarios");
    let scenario = db.collection::<serde_json::Value>("init");
    let doc = serde_json::json!({"a": 42, "b": true, "c": "Not Found"});
    let inserted = scenario.insert_one(doc, None).await.unwrap();
    println!("{:?}", inserted.inserted_id)
}
