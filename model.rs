use mongodb::bson::{doc, Bson};
use mongodb::{Client, Collection};

 // Relate the Address by passing the json data.


// #[derive(Debug, Serialize, Deserialize)]
// struct Address {
//     city: String,
//     country: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct User {
//     name: String,
//     age: i32,
//     address: Address,
// }

// // Example of inserting a user with an embedded address
// async fn insert_user(users: Collection<User>) -> mongodb::error::Result<()> {
//     let address = Address {
//         city: "New York".to_string(),
//         country: "USA".to_string(),
//     };
//     let user = User {
//         name: "John Doe".to_string(),
//         age: 30,
//         address: address,
//     };
//     users.insert_one(user, None).await?;
//     Ok(())
// }

// Reference them by ID

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    city: String,
    country: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: i32,
    address_id: ObjectId, // Reference to an Address document
}
// Example of inserting a user with a reference to an address
async fn insert_user(users: Collection<User>, address_id: ObjectId) -> mongodb::error::Result<()> {
    let user = User {
        name: "John Doe".to_string(),
        age: 30,
        address_id: address_id,
    };
    users.insert_one(user, None).await?;
    Ok(())
}
