use actix_web::*;
use mongodb::*;
pub use mongodb::bson::{doc, Document};



pub async fn thread_path(req: HttpRequest,db:web::Data<Database>,path: web::Path<String>)->impl Responder
{

  log::info!("thread_path::recived a request from :{:?}",req.peer_addr());
   let data = doc!{"tital":path.clone()};
   //creating a pointer to "thread" collection
   let collection = db.collection::<Document>("threads");
   let mut b:Document ; 

  match collection.find_one(data,None).await.expect(" ")
  {
     
        Some(a)=> 
        {
          //if thread with the name exist
          log::info!("thread_path::found path from request from {:?}",req.peer_addr());
          //format!("{:?} \n by :{:?} \n at: {:?} \n {:?} \n \n comment:{:?} ",a.get_str("tital").unwrap(),a.get_str("user_ip").unwrap(),a.get_str("time").unwrap(),a.get_str("post").unwrap(),a.get_document("comments"))
          return serde_json::to_string_pretty(&a).unwrap();

        }
        None => 
        {
          //if thread with tha name does not exits
          log::info!("thread_path:: bad request from :{:?}",req.peer_addr());
          return format!("no tital with name : '{}' exist",path.clone()).to_string()
        }
    
    
};

}