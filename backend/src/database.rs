use actix::{Actor, Message, SyncContext};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use failure::Fallible;
use r2d2::Pool;

pub struct DatabaseExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}
