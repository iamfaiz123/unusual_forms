use actix_web:: {web, App, HttpServer};
use mongodb::{Database};
use actix_web::dev:: Server;
use std::net::TcpListener;
use crate::routes::*;



//we send connection to databse from main function to the run function 
pub fn run(listener:TcpListener,db:Database) -> Result<Server, std::io:: Error> 
     {
         let db = web::Data::new(db);
         let server = HttpServer:: new(move|| {
         App:: new()
         .route("/thread/post", web::post().to(post_thread))
         .route("/thread/searchbytital",web::get().to(search_by_tital))
         .route("/thread/read/{threadname}",web::get().to(thread_path))
         .route("/thread/update/{threadname}",web::patch().to(thread_update))
         .route("/thread/addcomment/{threadname}",web::patch().to(add_comment))

         .app_data(db.clone())
       }).listen(listener)?
       .run();

Ok(server)
}