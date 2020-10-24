use std::time::SystemTime;

extern crate notify;
use notify::Watcher;

extern crate clap;
use clap::{Arg, App};

#[macro_use]
extern crate glium;

fn main()
{
    // Argument parsing
    let matches = App::new("ShaderToy Viewer")
        .arg(Arg::with_name("shader_path")
             .help("path of the GLSL shader")
             .required(true)
             .index(1))
        .get_matches();

    let fragment_path = matches.value_of("shader_path").unwrap();

    execute_main_loop(fragment_path);
}

fn execute_main_loop(fragment_path: &str)
{
    use glium::{glutin, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();
    let wb = glium::glutin::WindowBuilder::new();
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_code = std::fs::read_to_string("./shader/shadertoy.vert.glsl").expect("Unable to read file");

    let default_fragment_path = "./shader/default.frag.glsl";
    let default_fragment_code = load_fragment_code(default_fragment_path);
    let default_program = match glium::Program::from_source(&display, &vertex_code, &default_fragment_code, None) {
        Ok(val) => val,
        Err(err) => {
            print!("{}", err);
            panic!("error: {}: compilation failed.", fragment_path);
        }
    };

    let mut fragment_code = load_fragment_code(fragment_path);
    let mut program = glium::Program::from_source(&display, &vertex_code, &fragment_code, None);

    match &program {
        Ok(_) => {},
        Err(err) => { print!("{}", err); }
    };

    let mut frame_index : u64 = 0;
    let time = SystemTime::now();
    let mut is_running = true;
    let mut should_reload_shader = false;

    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    let mut watcher: notify::RecommendedWatcher = notify::Watcher::new(tx, std::time::Duration::from_millis(20)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    // TODO only watch the file we are interested in
    watcher.watch(".", notify::RecursiveMode::Recursive).unwrap();

    while is_running {
        // Watch FS for changes to our file
        // TODO improve very weak error handling
        loop {
            match rx.try_recv() {
                Ok(event) => match event {
                    notify::DebouncedEvent::Write(path) | notify::DebouncedEvent::Create(path) => {
                        println!("info: {}: file change detected.", path.to_str().unwrap());
                        should_reload_shader = true;
                    },
                    _ => {}
                },
                Err(_) => { break; }
            };
        }

        if should_reload_shader {
            fragment_code = load_fragment_code(fragment_path);
            program = glium::Program::from_source(&display, &vertex_code, &fragment_code, None);

            match &program {
                Ok(_) => {},
                Err(err) => { print!("{}", err); }
            };
            should_reload_shader = false;
        }

        let mut target = display.draw();

        let framebuffer_extent = display.get_framebuffer_dimensions();

        let time_msecs = time.elapsed().expect("Time error").as_millis();
        let time_secs: f32 = time_msecs as f32 / 1000.0;

        let uniforms = uniform! {
            iResolution: [framebuffer_extent.0 as f32, framebuffer_extent.1 as f32, 1f32],
            iTime: time_secs,
            iFrame: frame_index as f32,
        };

        target.clear_color(0.0, 0.0, 1.0, 1.0);

        // Allow fallback on default program when there's an error
        let program_to_use = match &program {
            Ok(val) => &val,
            Err(_) => &default_program
        };

        target.draw(glium::vertex::EmptyVertexAttributes{len:3}, &indices, &program_to_use, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => is_running = false,
                    _ => (),
                },
                _ => (),
            }
        });

        frame_index += 1;
    }
}

fn load_fragment_code(fragment_path: &str) -> String
{
    // Load ShaderToy specific code
    let shadertoy_preamble = std::fs::read_to_string("./shader/shadertoy_intro.frag.glsl").expect("Unable to read file");
    let shadertoy_postamble = std::fs::read_to_string("./shader/shadertoy_outro.frag.glsl").expect("Unable to read file");

    // Read whole fragment shader file
    let fragment_shader_string = std::fs::read_to_string(fragment_path).expect("Unable to read file");

    shadertoy_preamble + &fragment_shader_string + &shadertoy_postamble
}
