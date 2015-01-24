// COSMOLARK COMBAT SIMULATOR

//extern crate cl;
extern crate glutin;
extern crate gl;



fn main() {

    println!("Started Cosmlolark Combat Simulator!");

    let window = match glutin::Window::new() {
        Ok(w) => w,
        Err(e) => panic!("Encountered error while opening window: {}",e),
    };

    window.set_title("Cosmolark Combat Simulator");
    print_window_properties(&window);

    unsafe { window.make_current() };

    gl::load_with(|symbol| window.get_proc_address(symbol));

    unsafe { gl::ClearColor(31.0/256.0,190.0/256.0,214.0/256.0,1.0); }

    while !window.is_closed() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
        window.swap_buffers(); 
        
        println!("{:?}", window.wait_events().next());
    }

}



fn print_window_properties(win : &glutin::Window) {
    /* TODO this is not yet implemented for X11
    match win.get_outer_size() {
        Some((w,h)) => println!("Outer Size: {}x{}",w,h),
        None      => println!("Outer Size not returned")
    };
    */
    match win.get_inner_size() {
        Some((w,h)) => println!("Inner Size: {}x{}",w,h),
        None      => println!("Inner Size not returned")
    };

    println!("High-DPI factor: {}",win.hidpi_factor());
}
