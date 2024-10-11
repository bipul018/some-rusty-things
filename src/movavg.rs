
#[macro_export]
macro_rules! MakeMovAvg{
    ($struct_name:ident, $base_type:ty, $size:expr) => {
	struct $struct_name{
	    arr:[$base_type;$size],
	    inx:usize,
	    sum:$base_type,
	}
	impl $struct_name{
	    fn init(init_val: $base_type) -> Self{
		Self{
		    arr: [init_val;$size],
		    inx: 0,
		    sum: init_val*($size as $base_type)
		}
	    }
	    fn insert(&mut self, new_val: $base_type) -> $base_type{
		self.sum -= self.arr[self.inx];
		self.arr[self.inx] = new_val;
		self.sum += new_val;
		self.inx = (self.inx + 1) % $size;
		self.sum / ($size as $base_type)
	    }
	    fn get(&self) -> $base_type{
		self.sum / ($size as $base_type)
	    }
	}
    }
}

// pub fn main(){
//     MakeMovAvg!{MovAvg, f32, 5}
//     let mut avgs = MovAvg::init(0.0);
//     println!("Avg 1 : {}", avgs.insert(1.0));
//     println!("Avg 1 : {}", avgs.insert(1.0));
//     println!("Avg 1 : {}", avgs.insert(1.0));
//     println!("Avg 1 : {}", avgs.insert(1.0));
//     println!("Avg 1 : {}", avgs.insert(1.0));
//     println!("Avg 1 : {}", avgs.insert(1.0));    
// }
