mod euler;
fn main() {
    println!("Hello, world!");
    euler::euler_int(1.0,0.0,1.0,100, der_fun);
}

fn der_fun(y:f32,t:f32)->f32{
    return -y*1.0;
}