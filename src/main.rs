mod graphql_schema;
use crate::graphql_schema::*;
mod schema;

#[macro_use]
extern crate juniper;
extern crate diesel;


use std::io;
use std::sync::Arc;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

//  '/graphql'
fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

// '/graphiql'
fn graphiql() -> HttpResponse { 
    let html = graphiql_source("http://localhost:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}


 fn main() -> io::Result<()> {
    dotenv().ok();
    let schema = Arc::new(create_schema());
    //  This allows shared immutable state across the threads 

    HttpServer::new(move || { 
        //  The closure takes ownership of schema+
        App::new()
        .data(schema.clone())
        .service(web::resource("/graphql").route(web::post().to_async(graphql)))
        .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .workers(1)
    
    .bind("127.0.0.1:8080")?
    
    .run()
   

}


