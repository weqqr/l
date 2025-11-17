use asset::{Mesh, Vertex};
use glam::{IVec3, Vec2, Vec3, ivec3, vec3};
use world::Block;

pub fn make_mesh(block: &Block) -> Mesh {
    let mut mesh = Mesh::new();

    for z in 0..16 {
        for y in 0..16 {
            for x in 0..16 {
                let pos = ivec3(x, y, z);

                let name = block.get_name_by_id(block.get_node(pos).id).unwrap();

                if name == "air" {
                    continue;
                }

                let contains_block = |pos: IVec3| {
                    if pos.x < 0
                        || pos.y < 0
                        || pos.z < 0
                        || pos.x >= 16
                        || pos.y >= 16
                        || pos.z >= 16
                    {
                        return false;
                    }

                    block.get_name_by_id(block.get_node(pos).id).unwrap() != "air"
                };

                let sides = [
                    contains_block(pos + IVec3::X),
                    contains_block(pos - IVec3::X),
                    contains_block(pos + IVec3::Y),
                    contains_block(pos - IVec3::Y),
                    contains_block(pos + IVec3::Z),
                    contains_block(pos - IVec3::Z),
                ];

                for (i, has_neighbor_cube) in sides.iter().enumerate() {
                    if !*has_neighbor_cube {
                        for vertex in &CUBE_FACES[i] {
                            let mut vertex = vertex.clone();
                            vertex.position += vec3(pos.x as f32, pos.y as f32, pos.z as f32);
                            mesh.add_vertex(vertex.clone());
                        }
                    }
                }
            }
        }
    }

    mesh
}

const CUBE_FACES: [[Vertex; 6]; 6] = [
    // X+
    [
        Vertex {
            position: Vec3::new(0.5, -0.5, -0.5),
            normal: Vec3::new(1.0, 0.0, 0.0),
            texcoord: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.5, 0.5, -0.5),
            normal: Vec3::new(1.0, 0.0, 0.0),
            texcoord: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.5, 0.5, 0.5),
            normal: Vec3::new(1.0, 0.0, 0.0),
            texcoord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.5, -0.5, -0.5),
            normal: Vec3::new(1.0, 0.0, 0.0),
            texcoord: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.5, 0.5, 0.5),
            normal: Vec3::new(1.0, 0.0, 0.0),
            texcoord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.5, -0.5, 0.5),
            normal: Vec3::new(1.0, 0.0, 0.0),
            texcoord: Vec2::new(1.0, 0.0),
        },
    ],
    // X-
    [
        Vertex {
            position: Vec3::new(-0.5, -0.5, 0.5),
            normal: Vec3::new(-1.0, 0.0, 0.0),
            texcoord: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(-0.5, 0.5, 0.5),
            normal: Vec3::new(-1.0, 0.0, 0.0),
            texcoord: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.5, 0.5, -0.5),
            normal: Vec3::new(-1.0, 0.0, 0.0),
            texcoord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.5, -0.5, 0.5),
            normal: Vec3::new(-1.0, 0.0, 0.0),
            texcoord: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(-0.5, 0.5, -0.5),
            normal: Vec3::new(-1.0, 0.0, 0.0),
            texcoord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.5, -0.5, -0.5),
            normal: Vec3::new(-1.0, 0.0, 0.0),
            texcoord: Vec2::new(1.0, 0.0),
        },
    ],
    // Y+
    [
        Vertex {
            position: Vec3::new(-0.5, 0.5, -0.5),
            normal: Vec3::new(0.0, 1.0, 0.0),
            texcoord: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.5, 0.5, -0.5),
            normal: Vec3::new(0.0, 1.0, 0.0),
            texcoord: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.5, 0.5, 0.5),
            normal: Vec3::new(0.0, 1.0, 0.0),
            texcoord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.5, 0.5, -0.5),
            normal: Vec3::new(0.0, 1.0, 0.0),
            texcoord: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.5, 0.5, 0.5),
            normal: Vec3::new(0.0, 1.0, 0.0),
            texcoord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.5, 0.5, 0.5),
            normal: Vec3::new(0.0, 1.0, 0.0),
            texcoord: Vec2::new(1.0, 0.0),
        },
    ],
    // Y-
    [
        Vertex {
            position: Vec3::new(-0.5, -0.5, 0.5),
            normal: Vec3::new(0.0, -1.0, 0.0),
            texcoord: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.5, -0.5, 0.5),
            normal: Vec3::new(0.0, -1.0, 0.0),
            texcoord: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.5, -0.5, -0.5),
            normal: Vec3::new(0.0, -1.0, 0.0),
            texcoord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.5, -0.5, 0.5),
            normal: Vec3::new(0.0, -1.0, 0.0),
            texcoord: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.5, -0.5, -0.5),
            normal: Vec3::new(0.0, -1.0, 0.0),
            texcoord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.5, -0.5, -0.5),
            normal: Vec3::new(0.0, -1.0, 0.0),
            texcoord: Vec2::new(1.0, 0.0),
        },
    ],
    // Z+
    [
        Vertex {
            position: Vec3::new(-0.5, -0.5, 0.5),
            normal: Vec3::new(0.0, 0.0, 1.0),
            texcoord: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.5, -0.5, 0.5),
            normal: Vec3::new(0.0, 0.0, 1.0),
            texcoord: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.5, 0.5, 0.5),
            normal: Vec3::new(0.0, 0.0, 1.0),
            texcoord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.5, -0.5, 0.5),
            normal: Vec3::new(0.0, 0.0, 1.0),
            texcoord: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.5, 0.5, 0.5),
            normal: Vec3::new(0.0, 0.0, 1.0),
            texcoord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.5, 0.5, 0.5),
            normal: Vec3::new(0.0, 0.0, 1.0),
            texcoord: Vec2::new(1.0, 0.0),
        },
    ],
    // Z-
    [
        Vertex {
            position: Vec3::new(0.5, -0.5, -0.5),
            normal: Vec3::new(0.0, 0.0, -1.0),
            texcoord: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(-0.5, -0.5, -0.5),
            normal: Vec3::new(0.0, 0.0, -1.0),
            texcoord: Vec2::new(0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.5, 0.5, -0.5),
            normal: Vec3::new(0.0, 0.0, -1.0),
            texcoord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.5, -0.5, -0.5),
            normal: Vec3::new(0.0, 0.0, -1.0),
            texcoord: Vec2::new(0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(-0.5, 0.5, -0.5),
            normal: Vec3::new(0.0, 0.0, -1.0),
            texcoord: Vec2::new(1.0, 1.0),
        },
        Vertex {
            position: Vec3::new(0.5, 0.5, -0.5),
            normal: Vec3::new(0.0, 0.0, -1.0),
            texcoord: Vec2::new(1.0, 0.0),
        },
    ],
];
