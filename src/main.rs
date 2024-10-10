extern crate sdl2;
extern crate glam;
extern crate running_average;
use glam::{Vec2, Vec3, Mat4};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::FRect;    

// Import the model generation macros
mod generators;

// Import all the transformations things
mod transformations;
use transformations::*;

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

    let mut cam2d = Camera2D::init(Vec2::new(0.0,0.0), 0.0, 10.0);
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

    //Models used
    //let (cir3d_pts, cir3d_inx) = generate_circle3d!(10, 1.2, Vec3::new(4.0, 4.0, 1.0));
    let (cir3d_pts, cir3d_inx) = generate_sphere3d!(10, 1.0, Vec3::new(0.0,0.0,0.0));

    let mut time_window = running_average::RealTimeRunningAverage::default();

    // Draw on a custom surface first
    
    // let surface = match sdl2::surface::Surface::new(100, 100,
    // 						    sdl2::pixels::PixelFormatEnum::RGB24){
    // 	Ok(ok) => ok,
    // 	Err(err) => {
    // 	    println!("Error occured in creating new surface : {}", err);
    // 	    return;
    // 	}
    // };
    let tex_crtr = cnv.texture_creator();
    let tex_w = 100; let tex_h = 100;
    let mut tex = match tex_crtr.create_texture_streaming(None,tex_w,tex_h){
	Err(err) => {
	    println!("Got a TextureValueError while trying to create a streaming texture as {:?}",
		     err);
	    return;
	},
	Ok(ok) => ok
    };

    let tex_qry = tex.query();
    println!("Created a streaming texture as {:?}", tex_qry);
    let mut tex_dst = FRect::new(0.0,0.0, tex_w as f32, tex_h as f32);
    
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
			    Keycode::Right => tex_dst.x += 1.0,
			    Keycode::Left => tex_dst.x -= 1.0,
			    Keycode::Up => tex_dst.y -= 1.0,
			    Keycode::Down => tex_dst.y += 1.0,

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

	
	//Draw into texture
	_=tex.with_lock(None, |data: &mut [u8], pitch: usize|{
	    data.chunks_mut(4*10).for_each(|chk|{
		for i in chk.iter_mut(){
		    *i=255;
		}
		chk[4*9+0] = 0; chk[4*9+1] = 0; chk[4*9+2] = 0;
	    });
	});
	
	cnv.set_draw_color(bg_col);
        cnv.clear();

	//Time draw
	let now = std::time::Instant::now();
	
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

	let cam_proj = cam3d.mat();
	let cir3d_proj = cir3d_pts.map(|pt3d|{
	    cam_proj.project_point3(model_trns.mat().transform_point3(pt3d)).truncate()
	});
	_=draw_triangles(&cnv, &cam2d, &cir3d_proj, &cir3d_inx,
			 Color::RGB(255,255,0), false);
	_=draw_triangles(&cnv, &cam2d, &cir3d_proj, &cir3d_inx,
			 Color::RGB(0,0,0), true);
	

	let elapsed = now.elapsed();
	time_window.insert(elapsed.as_millis() as f64);

	
	_=cnv.copy_f(&tex, None, tex_dst);



	
        cnv.present();
    }

    println!("The average time over past few frames is {}", time_window.measurement());
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
