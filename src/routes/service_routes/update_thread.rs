use actix_web::*;
use mongodb::*;
use serde::Deserialize;
pub use mongodb::bson::{doc, Document};
pub use crate::collection::*;

#[derive(Deserialize)]
pub struct content{
    post:String

}




pub async fn thread_update(req: HttpRequest,db:web::Data<Database>,path: web::Path<String>,tital:web::Json<content>)->impl Responder
{
    
    let collection = db.collection::<Document>("threads");
  
    let a = collection.find_one(doc!{"tital":path.clone()},None).await.expect("interal server error");
    let a = a.unwrap();
    let ip = match req.peer_addr()
    {
        Some(a)=> a.ip().to_string(),
        None=>
        {
            log::info!("update_thread:: failed to parse ip for client: {:?}",req.peer_addr());
           "i am sad ya know ya".to_string()
        }
    };
    //these ips need to stored in variables for comparison
    let client_ip = format!("{:?}",ip).trim().to_string();
    let post_ip = a.get_str("user_ip").unwrap().trim().to_string();
    log::info!("update_thread::recieved a request form {:?}",client_ip);

    //if thread ip matches the ip of the client 
    if  client_ip.ne(&post_ip)
    {
        //call a function to update the thread content
        log::info!("update_thread :: ip matched for cleint:{:?} calling function now", client_ip);
        thread_update_logic(&db,path.clone(),tital.post.clone()).await
    }
    else 
    {
       
        log::info!("update_thread :: client ip :{:?} does not match the thread ip", client_ip);
        //response with custome message if client ip is not same as thread ip
        format!("you dont have authority to alter posts made by others")
    }
    
}
