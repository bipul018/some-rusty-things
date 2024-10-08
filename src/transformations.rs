use glam::{Vec2, Mat3, Vec3, Quat, Mat4};

#[derive(Debug)]
pub struct Camera3D{
    pub transform: Transform3D,
    pub fov_y_radians: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
}

#[allow(dead_code)]
impl Camera3D{
    pub fn init(fov_y_radians: f32, aspect_ratio: f32, z_range: [f32;2]) -> Self{
	Self{transform:Transform3D::init(),
	     fov_y_radians, aspect_ratio,
	     z_near: z_range[0], z_far: z_range[1]}
    }
    pub fn mat(&self) -> Mat4{
	Mat4::perspective_rh(self.fov_y_radians, self.aspect_ratio, self.z_near, self.z_far)
	    * self.transform.mat().inverse()
    }
}

// A system for rendering
// Takes in model -> applies model trans -> camera + proj trans -> makes Vec2 arrays
//       supplies it into 2D triangle drawing
// Need to do backface culling in 2D triangle drawing part ?? (sad)
#[derive(Debug)]
pub struct Transform3D{
    pub pos: Vec3,
    pub rotq: Quat, //Need to rotate this by additional quats
    pub scale: Vec3,
}

#[allow(dead_code)]
impl Transform3D{
    pub fn init() -> Self{
	Self{pos:Vec3::ZERO,rotq:Quat::IDENTITY, scale:Vec3::ONE}
    }
    pub fn reset(self) -> Self{
	Self::init()
    }
    pub fn from_mat4(affine_mat: &Mat4) -> Self{
	let (s, r, t) = affine_mat.to_scale_rotation_translation();
	Self{ pos: t, rotq: r, scale: s}
    }
    pub fn translate(self, delta: Vec3)->Self{
	Self{pos: self.pos + delta, ..self}
    }
    pub fn scalef(self, factor: f32)->Self{
	Self{scale:self.scale*factor, ..self}
    }
    //Angles are in radians
    pub fn rotatex(self, angle: f32)->Self{
	let rotq = Quat::from_rotation_x(angle);
	Self{rotq:self.rotq.mul_quat(rotq), ..self}
    }
    pub fn rotatey(self, angle: f32)->Self{
	let rotq = Quat::from_rotation_y(angle);
	Self{rotq:self.rotq.mul_quat(rotq), ..self}
    }
    pub fn rotatez(self, angle: f32)->Self{
	let rotq = Quat::from_rotation_z(angle);
	Self{rotq:self.rotq.mul_quat(rotq), ..self}
    }
    //Does translate * rotate * scale operation 
    pub fn mat(&self)->Mat4{
	Mat4::from_translation(self.pos) *
	    Mat4::from_quat(self.rotq) *
	    Mat4::from_scale(self.scale)
    }
}


pub struct Camera2D{
    pub pos: Vec2,   //Position in the world
    pub rot: f32,    //Angle with x axis of world in radians
    pub view: f32,  //Range along x (both front and back) upto which camera can see world
    // Might have to add a preferred direction where we preserve view
    // Maybe make it a vector, need to clip the vector line on window rect
}

#[allow(dead_code)]
impl Camera2D{
    pub fn init(pos:Vec2, rot:f32, view:f32) -> Camera2D{
	Camera2D{ pos, rot, view }
    }
    pub fn matrix<T:sdl2::render::RenderTarget>(&self, canvas: &sdl2::render::Canvas<T>) -> Mat3 {
	// To make a camera/projection matrix, we need to
	// translate to position
	// rotate by reverse
	// scale so view fits in screen

	let trmat = Mat3::from_translation(-self.pos);
	let romat = Mat3::from_angle(-self.rot);
	let vprt = canvas.viewport();
	// TODO:: need to find out if need to translate too on viewport not starting at 0,0
	// need to map view to vprt.width
	let scmat = Mat3::from_scale(Vec2::new(vprt.width() as f32/(self.view),
					       vprt.width() as f32/(self.view)));
	let tr2mat = Mat3::from_translation(Vec2::new(vprt.width() as f32 * 0.5,
						      vprt.height() as f32 * 0.5));
	return tr2mat * scmat * romat * trmat;
    }
    pub fn lookpt<T:sdl2::render::RenderTarget>(&self, canvas: &sdl2::render::Canvas<T>, point:Vec2) -> Vec2{
	self.matrix(canvas).transform_point2(point)
    }
    pub fn lookvec<T:sdl2::render::RenderTarget>(&self, canvas: &sdl2::render::Canvas<T>, vector:Vec2) -> Vec2{
	self.matrix(canvas).transform_vector2(vector)
    }
}
