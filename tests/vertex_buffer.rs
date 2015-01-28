#![feature(plugin)]
#![feature(unboxed_closures)]

#[plugin]
extern crate glium_macros;

extern crate glutin;

#[macro_use]
extern crate glium;

use glium::Surface;

mod support;

#[test]
fn vertex_buffer_creation() {
    let display = support::build_display();

    #[vertex_format]
    #[allow(dead_code)]
    #[derive(Copy)]
    struct Vertex {
        field1: [f32; 3],
        field2: [f32; 3],
    }

    glium::VertexBuffer::new(&display, 
        vec![
            Vertex { field1: [-0.5, -0.5, 0.0], field2: [0.0, 1.0, 0.0] },
            Vertex { field1: [ 0.0,  0.5, 1.0], field2: [0.0, 0.0, 1.0] },
            Vertex { field1: [ 0.5, -0.5, 0.0], field2: [1.0, 0.0, 0.0] },
        ]
    );

    display.assert_no_error();
}

#[test]
fn vertex_buffer_mapping_read() {
    let display = support::build_display();

    #[vertex_format]
    #[derive(Copy)]
    struct Vertex {
        field1: [u8; 2],
        field2: [u8; 2],
    }

    let mut vb = glium::VertexBuffer::new(&display, 
        vec![
            Vertex { field1: [ 2,  3], field2: [ 5,  7] },
            Vertex { field1: [12, 13], field2: [15, 17] },
        ]
    );

    let mapping = vb.map();
    assert_eq!(mapping[0].field1.as_slice(), [2, 3].as_slice());
    assert_eq!(mapping[1].field2.as_slice(), [15, 17].as_slice());

    display.assert_no_error();
}

#[test]
fn vertex_buffer_mapping_write() {
    let display = support::build_display();
    
    #[vertex_format]
    #[derive(Copy)]
    struct Vertex {
        field1: [u8; 2],
        field2: [u8; 2],
    }

    let mut vb = glium::VertexBuffer::new(&display, 
        vec![
            Vertex { field1: [ 2,  3], field2: [ 5,  7] },
            Vertex { field1: [12, 13], field2: [15, 17] },
        ]
    );

    {
        let mut mapping = vb.map();
        mapping[0].field1 = [0, 1];
    }

    let mapping = vb.map();
    assert_eq!(mapping[0].field1.as_slice(), [0, 1].as_slice());
    assert_eq!(mapping[1].field2.as_slice(), [15, 17].as_slice());

    display.assert_no_error();
}

#[test]
fn vertex_buffer_read() {
    let display = support::build_display();

    #[vertex_format]
    #[derive(Copy)]
    struct Vertex {
        field1: [u8; 2],
        field2: [u8; 2],
    }

    let vb = glium::VertexBuffer::new(&display, 
        vec![
            Vertex { field1: [ 2,  3], field2: [ 5,  7] },
            Vertex { field1: [12, 13], field2: [15, 17] },
        ]
    );

    let data = match vb.read_if_supported() {
        Some(d) => d,
        None => return
    };

    assert_eq!(data[0].field1.as_slice(), [2, 3].as_slice());
    assert_eq!(data[1].field2.as_slice(), [15, 17].as_slice());

    display.assert_no_error();
}

#[test]
fn vertex_buffer_read_slice() {
    let display = support::build_display();

    #[vertex_format]
    #[derive(Copy)]
    struct Vertex {
        field1: [u8; 2],
        field2: [u8; 2],
    }

    let vb = glium::VertexBuffer::new(&display, 
        vec![
            Vertex { field1: [ 2,  3], field2: [ 5,  7] },
            Vertex { field1: [12, 13], field2: [15, 17] },
        ]
    );

    let data = match vb.read_slice_if_supported(1, 1) {
        Some(d) => d,
        None => return
    };

    assert_eq!(data[0].field2.as_slice(), [15, 17].as_slice());
    
    display.assert_no_error();
}

