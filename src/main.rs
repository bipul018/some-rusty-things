extern crate sdl2;
extern crate glam;
#[allow(unused_imports)]
use glam::{Vec2, Vec3, Mat4, Vec4};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::FRect;    

// Import the model generation macros
mod generators;


// Import all the transformations things
mod transformations;
use transformations::*;

// Import moving average macro thing
mod movavg;

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
    let mut cam3d = Camera3D::init(std::f32::consts::PI/3.0, 9.0/9.0, [10.0,30.0]);
    // cam3d.transform = cam3d.transform
    // 	.translate(Vec3::new(0.0, 10.0, 0.0))
    // 	.scalef(10.0)
    // 	.rotatex(std::f32::consts::PI/2.0);
    cam3d.transform = Transform3D::from_mat4(
	&Mat4::look_at_rh(Vec3::new(0.0, 20.0, 0.0),
			  Vec3::new(0.0, 0.0, 0.0),
			  Vec3::new(0.0, 0.0, -1.0)).inverse());

    let mut model_trns = Transform3D::init();
    let model_color = Color::RGB(255,255,0);

    //Models used
    //let (cir3d_pts, cir3d_inx) = generate_circle3d!(10, 1.2, Vec3::new(4.0, 4.0, 1.0));
    let (cir3d_pts, cir3d_inx) = generate_sphere3d!(10, 1.0, Vec3::new(0.0,0.0,0.0));

    MakeMovAvg!{MovAvg, f64, 60}
    // used for calculating ms of rendering by pixels
    let mut pix_rndr_time = MovAvg::init(0.0);

    // used for calculating ms of rendering by sdl
    let mut sdl_rndr_time = MovAvg::init(0.0);
    // used for overall rendering
    let mut all_rndr_time = MovAvg::init(0.0);

    let tex_crtr = cnv.texture_creator();
    const TEX_W:usize = 320; const TEX_H:usize = 180;
    let mut tex = match tex_crtr.create_texture_streaming(sdl2::pixels::PixelFormatEnum::ABGR8888,TEX_W as u32,TEX_H as u32){
	Err(err) => {
	    println!("Got a TextureValueError while trying to create a streaming texture as {:?}",
		     err);
	    return;
	},
	Ok(ok) => ok
    };

    let tex_qry = tex.query();
    println!("Created a streaming texture as {:?}", tex_qry);
    let mut tex_dst = FRect::new(0.0,0.0, TEX_W as f32, TEX_H as f32);

    let mut tex_arr = [Color::RGBA(255,0,0,255);TEX_W*TEX_H];

    //Debug 2d camera
    {
	let in_pts=[
	    Vec2::new(0.0,0.0),
	    Vec2::new(-1.0, -1.0),
	    Vec2::new(1.0, 1.0),
	];

	let dims=[
	    [200, 100],
	    [150, 300]
	];

	for v in in_pts.iter(){
	    for w in dims.iter(){
		let o = ndc_mat(w[0], w[1]).transform_point2(*v);
		println!("v = {}, w = {:?}, o = {}", v, w, o);
	    }
	}
	
    }
    
    // This part used for debugging whether ray was casted properly
    // {
    // 	let eye = Vec3::new(0.0, 20.0, 0.0);
    // 	let center = Vec3::new(0.0, 0.0, 0.0);	    
    // 	let up = Vec3::new(0.0, 0.0, -1.0);
    // 	let atmat = Mat4::look_at_lh(eye, center, up);
    // 	let atinv = atmat.inverse();

    // 	println!("Transforming eye by atinv {}", atinv.transform_point3(eye));
    // 	println!("Transforming center by atinv {}", atinv.transform_point3(center));
    // 	println!("Transforming up by atinv {}", atinv.transform_vector3(up));

    // 	println!("Transforming origin by atmat {}", atmat.transform_point3(Vec3::ZERO));

    // 	println!("Transforming eye, center by atmat {} {}",
    // 		 atmat.transform_point3(eye),
    // 		 atmat.transform_point3(center));
	
    // 	let (x,y)=(TEX_W as f32 / 2.0, TEX_H as f32 / 2.0);
    // 	//let (x,y)=(0.0,0.0);
    // 	let mmat = model_trns.mat().inverse();
    // 	let c2dmat = cam2d.matrix(TEX_W, TEX_H).inverse();
    // 	let normpos = c2dmat.transform_point2(Vec2::new(x as f32, y as f32));

    // 	let (p,v) = cam3d.get_ray(normpos);
    // 	let (p2,v2) = (mmat.transform_point3(p), mmat.transform_vector3(v));
	
    // 	// Now the line is wrt the unit sphere
    // 	let pt = - v2.dot(p2) / v2.dot(v2);
    // 	let xpt = pt * v2 + p2;
    // 	println!("normpos = {}, p2 = {}, v2 = {}, pt = {} xpt = {}", normpos, p2, v2, pt, xpt);
    // 	println!("p = {}, v = {}", p, v);
    // 	if xpt.dot(xpt) <= 1.0{
    // 	    //Check front or back , for now consider pt will work (it will not)
    // 	    //if pt > 0.0{
    // 	    println!("sphere hit");
    // 	    //}
    // 	}
    // }    
    
    
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
			    Keycode::Right => tex_dst.x += 5.0,
			    Keycode::Left => tex_dst.x -= 5.0,
			    Keycode::Up => tex_dst.y -= 5.0,
			    Keycode::Down => tex_dst.y += 5.0,

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

	//Update portion
	{
	    ball_pos = ball_pos + ball_vel;
	    ball_vel = ball_vel + ball_acc;

	    // let win_size = Vec2::new(cnv.window().drawable_size().0 as f32,
	    // 			 cnv.window().drawable_size().1 as f32);
	    let win_size = Vec2::new(5.0, 5.0);
	    
	    if ball_pos.y >= win_size.y{
		ball_pos.y = win_size.y - (ball_pos.y - win_size.y);
		ball_vel.y = -ball_vel.y;
	    }
	}

	let all_rndr_timer = std::time::Instant::now();
	
	// Render pixel by pixel portion
	{
	    let pix_rndr_timer = std::time::Instant::now();
	    
	    let mut tex_pixel = |x:usize,y:usize, pix:Option<Color>|{
		let prev_pix = tex_arr[y*TEX_W+x];
		if let Some(p)=pix{
		    tex_arr[y*TEX_W+x] = p;
		}
		prev_pix
	    };
	    
	    let mmat = model_trns.mat().inverse();
	    let c2dmat = cam2d.matrix(TEX_W, TEX_H).inverse();
	    for x in 0..TEX_W{
		for y in 0..TEX_H{
		    let mut col = Color::RGB(0,0,0);
		    
		    //let dims = Vec2::new(TEX_W as f32, TEX_H as f32);
		    //let cenpos = Vec2::new(x as f32, y as f32) - dims * 0.5;

		    //let normpos = (cenpos / dims) * 2.0;
		    let normpos = c2dmat.transform_point2(Vec2::new(x as f32, y as f32));

		    let (p,v) = cam3d.get_ray(normpos);

		    let (p2,v2) = (mmat.transform_point3(p), mmat.transform_vector3(v));

		    // Now the line is wrt the unit sphere

		    let pt = - v2.dot(p2) / v2.dot(v2);
		    let xpt = pt * v2 + p2;
		    if xpt.dot(xpt) <= 1.0{
			//Check front or back , for now consider pt will work (it will not)
			//if pt > 0.0{
			//col = Color::RGB(255,255,255);
			col = model_color;
			//}
		    }
		    

		    _=tex_pixel(x,y,Some(col));
		    //_=tex_pixel(x,y,Some(Color::RGBA(255,0,0,255)));
		}
	    }

	    //Draw into texture
	    let transmuted:&[u8] = unsafe{ std::mem::transmute(&tex_arr[0..]) };
	    _=tex.update(None, transmuted, 
	 		 TEX_W * std::mem::size_of::<Color>());

	    pix_rndr_time.insert(pix_rndr_timer.elapsed().as_millis() as f64);
	}

	cnv.set_draw_color(bg_col);
        cnv.clear();

	
	// Render using sdl portion
	{
	    let sdl_rndr_timer = std::time::Instant::now();

	    let width = cnv.viewport().width() as usize;
	    let height = cnv.viewport().height() as usize;
	    

	    _=fill_circle(&cnv, cam2d.lookpt(width, height, ball_pos), 30.0, Color::RGB(0,0,255));
	    _=fill_circle(&cnv, cam2d.lookpt(width, height, ball2_pos), 30.0, Color::RGB(255,0,0));

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
			     model_color, false);
	    _=draw_triangles(&cnv, &cam2d, &cir3d_proj, &cir3d_inx,
			     Color::RGB(0,0,0), true);

	    
	    // need to get mouse point, transform it back to 3d near plane,
	    //      then draw a line from that 3d point to camera
	    // Or, just draw a sphere at the point??
	    {
		let mouse_state: sdl2::mouse::MouseState = event_pump.mouse_state();

		let mpos = Vec2::new(mouse_state.x() as f32, mouse_state.y() as f32);
		
		let c2dmat = cam2d.matrix(cnv.viewport().width() as usize,
					  cnv.viewport().height() as usize).inverse();
	
		//let dims = Vec2::new(TEX_W as f32, TEX_H as f32);
		//let cenpos = Vec2::new(x as f32, y as f32) - dims * 0.5;

		//let normpos = (cenpos / dims) * 2.0;
		let normpos = c2dmat.transform_point2(mpos);

		let (p,_v) = cam3d.get_ray(normpos);

		let pback3d = cam_proj.project_point3(p);
		let pback2d = c2dmat.transform_point2(pback3d.truncate());
		_=cnv.string(10, 280,
			     &format!("OG mpos = {:.2}, normpos = {:.2}", mpos, normpos),
			     Color::RGB(0,0,0));
		_=cnv.string(10, 300,
			     &format!("3dpos = {:.2}, pback3d = {:.2}", p, pback3d),
			     Color::RGB(0,0,0));
		_=cnv.string(10, 320,
			     &format!("pback2d = {:.2}", pback2d),
			     Color::RGB(0,0,0));


		let sph3d_proj = cir3d_pts.map(|pt3d|{
		    cam_proj.project_point3(pt3d + p).truncate()
		});
		_=draw_triangles(&cnv, &cam2d, &sph3d_proj, &cir3d_inx,
				 model_color, false);
		
		
	    }
	    sdl_rndr_time.insert(sdl_rndr_timer.elapsed().as_millis() as f64);
	}

	all_rndr_time.insert(all_rndr_timer.elapsed().as_millis() as f64);
	
	// Draw data from texture
	_=cnv.copy_f(&tex, None, tex_dst);

	_=cnv.string(10,10,&format!("Pixel Rendering Time : {:.2}", pix_rndr_time.get()), Color::RGB(0,0,0));
	_=cnv.string(10,20,&format!("SDL Rendering Time : {:.2}", sdl_rndr_time.get()), Color::RGB(0,0,0));
	_=cnv.string(10,30,&format!("Overall Rendering Time : {:.2}",all_rndr_time.get()), Color::RGB(0,0,0));

        cnv.present();
    }


}

// Make a function that renders triangles from mesh according to a given camera
fn draw_triangles<T:sdl2::render::RenderTarget>(canvas: &sdl2::render::Canvas<T>,
						cam: &Camera2D, 
						pts: &[Vec2], inxs: &[u16],
						color: Color, draw_mesh: bool) -> Result<(), String>{
  
    let mut ans:Result<(), String> = Ok(());
    
    let width = canvas.viewport().width() as usize;
    let height = canvas.viewport().height() as usize;
    
    inxs.chunks(3).for_each(|tr|{
	if (tr.len() == 3) && !ans.is_err(){
	    let pt1 = cam.lookpt(width, height, pts[tr[0] as usize]);
	    let pt2 = cam.lookpt(width, height, pts[tr[1] as usize]);
	    let pt3 = cam.lookpt(width, height, pts[tr[2] as usize]);
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
