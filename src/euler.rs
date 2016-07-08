pub fn euler_int(y0:f32 , t0:f32, tmax:f32,nsteps:i32 , der:fn(f32,f32) -> f32 ) -> (){
    let mut step_no=0;
    let mut y=y0;
    let mut t=t0;
    let step_size=(tmax-t0)/nsteps as f32;
    while step_no <nsteps {
        t+=step_size;
        y+=step_size*der(y,t);
        println!("{} {}", t,y);
        step_no+=1;
    }
}