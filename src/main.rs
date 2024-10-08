extern crate sdl2;
extern crate glam;
use glam::{Vec2, Mat3, Vec3, Quat, Mat4};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

macro_rules! generate_sphere3d {
    ($segments:expr, $radius:expr, $center:expr) => {{
        //let mut pts: Vec<Vec3> = Vec::with_capacity(($segments + 1) * ($segments + 1));
        //let mut inxs: Vec<u16> = Vec::with_capacity(6 * $segments * $segments);
	let mut pts = [Vec3::ZERO;($segments + 1) * ($segments + 1)];
	let mut inxs = [0 as u16; 6 * $segments * $segments];
        // Generate points on the sphere surface
	let mut pt_inx:usize = 0;
	let mut inx_inx:usize = 0;
        for lat in 0..=$segments {
            let theta = lat as f32 * std::f32::consts::PI / $segments as f32; // Latitude
            for lon in 0..=$segments {
                let phi = lon as f32 * 2.0 * std::f32::consts::PI / $segments as f32; // Longitude
                let x = $radius * f32::sin(theta) * f32::cos(phi) + $center.x;
                let y = $radius * f32::sin(theta) * f32::sin(phi) + $center.y;
                let z = $radius * f32::cos(theta) + $center.z;
		pts[pt_inx]=Vec3::new(x, y, z);
		pt_inx+=1;
                //pts.push(Vec3::new(x, y, z));
            }
        }

        // Generate indices for the triangles
        for lat in 0..$segments {
            for lon in 0..$segments {
                let first = (lat * ($segments + 1)) + lon;
                let second = first + $segments + 1;

                // Two triangles per quad
		inxs[inx_inx] = first as u16;
		inx_inx+=1;
		//inxs.push(first as u16);
		inxs[inx_inx] = second as u16;
		inx_inx+=1;
		//inxs.push(second as u16);
		inxs[inx_inx] = (first + 1) as u16;
		inx_inx+=1;
		//inxs.push((first + 1) as u16);
                
		inxs[inx_inx] = second as u16;
		inx_inx+=1;
		//inxs.push(second as u16);
		inxs[inx_inx] = (second + 1) as u16;
		inx_inx+=1;
		//inxs.push((second + 1) as u16);
		inxs[inx_inx] = (first + 1) as u16;
		inx_inx+=1;
		//inxs.push((first + 1) as u16);
            }
        }

        (pts, inxs)
    }};
}


macro_rules! generate_circle2d{
    ($segments:expr, $radius:expr, $center:expr) =>{{
	let mut pts: [Vec2; $segments + 1] = [Vec2::ZERO; $segments + 1];
        let mut inxs: [u16; 3 * $segments] = [0; 3 * $segments];

        pts[0] = $center;
	use std::f32;
        let step_angle = 2.0 * std::f32::consts::PI / ($segments as f32);
        
        for i in 1..=$segments {
            let angle = step_angle * i as f32;
            let x = f32::cos(angle) * $radius + $center.x;
            let y = f32::sin(angle) * $radius + $center.y;
            pts[i as usize] = Vec2::new(x, y);
        }

        // Populate the indices array
        for i in 0..$segments {
            inxs[(i * 3) as usize] = 0; // Center point index
            inxs[(i * 3 + 1) as usize] = i + 1; // Current point index
            inxs[(i * 3 + 2) as usize] = if i == $segments - 1 { 1 } else { i + 2 }; // Next point index
        }

        (pts, inxs)

    }};
}

macro_rules! generate_circle3d{
    ($segments:expr, $radius:expr, $center:expr) =>{{
	let center_3d = $center;
	let center_2d = center_3d.truncate();
	let (pts2d, inxs) = generate_circle2d!($segments, $radius, center_2d);
	let pts3d_array = pts2d.map(|pt2d|{Vec3::new(pt2d.x, pt2d.y, center_3d.z)});
        (pts3d_array, inxs)
    }};
}


