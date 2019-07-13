use std::time::SystemTime;

extern crate clap;
use clap::{Arg, App};

#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main()
{
    // Argument parsing
    let matches = App::new("CHIP-8 Emulator")
        .arg(Arg::with_name("shader_path")
             .required(true)
             .index(1))
        .get_matches();

    let fragment_path = matches.value_of("shader_path").unwrap();

    use glium::{glutin, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();
    let wb = glium::glutin::WindowBuilder::new();
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let vertex1 = Vertex { position: [-1.0, 1.0] };
    let vertex2 = Vertex { position: [ 3.0, 1.0] };
    let vertex3 = Vertex { position: [ -1.0, -3.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 300 es
        precision highp float;

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_preamble = r#"
        #version 300 es
        precision highp float;

        uniform vec3 iResolution;
        uniform float iTime;
        // uniform float iTimeDelta;
        // uniform float iFrame;
        // uniform float iChannelTime[4];
        // uniform vec4 iMouse;
        // uniform vec4 iDate;
        // uniform float iSampleRate;
        // uniform vec3 iChannelResolution[4];
        // uniform samplerXX iChanneli;
        out vec4 glFragColor;
        #line 0
    "#;

    let fragment_shader_postamble = r#"
        void main() {
            glFragColor.w = 1.0;
            mainImage(glFragColor, gl_FragCoord.xy);
            //mainImage(gl_FragColor, gl_FragCoord.xy);
        }
    "#;

    let fragment_shader_string = std::fs::read(&fragment_path).expect("Unable to read file");
    //let mut fragment_code = String::from_utf8(fragment_shader_string).unwrap();
    let mut fragment_code = String::new();
    fragment_code.push_str(fragment_shader_preamble);
    fragment_code.push_str(&String::from_utf8(fragment_shader_string).unwrap());
    fragment_code.push_str(fragment_shader_postamble);

    let program = match glium::Program::from_source(&display, vertex_shader_src, &fragment_code, None) {
        Result::Ok(val) => val,
        Result::Err(err) => {
            print!("{}", err);
            panic!("error: {}: compilation failed.", fragment_path);
        }
    };

    let time = SystemTime::now();
    let mut closed = false;

    while !closed {
        let mut target = display.draw();

        let time_msecs = time.elapsed().expect("Time error").as_millis();
        let time_secs: f32 = time_msecs as f32 / 1000.0;

        let (x, y) = display.get_framebuffer_dimensions();

        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let uniforms = uniform! {iTime: time_secs, iResolution: [x as f32, y as f32, 0f32]};
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
