use include_dir::{include_dir, Dir};

pub static ASSETS_DIR: Dir<'_> = include_dir!("assets");