#[test]
#[should_fail]
fn vertex_buffer_read_slice_out_of_bounds() {
    let display = support::build_display();

    #[vertex_format]
    #[derive(Copy)]
    struct Vertex {
        field1: [u8; 2],
        field2: [u8; 2],
    }

    let vb = glium::VertexBuffer::new(&display, 
        vec![
            Vertex { field1: [ 2,  3], field2: [ 5,  7] },
            Vertex { field1: [12, 13], field2: [15, 17] },
        ]
    );

    vb.read_slice_if_supported(0, 3).unwrap();
}

#[test]
fn vertex_buffer_any() {
    let display = support::build_display();

    #[vertex_format]
    #[allow(dead_code)]
    #[derive(Copy)]
    struct Vertex {
        field1: [f32; 3],
        field2: [f32; 3],
    }

    glium::VertexBuffer::new(&display, 
        vec![
            Vertex { field1: [-0.5, -0.5, 0.0], field2: [0.0, 1.0, 0.0] },
            Vertex { field1: [ 0.0,  0.5, 1.0], field2: [0.0, 0.0, 1.0] },
            Vertex { field1: [ 0.5, -0.5, 0.0], field2: [1.0, 0.0, 0.0] },
        ]
    ).into_vertex_buffer_any();

    display.assert_no_error();
}

#[test]
fn vertex_buffer_write() {
    let display = support::build_display();
    
    #[vertex_format]
    #[derive(Copy)]
    struct Vertex {
        field1: [u8; 2],
        field2: [u8; 2],
    }

    let mut vb = glium::VertexBuffer::new(&display, 
        vec![
            Vertex { field1: [ 2,  3], field2: [ 5,  7] },
            Vertex { field1: [ 0,  0], field2: [ 0,  0] },
        ]
    );

    vb.write(1, vec![Vertex { field1: [12, 13], field2: [15, 17] }]);

    let data = match vb.read_if_supported() {
        Some(d) => d,
        None => return
    };

    assert_eq!(data[0].field1.as_slice(), [2, 3].as_slice());
    assert_eq!(data[0].field2.as_slice(), [5, 7].as_slice());
    assert_eq!(data[1].field1.as_slice(), [12, 13].as_slice());
    assert_eq!(data[1].field2.as_slice(), [15, 17].as_slice());

    display.assert_no_error();
}

#[test]
fn multiple_buffers_source() {
    let display = support::build_display();

    let buffer1 = {
        #[vertex_format]
        #[derive(Copy)]
        struct Vertex {
            position: [f32; 2],
        }

        glium::VertexBuffer::new(&display, 
            vec![
                Vertex { position: [-1.0,  1.0] },
                Vertex { position: [ 1.0,  1.0] },
                Vertex { position: [-1.0, -1.0] },
                Vertex { position: [ 1.0, -1.0] },
            ]
        )
    };

    let buffer2 = {
        #[vertex_format]
        #[derive(Copy)]
        struct Vertex {
            color: [f32; 3],
        }

        glium::VertexBuffer::new(&display, 
            vec![
                Vertex { color: [1.0, 0.0, 0.0] },
                Vertex { color: [1.0, 0.0, 0.0] },
                Vertex { color: [1.0, 0.0, 0.0] },
                Vertex { color: [1.0, 0.0, 0.0] },
            ]
        )
    };

    let index_buffer = glium::IndexBuffer::new(&display,
        glium::index::TriangleStrip(vec![0u16, 1, 2, 3]));

    let program = glium::Program::from_source(&display,
        "
            #version 110

            attribute vec2 position;
            attribute vec3 color;

            varying vec3 v_color;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                v_color = color;
            }
        ",
        "
            #version 110
            varying vec3 v_color;

            void main() {
                gl_FragColor = vec4(v_color, 1.0);
            }
        ",
        None)
        .unwrap();

    let texture = support::build_renderable_texture(&display);
    texture.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);
    texture.as_surface().draw((&buffer1, &buffer2), &index_buffer, &program, &uniform!{},
                              &std::default::Default::default()).unwrap();

    let data: Vec<Vec<(f32, f32, f32, f32)>> = texture.read();
    for row in data.iter() {
        for pixel in row.iter() {
            assert_eq!(pixel, &(1.0, 0.0, 0.0, 1.0));
        }
    }
    
    display.assert_no_error();
}
