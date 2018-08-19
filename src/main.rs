#[macro_use]
extern crate glium;

use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::glutin;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_multisampling(16);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut closed = false;

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;
    out vec4 my_attr;      // our new attribute

    uniform mat4 matrix;

    void main() {
        my_attr = matrix * vec4(position, 0.0, 1.0);     // we need to set the value of each `out` variable.
        gl_Position = my_attr;
    }
    "#;

    let fragment_shader_src = r#"
    #version 140

    in vec4 my_attr;
    out vec4 color;

    void main() {
        color = my_attr;
    }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut t: f32 = -1.0;
    while !closed {
        let mut target = display.draw();
        t += 0.02;
        if t > 1.0 {
            t = -1.0;
        }
        let uniforms = uniform! {
            matrix: [
                [1.0 + t, 0.0, 0.0, 0.0],
                [0.0, 1.0 + t, 0.0, 0.0],
                [0.0, 0.0, 1.0 + t, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
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
