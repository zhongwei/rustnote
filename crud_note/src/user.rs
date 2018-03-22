use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::users;

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String
}

impl User {
    pub fn create(user: User, connection: &PgConnection) -> User {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");

        users::table.order(users::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &PgConnection) -> Vec<User> {
        users::table.order(users::id.asc()).load::<User>(connection).unwrap()
    }

    pub fn update(id: i32, user: User, connection: &PgConnection) -> bool {
        let query = diesel::update(users::table.find(id)).set(&user).execute(connection);
        !query.is_err()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        let query = diesel::delete(users::table.find(id)).execute(connection);
        !query.is_err()
    }
}