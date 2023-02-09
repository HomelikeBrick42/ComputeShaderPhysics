use glfw::{Action, Context, Key, OpenGlProfileHint, WindowEvent, WindowHint};
use rand::random;

#[repr(C)]
pub struct Circle {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::ContextVersion(4, 6));
    glfw.window_hint(WindowHint::OpenGlDebugContext(true));

    let (mut window, events) = glfw
        .create_window(640, 480, "Compute testing", glfw::WindowMode::Windowed)
        .unwrap();

    window.set_all_polling(true);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::None);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        extern "system" fn message_callback(
            _source: u32,
            _typ: u32,
            _id: u32,
            _severity: u32,
            _length: i32,
            message: *const i8,
            _user_param: *mut std::ffi::c_void,
        ) {
            let str = unsafe { std::ffi::CStr::from_ptr(message) }
                .to_str()
                .unwrap();
            eprintln!("{str}");
        }

        gl::Enable(gl::DEBUG_OUTPUT);
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(message_callback), std::ptr::null());
        gl::DebugMessageControl(
            gl::DONT_CARE,
            gl::DONT_CARE,
            gl::DEBUG_SEVERITY_NOTIFICATION,
            0,
            std::ptr::null(),
            false as _,
        );
    }

    unsafe fn compile_shader(source: &str, typ: u32) -> Result<u32, String> {
        let shader = gl::CreateShader(typ);
        let ptr = source.as_ptr() as *const i8;
        let len = source.len() as i32;
        gl::ShaderSource(shader, 1, &ptr, &len);
        gl::CompileShader(shader);
        let compile_status = {
            let mut status = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
            status
        };

        if compile_status != 0 {
            Ok(shader)
        } else {
            let mut info_log_length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_log_length);
            let mut bytes = vec![0; info_log_length as _];

            let mut written_length = 0;
            gl::GetShaderInfoLog(
                shader,
                info_log_length,
                &mut written_length,
                bytes.as_mut_ptr() as _,
            );

            gl::DeleteShader(shader);

            bytes.truncate(written_length as _);
            Err(String::from_utf8(bytes).unwrap())
        }
    }

    let square_shader = unsafe {
        let vertex_shader =
            compile_shader(include_str!("./circle.vert.glsl"), gl::VERTEX_SHADER).unwrap();
        let fragment_shader =
            compile_shader(include_str!("./circle.frag.glsl"), gl::FRAGMENT_SHADER).unwrap(); // TODO: delete the vertex shader if the fragment shader fails

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        let link_status = {
            let mut status = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
            status
        };

        gl::DetachShader(program, vertex_shader);
        gl::DetachShader(program, fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        if link_status != 0 {
            Ok(program)
        } else {
            let mut info_log_length = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut info_log_length);
            let mut bytes = vec![0; info_log_length as _];

            let mut written_length = 0;
            gl::GetProgramInfoLog(
                program,
                info_log_length,
                &mut written_length,
                bytes.as_mut_ptr() as _,
            );

            gl::DeleteProgram(program);

            bytes.truncate(written_length as _);
            Err(String::from_utf8(bytes).unwrap())
        }
    }
    .unwrap();

    let (square_vertex_array, square_vertex_buffer, square_vertex_count, square_primitive_type) = unsafe {
        let mut vertex_array = 0;
        gl::GenVertexArrays(1, &mut vertex_array);
        gl::BindVertexArray(vertex_array);

        let mut vertex_buffer = 0;
        gl::GenBuffers(1, &mut vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);

        #[repr(C)]
        struct Vertex {
            position: [f32; 2],
            uv: [f32; 2],
        }
        let vertices = [
            Vertex {
                position: [-0.5, -0.5],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5],
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5],
                uv: [1.0, 1.0],
            },
        ];
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(&vertices) as _,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            false as _,
            std::mem::size_of::<Vertex>() as _,
            (0 * std::mem::size_of::<f32>()) as _,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            false as _,
            std::mem::size_of::<Vertex>() as _,
            (2 * std::mem::size_of::<f32>()) as _,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        (
            vertex_array,
            vertex_buffer,
            vertices.len(),
            gl::TRIANGLE_STRIP,
        )
    };

    let (circle_buffers, mut active_circle_buffer, circles_count) = unsafe {
        let circles = std::iter::repeat_with(|| Circle {
            position: [
                (random::<f32>() * 2.0 - 1.0) * 50.0,
                (random::<f32>() * 2.0 - 1.0) * 50.0,
            ],
            velocity: [
                (random::<f32>() * 2.0 - 1.0) * 10.0,
                (random::<f32>() * 2.0 - 1.0) * 10.0,
            ],
        })
        .take(45000)
        .collect::<Vec<_>>();

        let mut buffers = [0; 2];
        gl::GenBuffers(buffers.len() as _, buffers.as_mut_ptr());

        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, buffers[0]);
        gl::BufferData(
            gl::SHADER_STORAGE_BUFFER,
            std::mem::size_of_val(circles.as_slice()) as _,
            circles.as_ptr() as _,
            gl::DYNAMIC_DRAW,
        );
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, buffers[1]);
        gl::BufferData(
            gl::SHADER_STORAGE_BUFFER,
            std::mem::size_of_val(circles.as_slice()) as _,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        );

        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        (buffers, 0, circles.len())
    };

    let physics_shader = unsafe {
        let shader =
            compile_shader(include_str!("./physics.comp.glsl"), gl::COMPUTE_SHADER).unwrap();

        let program = gl::CreateProgram();
        gl::AttachShader(program, shader);
        gl::LinkProgram(program);
        let link_status = {
            let mut status = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
            status
        };

        gl::DetachShader(program, shader);
        gl::DeleteShader(shader);

        if link_status != 0 {
            Ok(program)
        } else {
            let mut info_log_length = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut info_log_length);
            let mut bytes = vec![0; info_log_length as _];

            let mut written_length = 0;
            gl::GetProgramInfoLog(
                program,
                info_log_length,
                &mut written_length,
                bytes.as_mut_ptr() as _,
            );

            gl::DeleteProgram(program);

            bytes.truncate(written_length as _);
            Err(String::from_utf8(bytes).unwrap())
        }
    }
    .unwrap();

    let mut camera_position = [0.0, 0.0];
    let mut camera_scale = 1.0;

    let mut last_time = std::time::Instant::now();
    while !window.should_close() {
        let time = std::time::Instant::now();
        let ts = time.duration_since(last_time).as_secs_f32();
        last_time = time;

        print!("{}ms            \r", ts * 1000.0);

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Size(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                WindowEvent::Scroll(_, y) => {
                    let y = y as f32;
                    if y < 0.0 {
                        camera_scale /= 0.9;
                    }
                    if y > 0.0 {
                        camera_scale *= 0.9;
                    }
                }
                _ => {}
            }
        }

        const CAMERA_SPEED: f32 = 1.0;
        if window.get_key(Key::W) == Action::Press {
            camera_position[1] += CAMERA_SPEED * camera_scale * ts;
        }
        if window.get_key(Key::S) == Action::Press {
            camera_position[1] -= CAMERA_SPEED * camera_scale * ts;
        }
        if window.get_key(Key::A) == Action::Press {
            camera_position[0] -= CAMERA_SPEED * camera_scale * ts;
        }
        if window.get_key(Key::D) == Action::Press {
            camera_position[0] += CAMERA_SPEED * camera_scale * ts;
        }

        // Physics
        {
            let next_active_circle_buffer = (active_circle_buffer + 1) % circle_buffers.len();

            // Run the compute shader to calculate physics, both buffers should be the right size
            unsafe {
                gl::UseProgram(physics_shader);
                gl::ProgramUniform1i(
                    physics_shader,
                    gl::GetUniformLocation(physics_shader, b"u_CircleCount\0".as_ptr() as _),
                    circles_count as _,
                );
                gl::ProgramUniform1f(
                    physics_shader,
                    gl::GetUniformLocation(physics_shader, b"u_TS\0".as_ptr() as _),
                    ts,
                );
                gl::BindBufferBase(
                    gl::SHADER_STORAGE_BUFFER,
                    0,
                    circle_buffers[active_circle_buffer],
                );
                gl::BindBufferBase(
                    gl::SHADER_STORAGE_BUFFER,
                    1,
                    circle_buffers[next_active_circle_buffer],
                );
                gl::DispatchCompute((circles_count / 64) as _, 1, 1);
                gl::MemoryBarrier(gl::ALL_BARRIER_BITS);
            }

            active_circle_buffer = next_active_circle_buffer;
        }

        // Render to the window
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(square_shader);
            let (width, height) = window.get_size();
            gl::ProgramUniform2i(
                square_shader,
                gl::GetUniformLocation(square_shader, b"u_ScreenSize\0".as_ptr() as _),
                width,
                height,
            );
            gl::ProgramUniform2f(
                square_shader,
                gl::GetUniformLocation(square_shader, b"u_CameraPosition\0".as_ptr() as _),
                camera_position[0],
                camera_position[1],
            );
            gl::ProgramUniform1f(
                square_shader,
                gl::GetUniformLocation(square_shader, b"u_CameraScale\0".as_ptr() as _),
                camera_scale,
            );
            gl::BindVertexArray(square_vertex_array);
            gl::BindBufferBase(
                gl::SHADER_STORAGE_BUFFER,
                0,
                circle_buffers[active_circle_buffer],
            );
            gl::DrawArraysInstanced(
                square_primitive_type,
                0,
                square_vertex_count as _,
                circles_count as _,
            );

            window.swap_buffers();
        }
    }

    unsafe {
        gl::DeleteProgram(physics_shader);

        gl::DeleteBuffers(circle_buffers.len() as _, circle_buffers.as_ptr());

        gl::DeleteVertexArrays(1, &square_vertex_array);
        gl::DeleteBuffers(1, &square_vertex_buffer);

        gl::DeleteProgram(square_shader);
    }
}
