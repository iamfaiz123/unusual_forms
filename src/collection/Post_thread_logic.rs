pub use mongodb::bson::{doc, Document};
pub use mongodb::{options::ClientOptions, Database};
use serde::{Deserialize, Serialize};
use chrono::{Utc};



pub async fn post_thread_logic(user_ip:String,tag1:String,tag2:String,tag3:String,post:String,tital:String,db:&Database)->String
{

 
  let collection = db.collection::<Document>("threads");
  //creating a copy of user id for logging
  let id = user_ip.clone();

  //creating Bson document to insert into collection
  let data = doc!{"user_ip":user_ip,"tag1":tag1,"tag2":tag2,"tag3":tag3,"post":post,"tital":tital,"comments":[],"time":format!("{}",Utc::now())};
   
  match collection.insert_one(data,None).await
   {
    Ok(e)=>
    { 
      log::info!("data has been successfully posted for the user: {} :: {:?}",id,e);
      //response with custom message after successfully insert
       "your thread has been posted".to_string()
    },

    Err(e)=>
    {
        log::error!("failed to post data for {},reason : {:?}",e,id);
        //response with custom data
        format!("failed posting the thread,internal server error ::: {}",e)
    }

  }
  



}
