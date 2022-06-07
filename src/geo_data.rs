use geo::{Coordinate, LineString, Polygon, Point};
use regex::Regex;
use geo::prelude::Contains;

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[allow(dead_code)]
pub fn geo_converter_to_coordinate (raw_coordinates :  &str) -> Coordinate<f32> {
    let regex = Regex::new(r"[,]").unwrap();
    let splits : Vec<_> = regex.split(raw_coordinates).into_iter().collect();

    let x : f32;
    let y : f32;


    x = splits[0].parse::<f32>().unwrap();
    y = splits[1].parse::<f32>().unwrap();

    Coordinate {
        x : x,
        y : y
    }
}
#[allow(dead_code)]
pub fn geo_converter_to_point (raw_coordinates : &str) -> Point<f32> {
    let regex = Regex::new(r"[,]").unwrap();
    let splits : Vec<_> = regex.split(raw_coordinates).into_iter().collect();

    let x : f32;
    let y : f32;


    x = splits[0].parse::<f32>().unwrap();
    y = splits[1].parse::<f32>().unwrap();

    let point = Point::new(x, y);
    return point;
    
}
#[allow(dead_code)]
fn string_to_vec(input : &mut String) -> Vec<&str> {
    let input_str = input.as_str();
    let mut storage : Vec<&str> = vec![];
    let re = Regex::new("-?[0-9]+.[0-9]+,-[0-9]+.[0-9]+").unwrap();
    for cap in re.captures_iter(input_str) {
        storage.push(&cap.get(0).unwrap().as_str());
    }
    return storage;

    
    
}

pub fn creating_boundaries(coordinates : Vec<&str>) -> Polygon<f32> {
    let mut geo_coordinates : Vec<Coordinate<f32>> = vec![];

    for coordinate in coordinates.into_iter() {
        geo_coordinates.push(geo_converter_to_coordinate(coordinate))
    }

    let line_string : LineString<f32> = geo_coordinates.into_iter().collect();

    let polygon = Polygon::new(line_string, vec![]);

    return polygon
}
#[allow(dead_code)]
pub fn in_boundaries(geometry : &Polygon<f32>, point : &Point<f32>) -> &'static str {
    
    if geometry.contains(point) {
        return "true";
    } else {
        return "false";
    }
}

pub fn in_boundaries_result(input : &Vec<Destination>, location : &String) -> (String, String) {
    let mut string_storage : (String, String) = ("".to_string(),"".to_string());
    let string = &*location;
    let point = geo_converter_to_point(string);
    let mut check : &str = "false";
    
    for item in input.into_iter() {
        check = in_boundaries(&item.area, &point);
        if check == "true" {
            string_storage = (item.name.clone(), item.obj_path.clone());
            break;
        } else {
            string_storage = ("None".to_string(),"None".to_string());
        }
    }

    return string_storage; 

}

pub fn transform_vec_to_state(input : Vec<(String, String, String)>) -> Vec<Destination> {
    let mut state : Vec<Destination> = vec![];
    for item in input.iter() {
        let name = item.0.clone();
        let mut area = item.1.clone();
        let obj_path = item.2.clone();
        let area_vector = string_to_vec(&mut area);
        let polygon = creating_boundaries(area_vector);
        state.push(Destination::new(name, polygon, obj_path))
    }

    return state;
}

#[derive(Debug)]
pub struct Destination {
    name : String,
    area : Polygon<f32>,
    obj_path : String
}

impl Destination {
    pub fn new(name : String, area : Polygon<f32>, obj_path : String) -> Destination {
        Destination {
            name,
            area,
            obj_path
        }
    }
}