#[allow(unused_mut)]
#[allow(unreachable_code)]
#[allow(unused_variables)]
pub fn main() {
    // Unlike the C version, you don't give in the submodules to initialize here
    // It is intialized when you access it later (I think)
    let sdl_context = match sdl2::init(){
	Ok(sdl_cxt) => sdl_cxt,
	Err(sdl_err) => {
	    println!("Error occured in initializing sdl : {}", sdl_err);
	    return;
	}
    };
    // Video subsystem is good only for clipboard action and creating window
    let video_subsystem = match sdl_context.video(){
	Ok(vdo_subsys) => vdo_subsys,
	Err(vdo_err) => {
	    println!("Error occured in initializing sdl video subsystem : {}", vdo_err);
	    return;
	}
    };

    // Since canvas consumes window anyway, wrapping the window making...

    let mut cnv = 'make_canvas:{

	// You have to use window builder, otherwise you just have to directly interact with the C API
	//     and pass in raw SDL_window along with this video subsystem to create a window struct
	//     just for kicks, trying out creating window not as member of video subsystem
	let window = match sdl2::video::WindowBuilder
	    ::new(&video_subsystem, "My Rust SDL2 Demo", 800, 450)
	    .position_centered()
	    .resizable()
	    .build(){
		Ok(res) => res,
		Err(winderr) => {
		    println!("Error occured in initializing window : {}",
			     winderr.to_string());
		    return;
		}
	    };

	// They fking had to wrap renderer into canvas ??
	match window.into_canvas().present_vsync().build(){
	    Ok(res) => { break 'make_canvas res; }
	    Err(err) => {
		println!("Error occured in initializing canvas : {}",
			 err.to_string());
		return;
	    }
	};
    };
    /*
    So there are two ways of rendering into window,
    first is through renderer/canvas, where you can use graphics API or use draw rect or use gfx
    second is through window surface, where you get access to the raw pixels that you
    first set, then have to call to update
     */


    let mut event_pump = match sdl_context.event_pump(){
	Ok(res) => res,
	_ => {
	    println!("Couldn't initialize the event pump\n");
	    return;
	}
    };

    let bg_col = Color::RGB(127,127,127);


    //Build a sphere 3D model
    //Project it
    let ball2_pos = Vec2::new(0.0, 3.0);
    let mut ball_pos:Vec2 = Vec2::new(3.0, 0.0);
    let mut ball_vel:Vec2 = Vec2::new(0.0, 0.0);
    let ball_acc:Vec2 = Vec2::new(0.0, 0.0);

    let mut cam2d = Camera2D::init(Vec2::new(0.0,0.0), 0.0, 2.0);
    let mut cam3d = Camera3D::init(std::f32::consts::PI/2.0, 9.0/9.0, [0.0, 20.0]);
    // cam3d.transform = cam3d.transform
    // 	.translate(Vec3::new(0.0, 10.0, 0.0))
    // 	.scalef(10.0)
    // 	.rotatex(std::f32::consts::PI/2.0);
    cam3d.transform = Transform3D::from_mat4(
	&Mat4::look_at_rh(Vec3::new(0.0, 20.0, 0.0),
			 Vec3::new(0.0, 0.0, 0.0),
			 Vec3::new(0.0, 0.0, 1.0)).inverse());




    let mut model_trns = Transform3D::init();

    let mut control_mode:u16 = 0;
    'main_loop: loop {
	//break;
        for event in event_pump.poll_iter() {
	    use Event::KeyDown;
	    // For general events
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape) | Some(Keycode::Backspace), .. }
		=> break 'main_loop,

		//Remaining keydown matched together
		KeyDown{keycode: Some(key), ..} =>{

		    match control_mode{
			// For 2D events
			0 => match key{
			    Keycode::Q => cam2d.rot -= 0.05,
			    Keycode::E => cam2d.rot += 0.05,
			    Keycode::W => cam2d.pos.y += 0.1,
			    Keycode::S => cam2d.pos.y -= 0.1,
			    Keycode::A => cam2d.pos.x += 0.1,
			    Keycode::D => cam2d.pos.x -= 0.1,
			    Keycode::Z => cam2d.view /= 1.01,
			    Keycode::C => cam2d.view *= 1.01,

			    Keycode::M => control_mode=1,
			    _=>{}
			},
			// For 3D events
			1 => {
			    /*
			    AD -> x move, WS -> y move, QE -> z move
			    right/left ->x rotate, up/down->z rotate, pgup/pgdown-> y rotate
			    ZC -> zoom in/out
			     */
			    match key{
				Keycode::Q => cam3d.transform.pos.z -= 0.1,
				Keycode::E => cam3d.transform.pos.z += 0.1,
				Keycode::W => cam3d.transform.pos.y += 0.1,
				Keycode::S => cam3d.transform.pos.y -= 0.1,
				Keycode::A => cam3d.transform.pos.x += 0.1,
				Keycode::D => cam3d.transform.pos.x -= 0.1,
				Keycode::Z => cam3d.transform.scale /= 1.01,
				Keycode::C => cam3d.transform.scale *= 1.01,
				Keycode::Right => cam3d.transform=cam3d.transform.rotatex(0.05),
				Keycode::Left => cam3d.transform=cam3d.transform.rotatex(-0.05),
				Keycode::Up => cam3d.transform=cam3d.transform.rotatez(0.05),
				Keycode::Down => cam3d.transform=cam3d.transform.rotatez(-0.05),
				Keycode::PageUp => cam3d.transform=cam3d.transform.rotatey(0.05),
				Keycode::PageDown => cam3d.transform=cam3d.transform.rotatey(-0.05),
				Keycode::M => control_mode=2,
				_=>{}
			    }
			},
			// For 3D model event
			2 => {
			    /*
			    AD -> x move, WS -> y move, QE -> z move
			    right/left ->x rotate, up/down->z rotate, pgup/pgdown-> y rotate
			    ZC -> zoom in/out
			    model_trns
			     */
			    match key{
				Keycode::Q => model_trns.pos.z -= 0.1,
				Keycode::E => model_trns.pos.z += 0.1,
				Keycode::W => model_trns.pos.y += 0.1,
				Keycode::S => model_trns.pos.y -= 0.1,
				Keycode::A => model_trns.pos.x += 0.1,
				Keycode::D => model_trns.pos.x -= 0.1,
				Keycode::Z => model_trns.scale /= 1.01,
				Keycode::C => model_trns.scale *= 1.01,
				Keycode::Right => model_trns=model_trns.rotatex(0.05),
				Keycode::Left => model_trns=model_trns.rotatex(-0.05),
				Keycode::Up => model_trns=model_trns.rotatez(0.05),
				Keycode::Down => model_trns=model_trns.rotatez(-0.05),
				Keycode::PageUp => model_trns=model_trns.rotatey(0.05),
				Keycode::PageDown => model_trns=model_trns.rotatey(-0.05),

				Keycode::M => control_mode=0,
				_=>{}
			    }
			},
			_=>{}
		    }

		    
		},
                _ => {}
            }

	    
        }

	ball_pos = ball_pos + ball_vel;
	ball_vel = ball_vel + ball_acc;

	// let win_size = Vec2::new(cnv.window().drawable_size().0 as f32,
	// 			 cnv.window().drawable_size().1 as f32);
	let win_size = Vec2::new(5.0, 5.0);
	
	if ball_pos.y >= win_size.y{
	    ball_pos.y = win_size.y - (ball_pos.y - win_size.y);
	    ball_vel.y = -ball_vel.y;
	}
	
	cnv.set_draw_color(bg_col);
        cnv.clear();

	_=fill_circle(&cnv, cam2d.lookpt(&cnv, ball_pos), 30.0, Color::RGB(0,0,255));
	_=fill_circle(&cnv, cam2d.lookpt(&cnv, ball2_pos), 30.0, Color::RGB(255,0,0));

	let pts: Vec<Vec2> = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]]
	    .iter().map(|&v| Vec2::from_array(v)).collect();
	let inxs: Vec<u16> = [0, 1, 2, 2, 3, 0].iter().map(|x| *x as u16).collect();

	_=draw_triangles(&cnv, &cam2d, &pts, &inxs, Color::RGB(0,255,0), false);
	_=draw_triangles(&cnv, &cam2d, &pts, &inxs, Color::RGB(0,0,0), true);
	    
			      
	let (cir_pts, cir_inx) = generate_circle2d!(10, 1.2, Vec2::new(-3.0,0.0));
	
	_=draw_triangles(&cnv, &cam2d, &cir_pts, &cir_inx,
			 Color::RGB(255,255,255), false);
	_=draw_triangles(&cnv, &cam2d, &cir_pts, &cir_inx,
			 Color::RGB(0,0,0), true);

	//let (cir3d_pts, cir3d_inx) = generate_circle3d!(10, 1.2, Vec3::new(4.0, 4.0, 1.0));
	let (cir3d_pts, cir3d_inx) = generate_sphere3d!(10, 1.0, Vec3::new(0.0,0.0,0.0));
	let cam_proj = cam3d.mat();
	let cir3d_proj = cir3d_pts.map(|pt3d|{
	    cam_proj.project_point3(model_trns.mat().transform_point3(pt3d)).truncate()
	});
	_=draw_triangles(&cnv, &cam2d, &cir3d_proj, &cir3d_inx,
			 Color::RGB(255,255,0), false);
	_=draw_triangles(&cnv, &cam2d, &cir3d_proj, &cir3d_inx,
			 Color::RGB(0,0,0), true);
	

	
        cnv.present();
	

    }
}
#[derive(Debug)]
struct Camera3D{
    transform: Transform3D,
    fov_y_radians: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32,
}

