use crate::*;

pub struct ThreeD {

}

impl ThreeD {
    pub fn parse(bytes: &[u8]) -> Result<Vec<CPUMesh>, bincode::Error>
    {
        let mut decoded = bincode::deserialize::<ThreeDMesh>(bytes)
            .or_else(|_| Self::parse_version1(bytes))?;

        if decoded.submeshes.len() == 0 {
            decoded = Self::parse_version1(bytes)?;
            if decoded.submeshes.len() == 0 {
                Err(bincode::Error::new(bincode::ErrorKind::Custom("No mesh data in file!".to_string())))?;
            }
        }

        if decoded.magic_number != 61 {
            Err(bincode::Error::new(bincode::ErrorKind::Custom("Corrupt file!".to_string())))?;
        }

        let mut cpu_meshes = Vec::new();
        for mesh in decoded.submeshes {
            cpu_meshes.push(CPUMesh {
                indices: if mesh.indices.len() > 0 {Some(mesh.indices)} else {None},
                positions: mesh.positions,
                normals: if mesh.normals.len() > 0 {Some(mesh.normals)} else {None},
                uvs: if mesh.uvs.len() > 0 {Some(mesh.uvs)} else {None},
                ..Default::default()
            });
        }
        Ok(cpu_meshes)
    }

    fn parse_version1(bytes: &[u8]) -> Result<ThreeDMesh, bincode::Error> {
        bincode::deserialize::<ThreeDMeshV1>(bytes).map(|m| ThreeDMesh {
                magic_number: m.magic_number,
                version: 2,
                submeshes: vec![ThreeDMeshSubMesh {
                    indices: m.indices,
                    positions: m.positions,
                    normals: m.normals,
                    uvs: vec![]
                }]
            })
    }

    pub fn serialize(meshes: &Vec<CPUMesh>) -> Result<Vec<u8>, bincode::Error>
    {
        Ok(bincode::serialize::<ThreeDMesh>(&ThreeDMesh {
            magic_number: 61,
            version: 2,
            submeshes: meshes.iter().map(|mesh|
            ThreeDMeshSubMesh {
                indices: mesh.indices.as_ref().unwrap_or(&Vec::new()).to_owned(),
                positions: mesh.positions.to_owned(),
                normals: mesh.normals.as_ref().unwrap_or(&Vec::new()).to_owned(),
                uvs: mesh.uvs.as_ref().unwrap_or(&Vec::new()).to_owned()
            }).collect()
        })?)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ThreeDMesh {
    pub magic_number: u8,
    pub version: u8,
    pub submeshes: Vec<ThreeDMeshSubMesh>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ThreeDMeshSubMesh {
    pub indices: Vec<u32>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ThreeDMeshV1 {
    pub magic_number: u8,
    pub version: u8,
    pub indices: Vec<u32>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>
}