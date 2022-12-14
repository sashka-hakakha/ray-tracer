#![allow(dead_code)]

// extern crate alloc;
// use alloc::{vec::Vec, string::String};
// extern crate core;
// use core::option::Option::{self,*};
// use core::iter::Iterator;
// use core::panic::Panic;
use std::io::{self, BufRead};
use std::fs::File;
mod matrix;

pub struct OBJ {
    geo_verts: Vec<matrix::Vert>,
    vert_norms: Vec<matrix::Vert>,
    texture_coords: Vec<matrix::Vert>,
    faces: Vec<matrix::Face3>,
}

pub fn many_points_to_tri(
    geo_verts: &Vec<matrix::Vert>,
    geo_points: &Vec<usize>,
    vert_norms: &Vec<matrix::Vert>,
    norm_points: &Vec<usize>,
    texture_coords: &Vec<matrix::Vert>,
    texture_points: &Vec<usize>,
) -> Vec<matrix::Face3> {
    let mut faces = Vec::new();
    for i in 0..geo_points.len() - 2 {
        let new_vert_norms = None;
        let new_texture_coords = None;
        let new_geo_verts = matrix::M3 {
            col1: geo_verts[geo_points[0]],
            col2: geo_verts[geo_points[i]],
            col3: geo_verts[geo_points[i + 1]],
        };
        if norm_points.len() - 2 >= i {
            let new_vert_norms = Some(matrix::M3 {
                col1: vert_norms[norm_points[0]],
                col2: vert_norms[norm_points[i]],
                col3: vert_norms[norm_points[i + 1]],
            });
        }
        if texture_points.len() - 2 >= i {
            let new_texture_coords = Some(matrix::M3 {
                col1: texture_coords[texture_points[0]],
                col2: texture_coords[texture_points[i]],
                col3: texture_coords[texture_points[i + 1]],
            });
        }
        faces.push(matrix::Face3 {
            verts: new_geo_verts,
            normals: new_vert_norms,
            texture_coords: new_texture_coords,
        });
    }
    faces
}
pub fn three_geo_verts_to_face(points: Vec<&str>, verts: &Vec<matrix::Vert>) -> matrix::Face3 {
    let col1_index = points[0].parse::<usize>().unwrap();
    let col2_index = points[1].parse::<usize>().unwrap();
    let col3_index = points[2].parse::<usize>().unwrap();

    matrix::Face3 {
        verts: {
            matrix::M3 {
                col1: verts[col1_index],
                col2: verts[col2_index],
                col3: verts[col3_index],
            }
        },
        normals: None,
        texture_coords: None,
    }
}

pub fn process_lines(lines: io::Lines<io::BufReader<File>>) -> OBJ{
    let geo_verts: Vec<matrix::Vert> = vec![];
    let vert_norms: Vec<matrix::Vert> = vec![];
    let texture_coords: Vec<matrix::Vert> = vec![];
    let faces: Vec<matrix::Face3> = vec![];
    for line in lines{
        let line_str = line.unwrap();
        let mut split_line = line_str.split(" ");
        let split_vec = split_line.collect::<Vec<&str>>();
        let len = split_vec.length();
        if (splic_vec[0] == "v"){
            if (len < 4){
                panic!("too few args for vert!");
            }
            else if (len > 5){
                panic!("too many args for vert!")
            }
            else if(len == 4){
                geo_verts.push(matrix::Vert{
                    coords:[
                        split_vec[1].parse::<f32>.unwrap(),
                        split_vec[2].parse::<f32>.unwrap(),
                        split_vec[3].parse::<f32>.unwrap(),
                        -1.0
                    ]
                });
            }
            else{
                geo_verts.push(matrix::Vert{
                    coords:[
                        split_vec[1].parse::<f32>.unwrap(),
                        split_vec[2].parse::<f32>.unwrap(),
                        split_vec[3].parse::<f32>.unwrap(),
                        split_vec[4].parse::<f32>.unwrap(),
                    ]
                });
            }
        }
        if(split_vec[0] == "vt"){
            if(len < 2){
                panic!("too few arguments for texture coord")
            }
            else if(len > 4){
                panic!("too many arguments for texture coord")
            }
            else if(len == 2){
                texture_coords.push(matrix::Vert{
                    coords:[
                        split_vec[1].parse::<f32>.unwrap(),
                        -1.0,
                        -1.0,
                        -1.0,
                    ]
                })
            }
            else if(len == 3){
                texture_coords.push(matrix::Vert{
                    coords:[
                        split_vec[1].parse::<f32>.unwrap(),
                        split_vec[2].parse::<f32>.unwrap(),
                        -1.0,
                        -1.0,
                    ]
                })
            }
            else if(len == 3){
                texture_coords.push(matrix::Vert{
                    coords:[
                        split_vec[1].parse::<f32>.unwrap(),
                        split_vec[2].parse::<f32>.unwrap(),
                        split_vec[3].parse::<f32>.unwrap(),
                        -1.0,
                    ]
                })
            }
        }
        
    }
    OBJ{
        geo_verts: geo_verts,
        vert_norms: vert_norms,
        texture_coords: texture_coords,
        faces: faces,
    }
}

fn main() {
    println!("geometry good!");
}
