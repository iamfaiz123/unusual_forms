use actix_web::*;
use mongodb::Database;
pub use crate::collection::*;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct comment
{
  content:String
}

pub async fn add_comment(req: HttpRequest,db:web::Data<Database>,path: web::Path<String>,comment:web::Json<comment>)->impl Responder
{
  //no need to make module to insert comment, we can do it in a single module
  let collection = db.collection::<Document>("threads");
  //get ip address of client
  let client_ip = match req.peer_addr()
  {
      Some(a)=> a.ip().to_string(),
      None=>
      {
          log::info!("update_thread:: failed to parse ip for client: {:?}",req.peer_addr());
         "i am sad ya know ya".to_string()
      }
  };



let filter = doc!{"tital":path.clone()};

let data = doc!{"$push":{"comments":{"by":client_ip.clone(),"content":comment.content.clone()}}};

collection.update_one(filter,data,None).await;
format!("commet has been added")


}