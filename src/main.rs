use std::time::SystemTime;

extern crate clap;
use clap::{Arg, App};

#[macro_use]
extern crate glium;

fn load_fragment_code(fragment_path: &str) -> String
{
    let shadertoy_preamble = r#"
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

    let shadertoy_postamble = r#"
        void main() {
            glFragColor.w = 1.0;
            mainImage(glFragColor, gl_FragCoord.xy);
            //mainImage(gl_FragColor, gl_FragCoord.xy);
        }
    "#;

    // Read whole fragment shader file
    let fragment_shader_string = std::fs::read(fragment_path).expect("Unable to read file");

    // Add ShaderToy specific code
    let mut fragment_code = String::new();
    fragment_code.push_str(shadertoy_preamble);
    fragment_code.push_str(&String::from_utf8(fragment_shader_string).unwrap());
    fragment_code.push_str(shadertoy_postamble);

    fragment_code
}

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
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 300 es
        precision highp float;

        void main() {
            vec2 vertices[3];
            vertices[0] = vec2(-1.0, 1.0);
            vertices[1] = vec2( 3.0, 1.0);
            vertices[2] = vec2(-1.0,-3.0);
            gl_Position = vec4(vertices[gl_VertexID], 0.0, 1.0);
        }
    "#;
    let fragment_code = load_fragment_code(fragment_path);

    let program = match glium::Program::from_source(&display, vertex_shader_src, &fragment_code, None) {
        Result::Ok(val) => val,
        Result::Err(err) => {
            print!("{}", err);
            panic!("error: {}: compilation failed.", fragment_path);
        }
    };

    let time = SystemTime::now();
    let mut is_running = true;

    while is_running {
        let mut target = display.draw();

        let framebuffer_extent = display.get_framebuffer_dimensions();

        let time_msecs = time.elapsed().expect("Time error").as_millis();
        let time_secs: f32 = time_msecs as f32 / 1000.0;

        let uniforms = uniform! {
            iResolution: [framebuffer_extent.0 as f32, framebuffer_extent.1 as f32, 1f32],
            iTime: time_secs,
        };

        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(glium::vertex::EmptyVertexAttributes{len:3}, &indices, &program, &uniforms, &Default::default()).unwrap();
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
    }
}
