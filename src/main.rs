extern crate serde_yaml;
extern crate failure;
extern crate tracy;

use std::io::{BufReader};
use std::fs;
use tracy::{Scene};

fn read_scene(path: &str) -> Result<Scene, failure::Error> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let val = serde_yaml::from_reader(reader)?;

    Ok(val)
}

fn main() {
    let scene = read_scene("scene.yml").unwrap();
    scene.write("./out/image.ppm").unwrap();
}
