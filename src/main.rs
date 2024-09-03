// mod entities;
//
// use sea_orm::*;
// use sea_orm::sea_query::{Expr, JoinType};
// use serde_json::json;
// use entities::{prelude::*, *};
// use crate::entities::bakery::ActiveModel;
//
// const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/postgres";
//
// async fn run() -> Result<(), DbErr> {
//     let db_conn = Database::connect(DATABASE_URL).await?;
//     let db = db_conn.get_database_backend();
//
//     if db == DbBackend::Postgres {
//         // db_conn.execute(Statement::from_string(db, format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME))).await?;
//         // db_conn.execute(Statement::from_string(db, format!("CREATE DATABASE \"{}\";", DB_NAME))).await?;
//
//         // Database::connect(DATABASE_URL).await?;
//
//         // create_records(&db_conn).await?;
//
//         show_records(&db_conn).await?;
//     }
//
//     Ok(())
// }
//
// async fn create_records(db_conn: &DatabaseConnection) -> Result<(), DbErr> {
//     // let sad_bakery = bakery::ActiveModel {
//     //     name: ActiveValue::Set("Sad Bakery".to_owned()),
//     //     profit_margin: ActiveValue::Set(0.0),
//     //     ..Default::default()
//     // };
//
//     // sad_bakery.insert(db_conn).await?;
//
//     // let la_boulangerie = bakery::ActiveModel {
//     //     name: ActiveValue::Set("La Boulangerie".to_owned()),
//     //     profit_margin: ActiveValue::Set(0.0),
//     //     ..Default::default()
//     // };
//
//     // let la_bakery_res = Bakery::insert(la_boulangerie).exec(db_conn).await?;
//
//     // let la_chefs: Vec<chef::ActiveModel> = ["Jolie", "Charles", "Madeleine", "Frederic"].into_iter().map(|chef_name|
//     // chef::ActiveModel {
//     //     name: ActiveValue::Set(chef_name.to_owned()),
//     //     bakery_id: ActiveValue::Set(3),
//     //     contact_details: ActiveValue::Set(json!({
//     //         "mobile": "9876543210"
//     //     })),
//     //     ..Default::default()
//     // }
//     // ).collect();
//
//     // let _la_chef_res = Chef::insert_many(la_chefs).exec(db_conn).await?;
//
//     let sad_charles = chef::ActiveModel {
//         name: ActiveValue::Set("Charles".to_owned()),
//         bakery_id: ActiveValue::Set(2),
//         contact_details: ActiveValue::Set(json!({
//             "mobile": "9876543211"
//         })),
//         ..Default::default()
//     };
//
//     sad_charles.insert(db_conn).await?;
//
//     Ok(())
// }
//
// async fn show_records(db_conn: &DatabaseConnection) -> Result<(), DbErr> {
//     // let bakeries: Vec<bakery::Model> = Bakery::find().all(db_conn).await?;
//     // println!("Bakeries : {:?}",bakeries);
//
//     // // Find by ID
//     // let _sad_bakery_id = Bakery::find_by_id(1).one(db_conn).await?;
//
//     // // Finding by arbitrary column with `filter()`
//     // let _sad_bakery_name = Bakery::find()
//     //     .filter(bakery::Column::Name.eq("Sad bakery"))
//     //     .one(db_conn).await?;
//
//     // let chefs: Vec<chef::Model> = Chef::find().all(db_conn).await?;
//     // println!("Chefs : {:?}",chefs);
//
//
//     // let bakeries = Bakery::find()
//     //     .filter(
//     //         Condition::any()
//     //             .add(bakery::Column::Id.eq(2))
//     //             .add(bakery::Column::Id.eq(3))
//     //     ).all(db_conn).await?;
//     // println!("Bakeries : {:?}", bakeries);
//
//     // // Then use loader to load the chefs in one query.
//     // let chefs: Vec<Vec<chef::Model>> = bakeries.load_many(Chef, db_conn).await?;
//     // println!("All Chefs : {:?}", chefs);
//
//     // let sad_chefs = chefs[0].to_owned();
//     // println!("Sad Chefs : {:?}", sad_chefs);
//
//     // let la_chefs = chefs[1].to_owned();
//     // println!("La Chefs : {:?}", la_chefs);
//
//     // Search in which bakeries Charles work
//     // let charles_bakeries = Chef::find()
//     //     .column_as()
//     //     .filter(chef::Column::Name.eq("Charles"))
//     //     .find_also_related(Bakery)
//     //     .all(db_conn)
//     //     .await?; // Method1 - working OK
//     // OR
//     let charles_bakeries = Chef::find()
//         .filter(chef::Column::Name.eq("Charles"))
//         .select_only()
//         .column_as(bakery::Column::Name, "bakery_name")
//         .inner_join(Bakery)
//         .into_tuple::<(String)>()
//         .all(db_conn)
//         .await?; // Method 2 - working OK
//
//     println!("Charles Bakeries : {:?}", charles_bakeries);
//
//     Ok(())
// }
//
// async fn update_records(db_conn: &DatabaseConnection, res: InsertResult<ActiveModel>) -> Result<(), DbErr> {
//     // Update the 1st record's name in the bakery table
//     let sad_bakery = bakery::ActiveModel {
//         id: ActiveValue::Set(res.last_insert_id),
//         name: ActiveValue::Set("Sad Bakery".to_owned()),
//         profit_margin: ActiveValue::NotSet
//     };
//
//     sad_bakery.insert(db_conn).await?;
//
//     let la_boulangerie = bakery::ActiveModel {
//         name: ActiveValue::Set("La Boulangerie".to_owned()),
//         profit_margin: ActiveValue::Set(0.0),
//         ..Default::default()
//     };
//
//     let la_bakery_res = Bakery::insert(la_boulangerie).exec(db_conn).await?;
//
//     let la_chefs = ["Jolie", "Charles", "Madeleine", "Frederic"].into_iter().map(|chef_name|
//         chef::ActiveModel {
//             name: ActiveValue::Set(chef_name.to_owned()),
//             bakery_id: ActiveValue::Set(la_bakery_res.last_insert_id),
//             ..Default::default()
//         }
//     );
//
//     let _la_chef_res = Chef::insert_many(la_chefs).exec(db_conn).await?;
//
//     Ok(())
// }
//
// async fn delete_records(db_conn: &DatabaseConnection) -> Result<(), DbErr> {
//     let john = chef::ActiveModel {
//         id: ActiveValue::Set(1), // The primary key must be set
//         ..Default::default()
//     };
//     john.delete(db_conn).await?;
//
//     let sad_bakery = bakery::ActiveModel {
//         id: ActiveValue::Set(1), // The primary key must be set
//         ..Default::default()
//     };
//     sad_bakery.delete(db_conn).await?;
//
//     Ok(())
// }
//
// #[tokio::main]
// async fn main() {
//     if let Err(err) = run().await {
//         panic!("{}", err);
//     }
// }

mod setup;
mod entities;
mod error_responder;
mod bakeries_routes;

use rocket::*;

use crate::setup::setup_db;
use crate::bakeries_routes::{get_all_bakeries, create_new_bakery};

#[launch] // the "main" function of the program
async fn rocket() -> _ {
    let db_conn = match setup_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err)
    };

    rocket::build().manage(db_conn)
        .mount("/", routes![
            health
        ])
        .mount("/bakeries", routes![
            get_all_bakeries,
            create_new_bakery
        ])
}

#[get("/health")]
async fn health() -> &'static str {
    "OK Hello World"
}