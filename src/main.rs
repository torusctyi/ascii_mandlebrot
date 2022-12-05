use num::complex::Complex;
use std::iter::repeat;
use itertools::Itertools;
use itertools::iterate;

// number of lattice points in x (resp y) direction
const N_X_MAX: usize = 230;
const N_Y_MAX: usize = 100;

const X_RANGE: (f64, f64) = (-2.3, 1.4);
const Y_LIM  : f64        = 2.0;

const Y_RANGE: (f64, f64) = (0.0, Y_LIM); 


const MAX_ITERS: usize = 5000;

const FILL_CHAR  : char  = '*';
const BLANK_CHAR : char  = ' ';


fn main() {

   let upper_half : Vec<String>  = (0..N_Y_MAX).rev() // draw the image from the upper limit of Y_RANGE downwards
                                 .map(|y| generate_line(y))
                                 .collect();
  
   let lower_half : Vec<String> = upper_half.iter().cloned()
                                .rev()
                                .skip(1)    //otherwise generate_line(0) would be plotted twice
                                .collect();


   let result = upper_half.iter().chain(lower_half.iter()).join("\n");
   print!("{}", result);
}
                                              
fn generate_line(y: usize) -> String{
   
  (0..N_X_MAX).into_iter()                       
              .zip(repeat(y))
              .map(|pt| lattice_to_complex(pt))
              .map(|z| test_number(z))
              .map(|res| if res {FILL_CHAR} else {BLANK_CHAR})
              .collect()
}

fn test_number(c: Complex<f64>) -> bool{
   
    
    // special case where c is outside of the ring of divergence
    if c.norm() > 2.0 {return false};

    // special case where c is a real number 
    if c.im == 0.0 && (c.re >= -2.0 && c.re <= 0.25) {return true}
    

    return match _test_number(c){
           None => true,
           Some(_) => false
    };
   

   fn _test_number(c: Complex<f64>) -> Option<Complex<f64>>{
     iterate(c, |z| z*z + c).take(MAX_ITERS).find(|w| w.norm() > 2.0)
   }
}

fn lattice_to_complex(lattice_point: (usize, usize)) -> Complex<f64>{
	let x_val = discrete_to_float_single(lattice_point.0, N_X_MAX,  X_RANGE.0, X_RANGE.1);
	let y_val = discrete_to_float_single(lattice_point.1, N_Y_MAX, Y_RANGE.0, Y_RANGE.1); 

	return Complex::new(x_val, y_val);

	fn discrete_to_float_single(val: usize, max: usize, lower_limit: f64, upper_limit: f64) -> f64{
		let ratio = (val as f64)/(max as f64);

		return  lower_limit + (upper_limit - lower_limit)*ratio ;
	}
}

