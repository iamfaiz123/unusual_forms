pub use mongodb::bson::{doc, Document};

use actix_web::*;
use mongodb::{Database};
pub async fn thread_update_logic(db:&Database,tital:String,content:String)->String
{   
    //get a pointer to the thread collection
    let collection = db.collection::<Document>("threads");

    let filter = doc!{"tital":tital};
    let update = doc!{"$set": {"post":content}};
    
    //send the data base query result to the client
    let apple =  collection.update_one(filter,update,None).await.expect("fail to update");
    format!("{:?}",apple)
    

}