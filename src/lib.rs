use actix::{Actor, Context};
use mongodb::Client;

pub extern crate mongodb;

pub struct MongoActor{
    pub client: Client
}

impl MongoActor{
    pub async fn new(connection_url: String) ->Self{
        Self{
            client: Client::with_uri_str(
                connection_url
            ).await.expect("Can't create mongo client")
        }

    }
}


impl Actor for MongoActor{
    type Context = Context<Self>;
}
