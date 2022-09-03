//! general db implementation
// FIXME: move this in another crate
use rusty_leveldb::{Options, DB};
use std::fmt;

pub struct GalaxyDB {
    db: DB,
}

impl GalaxyDB {
    /// Crete the new instance the galaxy db with a particular name.
    pub fn new(path: &str, db_name: &str) -> Self {
        let opts = Options::default();
        GalaxyDB {
            db: DB::open(format!("{}/{}", path, db_name), opts).unwrap(),
        }
    }

    pub fn close<'a>(&'a mut self) -> Result<(), GalaxyDBErr> {
        match self.db.close() {
            Ok(_) => Ok(()),
            Err(err) => Err(GalaxyDBErr {
                msg: format!("{}", err),
            }),
        }
    }
}

pub struct GalaxyDBErr {
    msg: String,
}

impl fmt::Display for GalaxyDBErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
