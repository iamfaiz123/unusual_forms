use actix_web::*;
use mongodb::Database;
pub use crate::collection::*;
use serde::{Deserialize};


//required json structure for posting a
#[derive(Deserialize)]
pub struct thread
{
    tital:String,
    post:String,
    tag1:String,
    tag2:String,
    tag3:String,
}

pub async fn post_thread(req: HttpRequest,form: web::Json<thread>,db : web::Data<Database>)->impl Responder
{
   //capture if user's Ip adderess
   let client_ip = match req.peer_addr()
   {
       Some(a) => 
       { 
           a.ip().to_string()
       },

       None=> "no ip add".to_string()

   };

  log::info!("recived a thread post request from :{} ",client_ip);
  //call function to post thread
  let a = post_thread_logic(client_ip.clone(), form.tag1.clone() , form.tag2.clone() , form.tag3.clone() , form.post.clone() , form.tital.clone() ,&db).await;
  return a;
       
   
}

   