#[allow(dead_code)]
impl Camera3D{
    fn init(fov_y_radians: f32, aspect_ratio: f32, z_range: [f32;2]) -> Self{
	Self{transform:Transform3D::init(),
	     fov_y_radians, aspect_ratio,
	     z_near: z_range[0], z_far: z_range[1]}
    }
    fn mat(&self) -> Mat4{
	Mat4::perspective_rh(self.fov_y_radians, self.aspect_ratio, self.z_near, self.z_far)
	    * self.transform.mat().inverse()
    }
}

// A system for rendering
// Takes in model -> applies model trans -> camera + proj trans -> makes Vec2 arrays
//       supplies it into 2D triangle drawing
// Need to do backface culling in 2D triangle drawing part ?? (sad)
#[derive(Debug)]
struct Transform3D{
    pos: Vec3,
    rotq: Quat, //Need to rotate this by additional quats
    scale: Vec3,
}

#[allow(dead_code)]
impl Transform3D{
    fn init() -> Self{
	Self{pos:Vec3::ZERO,rotq:Quat::IDENTITY, scale:Vec3::ONE}
    }
    fn reset(self) -> Self{
	Self::init()
    }
    fn from_mat4(affine_mat: &Mat4) -> Self{
	let (s, r, t) = affine_mat.to_scale_rotation_translation();
	Self{ pos: t, rotq: r, scale: s}
    }
    fn translate(self, delta: Vec3)->Self{
	Self{pos: self.pos + delta, ..self}
    }
    fn scalef(self, factor: f32)->Self{
	Self{scale:self.scale*factor, ..self}
    }
    //Angles are in radians
    fn rotatex(self, angle: f32)->Self{
	let rotq = Quat::from_rotation_x(angle);
	Self{rotq:self.rotq.mul_quat(rotq), ..self}
    }
    fn rotatey(self, angle: f32)->Self{
	let rotq = Quat::from_rotation_y(angle);
	Self{rotq:self.rotq.mul_quat(rotq), ..self}
    }
    fn rotatez(self, angle: f32)->Self{
	let rotq = Quat::from_rotation_z(angle);
	Self{rotq:self.rotq.mul_quat(rotq), ..self}
    }
    //Does translate * rotate * scale operation 
    fn mat(&self)->Mat4{
	Mat4::from_translation(self.pos) *
	    Mat4::from_quat(self.rotq) *
	    Mat4::from_scale(self.scale)
    }
}

