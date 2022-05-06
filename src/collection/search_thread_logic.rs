pub use mongodb::bson::{doc, Document};

use actix_web::*;
use mongodb::{Database};
pub async fn search_by_tital_logic(db:&Database,tital:String)->impl Responder
{
    //get a pointer to collection where thread data are stored
    let collection = db.collection::<Document>("threads");
    
    let data = doc!{"tital":tital.clone()};


    match collection.find_one(data,None).await.expect(" ")
    {
        Some(a)=> 
            {
              log::info!("found collection");
              format!("{:?} \n by :{:?} \n at: {:?} \n {:?}",a.get_str("tital").unwrap(),a.get_str("user_ip").unwrap(),a.get_str("time").unwrap(),a.get_str("post").unwrap())
            },
            None => 
            {
                log::info!("error to find requested data");
                format!("no tital with name : '{}' exist",tital.clone()).to_string()
            }
    }
} 

pub async fn search_by_tags_logic(db:&Database,t1:String,t2:String,t3:String)->String
{
    let collection = db.collection::<Document>("threads");
    
    //let pipeline = vec![doc! {"$match": {"name": "FOO"}}];

    let data = doc!{"$or":[{"tag1":t1.clone()},{"tag2":t2.clone()},{"tag3":t3.clone()}]};

    match collection.find_one(data,None).await.expect("failed to fetch query")
    {
        Some(a)=>
        {format!("post that has one of the tag {},{},{} are \n {:?} \n by: {:?} \n  at: {:?} \n {:?}",t1.clone(),t2.clone(),t3.clone(),a.get_str("tital").unwrap(),a.get_str("user_ip").unwrap(),a.get_str("time").unwrap(),a.get_str("post").unwrap())
    }
        None=>format!("can not find any threads having tags {} , {} , {}",t1,t2,t3)                

    }
    
    

    
    
} 