use std::error::Error;
use std::ops::Deref;
use rocket::request::{self, FromRequest};
use rocket::{Request, Outcome, State};
use super::AppState;

use csv;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub notes: String,
}

pub fn load(csv_file: &str) -> Result<Vec<Product>, Box<Error>> {
    let mut reader = csv::Reader::from_path(csv_file)?;

    Ok(reader
           .deserialize()
           .skip(1)
           .map(|record| {
                    let row: Product = record.unwrap();
                    row
                })
           .collect())
}

pub struct ProductList(pub Vec<Product>);

impl<'a, 'r> FromRequest<'a, 'r> for ProductList {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ProductList, ()> {
        Outcome::Success(ProductList(request
                                         .guard::<State<AppState>>()?
                                         .product_list
                                         .to_vec()))
    }
}

impl<'a> Deref for ProductList {
    type Target = Vec<Product>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}