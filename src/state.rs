use cw_storage_plus::{ Map};


/// Track used hashes for uniqueness
/// Key: hash string, Value: always true
pub const USED_HASHES: Map<&str, bool> = Map::new("used_hashes");
