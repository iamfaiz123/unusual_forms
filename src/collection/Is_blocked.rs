use mongodb::Client;
pub use mongodb::bson::{doc, Document};
pub async fn is_blocked(ip:&String,client:Client)->bool
{
    //we are using differnt database for storing user_ip
   let blocked = client.database("blocked_ip");
    let collection = blocked.collection::<Document>("blockedip");
    let data = doc!{"ip" : ip.clone()};
    match collection.find_one(data,None).await.expect("")
    {
        Some(s) => true,
        _ => false
    }

}