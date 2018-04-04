#![allow(dead_code)] // TODO(claire): Remove

use glium::index::PrimitiveType;
use glium::{Display, IndexBuffer, Program, VertexBuffer};

use adequate_math::*;


#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vertex {
    offset: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, offset);


pub struct Mesh {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u16>,
    pub shadow_indices: IndexBuffer<u16>,
}


pub fn create_shader(display: &Display, shader_source: &str) -> Program {
    use glium::program::ProgramCreationInput;

    let mut splits = shader_source.split("---");
    let vertex_source = splits.next().unwrap();
    let fragment_source = splits.next().unwrap();

    Program::new(
        display,
        ProgramCreationInput::SourceCode {
            vertex_shader: vertex_source,
            fragment_shader: fragment_source,
            outputs_srgb: true,
            geometry_shader: None,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            transform_feedback_varyings: None,
            uses_point_size: false,
        },
    ).unwrap()
}


pub fn create_cube_mesh(display: &Display, scale: Vec3<f32>) -> Mesh {
    let offsets = [
        vec3(-0.5, 0.5, -0.5) * scale,
        vec3(-0.5, -0.5, -0.5) * scale,
        vec3(0.5, 0.5, -0.5) * scale,
        vec3(0.5, -0.5, -0.5) * scale,
        vec3(-0.5, 0.5, 0.5) * scale,
        vec3(-0.5, -0.5, 0.5) * scale,
        vec3(0.5, 0.5, 0.5) * scale,
        vec3(0.5, -0.5, 0.5) * scale,
    ];

    let indices = [
        0, 1, 2, 1, 3, 2, 6, 7, 4, 7, 5, 4, 4, 0, 6, 6, 0, 2, 4, 5, 0, 5, 1, 0, 2,
        3, 6, 3, 7, 6, 1, 5, 3, 5, 7, 3,
    ];

    create_mesh(display, &offsets, &indices)
}


fn create_mesh(
    display: &Display,
    offsets: &[Vec3<f32>],
    indices: &[usize],
) -> Mesh {
    let face_normals = generate_face_normals(offsets, indices);

    let (vertices, flat_indices, shadow_indices) =
        generate_flat_mesh(offsets, indices, &face_normals);

    let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
    let index_buffer = IndexBuffer::new(
        display,
        PrimitiveType::TrianglesList,
        &flat_indices,
    ).unwrap();
    let shadow_index_buffer = IndexBuffer::new(
        display,
        PrimitiveType::TrianglesList,
        &shadow_indices,
    ).unwrap();


    Mesh {
        vertices: vertex_buffer,
        indices: index_buffer,
        shadow_indices: shadow_index_buffer,
    }
}


fn create_mesh_smooth(
    display: &Display,
    offsets: &[Vec3<f32>],
    indices: &[usize],
) -> Mesh {
    let face_normals = generate_face_normals(offsets, indices);
    let vertex_normals = generate_vertex_normals(offsets, indices, &face_normals);

    let mut vertices = Vec::with_capacity(offsets.len());
    for (offset, normal) in offsets.iter().zip(vertex_normals.iter()) {
        vertices.push(Vertex {
            offset: offset.0,
            normal: normal.0,
        });
    }

    let (mut flat_vertices, _flat_indices, shadow_indices) =
        generate_flat_mesh(offsets, indices, &face_normals);

    let smooth_indices: Vec<u16> = indices.iter().map(|&x| x as u16).collect();
    let smooth_count = offsets.len() as u16;

    vertices.append(&mut flat_vertices);
    let shadow_indices: Vec<u16> = shadow_indices
        .iter()
        .map(|&x| x + smooth_count)
        .collect();

    let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
    let index_buffer = IndexBuffer::new(
        display,
        PrimitiveType::TrianglesList,
        &smooth_indices,
    ).unwrap();
    let shadow_index_buffer = IndexBuffer::new(
        display,
        PrimitiveType::TrianglesList,
        &shadow_indices,
    ).unwrap();


    Mesh {
        vertices: vertex_buffer,
        indices: index_buffer,
        shadow_indices: shadow_index_buffer,
    }
}


fn generate_face_normals(
    offsets: &[Vec3<f32>],
    indices: &[usize],
) -> Vec<Vec3<f32>> {
    let mut face_normals = Vec::with_capacity(indices.len() / 3);
    for triangle in indices.chunks(3) {
        let i0 = triangle[0];
        let i1 = triangle[1];
        let i2 = triangle[2];
        let u: Vec3<f32> = offsets[i1] - offsets[i0];
        let v: Vec3<f32> = offsets[i2] - offsets[i0];
        let n = v.cross(u).norm();
        face_normals.push(n);
    }
    face_normals
}


fn generate_vertex_normals(
    offsets: &[Vec3<f32>],
    indices: &[usize],
    face_normals: &[Vec3<f32>],
) -> Vec<Vec3<f32>> {
    let mut vertex_normals = vec![vec3(0.0, 0.0, 0.0); offsets.len()];
    for (triangle_index, triangle) in indices.chunks(3).enumerate() {
        for &index in triangle {
            vertex_normals[index] += face_normals[triangle_index];
        }
    }

    for normal in &mut vertex_normals {
        *normal = normal.norm();
    }

    vertex_normals
}


fn generate_flat_mesh(
    offsets: &[Vec3<f32>],
    indices: &[usize],
    face_normals: &[Vec3<f32>],
) -> (Vec<Vertex>, Vec<u16>, Vec<u16>) {
    use std::collections::HashMap;

    let mut edge_map = HashMap::new();

    let mut vertices = Vec::with_capacity(indices.len());
    for (triangle_index, triangle) in indices.chunks(3).enumerate() {
        let new_index = vertices.len();

        for i in 0..3 {
            let j = (i + 1) % 3;
            let old_edge = (triangle[i], triangle[j]);
            let new_edge = (new_index + i, new_index + j);
            edge_map.insert(old_edge, new_edge);
        }

        for &index in triangle {
            vertices.push(Vertex {
                offset: offsets[index].0,
                normal: face_normals[triangle_index].0,
            });
        }
    }

    let flat_indices: Vec<u16> = (0..indices.len() as u16).collect();

    let mut shadow_indices = flat_indices.clone();
    for (old_edge, edge_a) in &edge_map {
        let &(a, b) = old_edge;
        let edge_b = edge_map
            .get(&(b, a))
            .expect("Mesh isn't closed - found an open edge");
        shadow_indices.push(edge_a.1 as u16);
        shadow_indices.push(edge_a.0 as u16);
        shadow_indices.push(edge_b.0 as u16);
    }

    (vertices, flat_indices, shadow_indices)
}
