

#[macro_export]
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

#[macro_export]
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

#[macro_export]
macro_rules! generate_circle3d{
    ($segments:expr, $radius:expr, $center:expr) =>{{
	let center_3d = $center;
	let center_2d = center_3d.truncate();
	let (pts2d, inxs) = generate_circle2d!($segments, $radius, center_2d);
	let pts3d_array = pts2d.map(|pt2d|{Vec3::new(pt2d.x, pt2d.y, center_3d.z)});
        (pts3d_array, inxs)
    }};
}
