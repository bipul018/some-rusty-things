extern crate sdl2;
extern crate glam;
use glam::{Vec2, Mat3};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;




macro_rules! generate_circle{
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
	    ::new(&video_subsystem, "My Rust SDL2 Demo", 800, 600)
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

    let mut cam = Camera2D::new(Vec2::new(0.0,0.0), 0.0, 10.0);
    
    'main_loop: loop {
        for event in event_pump.poll_iter() {
	    use Event::KeyDown;
	 
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape) | Some(Keycode::Backspace), .. }
		=> break 'main_loop,

		//Remaining keydown matched together
		KeyDown{keycode: Some(key), ..} =>{
		    match key{
			Keycode::Q => cam.rot -= 0.05,
			Keycode::E => cam.rot += 0.05,
			Keycode::W => cam.pos.y += 0.1,
			Keycode::S => cam.pos.y -= 0.1,
			Keycode::A => cam.pos.x += 0.1,
			Keycode::D => cam.pos.x -= 0.1,
			Keycode::Z => cam.view /= 1.01,
			Keycode::C => cam.view *= 1.01,
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

	_=fill_circle(&cnv, cam.lookpt(&cnv, ball_pos), 30.0, Color::RGB(0,0,255));
	_=fill_circle(&cnv, cam.lookpt(&cnv, ball2_pos), 30.0, Color::RGB(255,0,0));

	let pts: Vec<Vec2> = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]]
	    .iter().map(|&v| Vec2::from_array(v)).collect();
	let inxs: Vec<u16> = [0, 1, 2, 2, 3, 0].iter().map(|x| *x as u16).collect();

	_=draw_triangles(&cnv, &cam, &pts, &inxs, Color::RGB(0,255,0), false);
	_=draw_triangles(&cnv, &cam, &pts, &inxs, Color::RGB(0,0,0), true);
	    
			      
	let (cir_pts, cir_inx) = generate_circle!(10, 1.2, Vec2::new(-3.0,0.0));
	
	_=draw_triangles(&cnv, &cam, &cir_pts, &cir_inx,
			 Color::RGB(255,255,255), false);
	_=draw_triangles(&cnv, &cam, &cir_pts, &cir_inx,
			 Color::RGB(0,0,0), true);
	
        cnv.present();

	
    }
}

// A system for rendering
// Takes in model -> applies model trans -> camera + proj trans -> makes Vec2 arrays
//       supplies it into 2D triangle drawing
// Need to do backface culling in 2D triangle drawing part ?? (sad)


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
    fn new(pos:Vec2, rot:f32, view:f32) -> Camera2D{
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