// Make a function that renders triangles from mesh according to a given camera
fn draw_triangles<T:sdl2::render::RenderTarget>(canvas: &sdl2::render::Canvas<T>,
						cam: &Camera2D, 
						pts: &[Vec2], inxs: &[u16],
						color: Color, draw_mesh: bool) -> Result<(), String>{
  
    let mut ans:Result<(), String> = Ok(());
    
    inxs.chunks(3).for_each(|tr|{
	if (tr.len() == 3) && !ans.is_err(){
	    let pt1 = cam.lookpt(&canvas, pts[tr[0] as usize]);
	    let pt2 = cam.lookpt(&canvas, pts[tr[1] as usize]);
	    let pt3 = cam.lookpt(&canvas, pts[tr[2] as usize]);
	    // Implement culling
	    let cross = (pt2 - pt1).perp_dot(pt3 - pt1);
	    if cross >= 0.0{
		let func = if draw_mesh { sdl2::render::Canvas::<T>::aa_trigon }
		else { sdl2::render::Canvas::<T>::filled_trigon };
		if let Err(err) = func(canvas,
				       pt1.x as i16, pt1.y as i16,
				       pt2.x as i16, pt2.y as i16,
				       pt3.x as i16, pt3.y as i16,
				       color){
		    ans = Err(err);
		}
	    }
	}
    });
    
    ans
}

fn fill_circle<T:sdl2::render::RenderTarget>(canvas: &sdl2::render::Canvas<T>,
					     center: Vec2, radius: f32,
					     color: Color) -> Result<(), String>{
    canvas.filled_circle(center.x as i16, center.y as i16, radius as i16, color)
}

struct Camera2D{
    pos: Vec2,   //Position in the world
    rot: f32,    //Angle with x axis of world in radians
    view: f32,  //Range along x (both front and back) upto which camera can see world
    // Might have to add a preferred direction where we preserve view
    // Maybe make it a vector, need to clip the vector line on window rect
}

#[allow(dead_code)]
impl Camera2D{
    fn init(pos:Vec2, rot:f32, view:f32) -> Camera2D{
	Camera2D{ pos, rot, view }
    }
    fn matrix<T:sdl2::render::RenderTarget>(&self, canvas: &sdl2::render::Canvas<T>) -> Mat3 {
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
    fn lookpt<T:sdl2::render::RenderTarget>(&self, canvas: &sdl2::render::Canvas<T>, point:Vec2) -> Vec2{
	self.matrix(canvas).transform_point2(point)
    }
    fn lookvec<T:sdl2::render::RenderTarget>(&self, canvas: &sdl2::render::Canvas<T>, vector:Vec2) -> Vec2{
	self.matrix(canvas).transform_vector2(vector)
    }
}
