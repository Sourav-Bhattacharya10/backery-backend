use sea_orm::*;
use rocket::{get, post, State};
use rocket::serde::{json::Json, Deserialize};
use sea_orm::DatabaseConnection;

use crate::entities::bakery;
use crate::error_responder::ErrorResponder;
use crate::entities::prelude::Bakery;

use rocket::http::Status;

#[derive(Deserialize)]
struct BakeryData {
    id: Option<i32>,
    name: String,
    profit_margin: f64
}

#[get("/")]
pub async fn get_all_bakeries(db_conn: &State<DatabaseConnection>) -> Result<Json<Vec<bakery::Model>>, ErrorResponder> {
    let db = db_conn as &DatabaseConnection;

    let bakery_names = Bakery::find()
        .all(db)
        .await
        .map_err(Into::<ErrorResponder>::into)?
        .into_iter()
        .collect::<Vec<bakery::Model>>();

    Ok(Json(bakery_names))
}

#[post("/", data="<bakery_data>")]
pub async fn create_new_bakery(db_conn: &State<DatabaseConnection>, bakery_data: Json<BakeryData>) -> Result<Status, ErrorResponder> {
    let db = db_conn as &DatabaseConnection;

    let new_bakery = bakery::ActiveModel {
        name: ActiveValue::Set(bakery_data.name.to_owned()),
        profit_margin: ActiveValue::Set(bakery_data.profit_margin),
        ..Default::default()
    };

    let res = Bakery::insert(new_bakery).exec(db).await?;
    if res.last_insert_id != 0 {
        Ok(Status::Created)
    }
    else {
        Err(ErrorResponder::from("Something went wrong"))
    }
}
