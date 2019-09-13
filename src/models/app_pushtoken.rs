// #![allow(unused_imports)]

extern crate diesel;

use diesel::prelude::*;
// use diesel::sql_types::Timestamptz;
use serde::{Serialize, Deserialize};

// use crate::*;
use crate::models::*;
use crate::helpers::AppResult;
use crate::helpers::db_establish_connection;


#[derive(Debug, Default, Deserialize, Serialize, Queryable)]
pub struct AppPushtoken {
    id: i32,
    token: String,
    created: i32,
}

impl AppPushtoken {
    pub fn fetch() -> AppResult<Self> {
        use self::schema::pushtoken::dsl::*;

        let conn = db_establish_connection();

        let res = pushtoken
            // .filter(published.eq(true))
            .limit(5)
            .load::<Self>(&conn)
            .expect("Error loading data");

        println!(">>> DB result: {:?}", &res);

        Ok(Self { ..Default::default() })
    }
}
