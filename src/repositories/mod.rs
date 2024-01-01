use crate::models::Product;
use crate::models::NewProduct;
use crate::database::products;
use diesel::{
    query_dsl::methods::{FindDsl, LimitDsl, SelectDsl, OrderDsl},
    ExpressionMethods,
    QueryResult, 
    SqliteConnection,
    RunQueryDsl
};


pub struct ProductRepository;

impl ProductRepository {
    pub fn find_all(c: &mut SqliteConnection) -> QueryResult<Vec<Product>> {
        products::table.limit(100).load::<Product>(c)
    }

    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Product> {
        products::table.find(id).get_result::<Product>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_product: NewProduct) -> QueryResult<Product> {
        diesel::insert_into(products::table)
            .values(new_product)
            .execute(c)?;

        let last_id = Self::last_id(c)?;
        Self::find(c, last_id)
    }

    fn last_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        products::table.select(products::id).order(products::id.desc()).first(c)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(products::table.find(id)).execute(c)
    } 

    pub fn save(c: &mut SqliteConnection, product: Product) -> QueryResult<Product> {
        diesel::update(products::table.find(product.id))
            .set((
                products::name.eq(product.name.to_owned()),
                products::description.eq(product.description.to_owned())
            ))
            .execute(c)?;
        Self::find(c, product.id)
    }
}