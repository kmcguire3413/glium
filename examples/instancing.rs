#![feature(plugin)]

#[plugin]
extern crate glium_macros;

extern crate glutin;

#[macro_use]
extern crate glium;

use glium::Surface;

fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[vertex_format]
        #[derive(Copy)]
        struct Vertex {
            position: [f32; 2]
        }

        glium::VertexBuffer::new(&display, 
            vec![
                Vertex { position: [-0.005, -0.005] },
                Vertex { position: [  0.0 , 0.005] },
                Vertex { position: [ 0.005, -0.005] },
            ]
        )
    };

    // building the instances buffer
    let per_instance = {
        #[vertex_format]
        #[derive(Copy)]
        struct Attr {
            world_position: [f32; 2],
        }

        let mut data = Vec::new();
        for x in (0u32 .. 104) {
            for y in (0u32 .. 82) {
                data.push(Attr {
                    world_position: [((x as f32) / 50.0) - 1.0, ((y as f32) / 40.0) - 1.0],
                });
            }
        }

        glium::vertex::PerInstanceAttributesBuffer::new_if_supported(&display, data).unwrap()
    };

    let index_buffer = glium::IndexBuffer::new(&display,
        glium::index::TrianglesList(vec![0u16, 1, 2]));

    let program = glium::Program::from_source(&display,
        "
            #version 110

            attribute vec2 position;
            attribute vec2 world_position;

            void main() {
                gl_Position = vec4(position + world_position, 0.0, 1.0);
            }
        ",
        "
            #version 110

            void main() {
                gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
            }
        ",
        None)
        .unwrap();
    
    // the main loop
    // each cycle will draw once
    'main: loop {
        use std::io::timer;
        use std::time::Duration;

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw((&vertex_buffer, &per_instance), &index_buffer, &program, &uniform!{},
                    &std::default::Default::default()).unwrap();
        target.finish();

        // sleeping for some time in order not to use up too much CPU
        timer::sleep(Duration::milliseconds(17));

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => ()
            }
        }
    }
}
