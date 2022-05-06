use serde::*;
use actix_web::*;
use crate::collection::*;
use mongodb::Database;
#[derive(Deserialize)]

pub struct Tital
{
   tital:String
}


pub async fn search_by_tital(req: HttpRequest,userData: web::Form<Tital>,db :web::Data<Database>)->impl Responder
{
    log::info!("thread_search:: recieved a request from :{:?}, query:{} ",req.peer_addr(),userData.tital);
    //call function to search thread with asked tital
    search_by_tital_logic(&db,userData.tital.clone()).await
}

//search by tags havent implmented yet