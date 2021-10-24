use crate::renderer::*;

#[derive(Clone)]
pub struct Sphere<M: ForwardMaterial> {
    model: Model,
    center: Vec3,
    radius: f32,
    /// The material applied to the sphere
    pub material: M,
}

impl<M: ForwardMaterial> Sphere<M> {
    pub fn new(context: &Context, center: Vec3, radius: f32, material: M) -> ThreeDResult<Self> {
        let mesh = CPUMesh::sphere((radius * 20.0).max(4.0) as u32);
        let mut model = Model::new(context, &mesh)?;
        model.set_transformation(Mat4::from_translation(center) * Mat4::from_scale(radius));
        Ok(Self {
            material,
            model,
            center,
            radius,
        })
    }

    pub fn set_center(&mut self, center: Vec3) {
        self.center = center;
        self.model.set_transformation(
            Mat4::from_translation(self.center) * Mat4::from_scale(self.radius),
        );
    }

    pub fn center(&self) -> &Vec3 {
        &self.center
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
        self.model.set_transformation(
            Mat4::from_translation(self.center) * Mat4::from_scale(self.radius),
        );
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl<M: ForwardMaterial> Shadable for Sphere<M> {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        self.model.render_forward(material, camera, lights)
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.model.render_deferred(material, camera, viewport)
    }
}

impl<M: ForwardMaterial> Geometry for Sphere<M> {
    fn aabb(&self) -> &AxisAlignedBoundingBox {
        self.model.aabb()
    }

    fn transformation(&self) -> &Mat4 {
        self.model.transformation()
    }
}

impl<M: ForwardMaterial> GeometryMut for Sphere<M> {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.model.set_transformation(transformation);
    }
}

impl<M: ForwardMaterial> Object for Sphere<M> {
    fn render(&self, camera: &Camera, lights: &Lights) -> ThreeDResult<()> {
        self.model.render_forward(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        false
    }
}
