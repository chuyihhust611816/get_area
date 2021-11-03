use std::fmt::Debug;

#[derive(Debug)]
struct Square {
    side: f64,  
}
impl Square {
    fn new(s: f64) -> Square {
        Square {
            side: s,
        }
    }
}

#[derive(Debug)]
struct Triangle {
    base: f64, 
    height: f64,    
}
impl Triangle {
    fn new(be: f64, ht: f64) -> Triangle {
        Triangle {
            base: be,
            height: ht,
        }
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64, 
}
impl Circle {
    fn new(rs: f64) -> Circle {
        Circle {
            radius: rs
        }
    }
}

trait Area {
    fn area(&self) -> f64;
}



impl Area for Square {
    fn area(&self) -> f64 {
        self.side.powf(2.0)
        
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        (self.base * self.height) / 2.0 
    }
 }

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)  
    }
}

fn print_area<T: Area + Debug>(t: T) {
    println!("图形:{:?}, 面积:{}",t, t.area());
}


fn main() {
    let t1 = Square::new(2.0);
    let t2 = Triangle::new(5.0, 4.0); 
    let t3 = Circle::new(2.0);
    print_area(t1);
    print_area(t2);
    print_area(t3);
}
