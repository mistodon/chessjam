use glium::{Display, IndexBuffer, Program, VertexBuffer};

use adequate_math::*;


#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vertex {
    offset: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, offset, normal);


pub struct Mesh {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u16>,
    pub shadow_vertices: VertexBuffer<Vertex>,
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
            outputs_srgb: false,
            geometry_shader: None,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            transform_feedback_varyings: None,
            uses_point_size: false,
        },
    ).unwrap()
}


pub fn create_obj_mesh(display: &Display, source: &str) -> Mesh {
    use glium::index::PrimitiveType::TrianglesList;
    use wavefront_obj::obj::{self, Object, Primitive::Triangle};

    let obj_set = obj::parse(source.to_owned()).unwrap();
    let (mesh, shadow) = if obj_set.objects[0].name.starts_with("Shadow") {
        (&obj_set.objects[1], &obj_set.objects[0])
    }
    else {
        (&obj_set.objects[0], &obj_set.objects[1])
    };

    fn parse_shape(object: &Object) -> (Vec<Vertex>, Vec<u16>) {
        let offsets = object
            .vertices
            .iter()
            .map(|v| vec3(v.x, v.y, v.z).as_f32())
            .collect::<Vec<Vec3<f32>>>();

        let normals = object
            .normals
            .iter()
            .map(|v| vec3(v.x, v.y, v.z).as_f32())
            .collect::<Vec<Vec3<f32>>>();

        let mut vertices = Vec::new();

        for geo in &object.geometry {
            for shape in &geo.shapes {
                match shape.primitive {
                    // TODO(***realname***): Why is it inside out unless I use this order?
                    Triangle(i0, i1, i2) => {
                        for v in &[i0, i2, i1] {
                            let offset = offsets[v.0].0;
                            let normal = normals[v.2.unwrap()].0;
                            vertices.push(Vertex { offset, normal });
                        }
                    }
                    _ => unreachable!("Expected only triangles in obj files"),
                }
            }
        }

        // TODO(***realname***): Re-use identical vertices
        let indices = (0..vertices.len() as u16).collect::<Vec<_>>();
        (vertices, indices)
    }

    let (mesh_vertices, mesh_indices) = parse_shape(mesh);

    // TODO(***realname***): Make shadow mesh properly - with degenerate edge quads.
    let (shadow_vertices, shadow_indices) = {
        let offsets = shadow
            .vertices
            .iter()
            .map(|v| vec3(v.x, v.y, v.z).as_f32())
            .collect::<Vec<Vec3<f32>>>();

        let mut indices = Vec::new();
        for geo in &shadow.geometry {
            for shape in &geo.shapes {
                match shape.primitive {
                    // TODO(***realname***): Why is it inside out unless I use this order?
                    Triangle(i0, i1, i2) => {
                        indices.push(i0.0);
                        indices.push(i2.0);
                        indices.push(i1.0);
                    }
                    _ => unreachable!("Expected only triangles in obj files"),
                }
            }
        }

        let face_normals = generate_face_normals(&offsets, &indices);
        let (vertices, _flat_indices, shadow_indices) =
            generate_flat_mesh(&offsets, &indices, &face_normals);

        (vertices, shadow_indices)
    };

    Mesh {
        vertices: VertexBuffer::new(display, &mesh_vertices).unwrap(),
        indices: IndexBuffer::new(display, TrianglesList, &mesh_indices).unwrap(),
        shadow_vertices: VertexBuffer::new(display, &shadow_vertices).unwrap(),
        shadow_indices: IndexBuffer::new(display, TrianglesList, &shadow_indices)
            .unwrap(),
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

        // TODO(***realname***): We require a closed mesh here, which precludes having
        // sharp edges by splitting the mesh. A simple solution would be to fuse
        // the mesh pieces for shadow generation, but that may be too much online
        // work.
        let edge_b = edge_map
            .get(&(b, a))
            .expect("Mesh isn't closed - found an open edge");
        shadow_indices.push(edge_a.1 as u16);
        shadow_indices.push(edge_a.0 as u16);
        shadow_indices.push(edge_b.0 as u16);
    }

    (vertices, flat_indices, shadow_indices)
}
