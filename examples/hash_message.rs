use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use firecore_util::text::MessageSet;

fn main() {

    let message = MessageSet::default();

    let mut hasher = DefaultHasher::new();
    message.hash(&mut hasher);
    let h1 = hasher.finish();

    let mut hasher = DefaultHasher::new();
    message.hash(&mut hasher);
    let h2 = hasher.finish();

    println!("Default message hashes: {}, {}", h1, h2);

}