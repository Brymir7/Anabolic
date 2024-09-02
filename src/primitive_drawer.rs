pub mod primitive_drawer {
    use std::collections::BTreeMap;
    use glam::{ vec2, vec3, vec4, Mat4, Vec2, Vec3, Vec4 };
    use miniquad::{
        Backend,
        Bindings,
        BlendFactor,
        BlendState,
        BlendValue,
        BufferLayout,
        BufferSource,
        BufferType,
        BufferUsage,
        Comparison,
        Context,
        Equation,
        PipelineParams,
        PrimitiveType,
        RenderingBackend,
        ShaderId,
        ShaderSource,
        TextureId as MiniquadTexture,
        UniformDesc,
        UniformType,
        VertexAttribute,
        VertexFormat,
    };
    mod shader {
        use miniquad::*;
        pub const VERTEX: &str =
            r#"#version 100
        attribute vec3 pos;
        attribute vec2 texcoord;
        attribute vec4 color0;
        uniform mat4 mvp;
        varying lowp vec4 color;
        void main() {
            gl_Position = mvp * vec4(pos, 1.0);
            color = color0;
        }"#;

        pub const FRAGMENT: &str =
            r#"#version 100
        varying lowp vec4 color;
        void main() {
            gl_FragColor = color;
        }
        "#;

        pub fn meta() -> ShaderMeta {
            ShaderMeta {
                images: vec![],
                uniforms: UniformBlockLayout {
                    uniforms: uniforms()
                        .into_iter()
                        .map(|(name, kind)| UniformDesc::new(name, kind))
                        .collect(),
                },
            }
        }
        pub fn uniforms() -> Vec<(&'static str, UniformType)> {
            vec![("mvp", UniformType::Mat4)]
        }
    }
    #[derive(Clone)]
    struct PipelineExt {
        pipeline: miniquad::Pipeline,
        uniforms: Vec<Uniform>,
        uniforms_data: Vec<u8>,
        textures: Vec<String>,
        textures_data: BTreeMap<String, MiniquadTexture>,
    }

    impl PipelineExt {
        fn set_uniform<T>(&mut self, name: &str, uniform: T) {
            let uniform_meta = self.uniforms
                .iter()
                .find(|Uniform { name: uniform_name, .. }| uniform_name == name);
            if uniform_meta.is_none() {
                panic!("Trying to set non-existing uniform: {}", name);
            }
            let uniform_meta = uniform_meta.unwrap();
            let uniform_format = uniform_meta.uniform_type;
            let uniform_byte_size = uniform_format.size();
            let uniform_byte_offset = uniform_meta.byte_offset;

            if std::mem::size_of::<T>() != uniform_byte_size {
                panic!(
                    "Trying to set uniform {} sized {} bytes value of {} bytes",
                    name,
                    uniform_byte_size,
                    std::mem::size_of::<T>()
                );
            }
            if uniform_byte_size != uniform_meta.byte_size {
                panic!("set_uniform do not support uniform arrays");
            }
            macro_rules! transmute_uniform {
                ($uniform_size:expr, $byte_offset:expr, $n:expr) => {
                    if $uniform_size == $n {
                            let data: [u8; $n] = unsafe { std::mem::transmute_copy(&uniform) };

                        for i in 0..$uniform_size {
                            self.uniforms_data[$byte_offset + i] = data[i];
                        }
                    }
                };
            }
            transmute_uniform!(uniform_byte_size, uniform_byte_offset, 4);
            transmute_uniform!(uniform_byte_size, uniform_byte_offset, 8);
            transmute_uniform!(uniform_byte_size, uniform_byte_offset, 12);
            transmute_uniform!(uniform_byte_size, uniform_byte_offset, 16);
            transmute_uniform!(uniform_byte_size, uniform_byte_offset, 64);
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum DrawMode {
        Triangles,
        Lines,
    }

    struct PipelinesStorage {
        pipelines: [Option<PipelineExt>; Self::MAX_PIPELINES],
        pipelines_amount: usize,
    }
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GlPipeline(usize);
    #[derive(Clone, Debug)]
    struct Uniform {
        name: String,
        uniform_type: UniformType,
        byte_offset: usize,
        byte_size: usize,
    }
    impl PipelinesStorage {
        const MAX_PIPELINES: usize = 32;
        const TRIANGLES_PIPELINE: GlPipeline = GlPipeline(0);
        const LINES_PIPELINE: GlPipeline = GlPipeline(1);
        const TRIANGLES_DEPTH_PIPELINE: GlPipeline = GlPipeline(2);
        const LINES_DEPTH_PIPELINE: GlPipeline = GlPipeline(3);

        fn new(ctx: &mut dyn RenderingBackend) -> PipelinesStorage {
            let shader = ctx
                .new_shader(
                    match ctx.info().backend {
                        Backend::OpenGl =>
                            ShaderSource::Glsl {
                                vertex: shader::VERTEX,
                                fragment: shader::FRAGMENT,
                            },
                        Backend::Metal => panic!("Metal currently not supported"),
                    },
                    shader::meta()
                )
                .unwrap_or_else(|e| panic!("Failed to load shader: {}", e));

            let params = PipelineParams {
                color_blend: Some(
                    BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
                    )
                ),
                ..Default::default()
            };

            let mut storage = PipelinesStorage {
                pipelines: Default::default(),
                pipelines_amount: 0,
            };

            let triangles_pipeline = storage.make_pipeline(
                ctx,
                shader,
                PipelineParams {
                    primitive_type: PrimitiveType::Triangles,
                    ..params
                },
                vec![],
                vec![]
            );
            assert_eq!(triangles_pipeline, Self::TRIANGLES_PIPELINE);

            let lines_pipeline = storage.make_pipeline(
                ctx,
                shader,
                PipelineParams {
                    primitive_type: PrimitiveType::Lines,
                    ..params
                },
                vec![],
                vec![]
            );
            assert_eq!(lines_pipeline, Self::LINES_PIPELINE);

            let triangles_depth_pipeline = storage.make_pipeline(
                ctx,
                shader,
                PipelineParams {
                    depth_write: true,
                    depth_test: Comparison::LessOrEqual,
                    primitive_type: PrimitiveType::Triangles,
                    ..params
                },
                vec![],
                vec![]
            );
            assert_eq!(triangles_depth_pipeline, Self::TRIANGLES_DEPTH_PIPELINE);

            let lines_depth_pipeline = storage.make_pipeline(
                ctx,
                shader,
                PipelineParams {
                    depth_write: true,
                    depth_test: Comparison::LessOrEqual,
                    primitive_type: PrimitiveType::Lines,
                    ..params
                },
                vec![],
                vec![]
            );
            assert_eq!(lines_depth_pipeline, Self::LINES_DEPTH_PIPELINE);

            storage
        }

        fn make_pipeline(
            &mut self,
            ctx: &mut dyn RenderingBackend,
            shader: ShaderId,
            params: PipelineParams,
            mut uniforms: Vec<UniformDesc>,
            textures: Vec<String>
        ) -> GlPipeline {
            let pipeline = ctx.new_pipeline(
                &[BufferLayout::default()],
                &[
                    VertexAttribute::new("pos", VertexFormat::Float3),
                    VertexAttribute::new("texcoord", VertexFormat::Float2),
                    VertexAttribute::new("color0", VertexFormat::Float4),
                ],
                shader,
                params
            );

            let id = self.pipelines
                .iter()
                .position(|p| p.is_none())
                .unwrap_or_else(|| panic!("Pipelines amount exceeded"));

            let mut max_offset = 0;

            for (name, kind) in shader::uniforms().into_iter().rev() {
                uniforms.insert(0, UniformDesc::new(name, kind));
            }

            let uniforms = uniforms
                .iter()
                .scan(0, |offset, uniform| {
                    let byte_size = uniform.uniform_type.size() * uniform.array_count;
                    let uniform = Uniform {
                        name: uniform.name.clone(),
                        uniform_type: uniform.uniform_type,
                        byte_size,
                        byte_offset: *offset,
                    };
                    *offset += byte_size;
                    max_offset = *offset;

                    Some(uniform)
                })
                .collect();

            self.pipelines[id] = Some(PipelineExt {
                pipeline,
                uniforms,
                uniforms_data: vec![0; max_offset],
                textures,
                textures_data: BTreeMap::new(),
            });
            self.pipelines_amount += 1;
            GlPipeline(id)
        }

        fn get(&self, draw_mode: DrawMode, depth_enabled: bool) -> GlPipeline {
            match (draw_mode, depth_enabled) {
                (DrawMode::Triangles, false) => Self::TRIANGLES_PIPELINE,
                (DrawMode::Triangles, true) => Self::TRIANGLES_DEPTH_PIPELINE,
                (DrawMode::Lines, false) => Self::LINES_PIPELINE,
                (DrawMode::Lines, true) => Self::LINES_DEPTH_PIPELINE,
            }
        }

        fn get_quad_mut_pipeline(&mut self, pipe: GlPipeline) -> &mut PipelineExt {
            self.pipelines[pipe.0].as_mut().unwrap()
        }
    }
    pub struct GlBindings(usize);
    #[repr(C)]
    pub struct Vertex3D {
        pos: Vec3,
        texcoord: Vec2,
        color: Vec4,
    }
    pub struct PrimitiveDrawer {
        pipelines: PipelinesStorage,
        cube_bindings: GlBindings,
        bindings: [Option<Bindings>; 32],
    }

    impl PrimitiveDrawer {
        pub fn default(ctx: &mut Context) -> Self {
            const ARRAY_REPEAT_VALUE: Option<Bindings> = None;
            let mut p_drawer = PrimitiveDrawer {
                pipelines: PipelinesStorage::new(ctx),
                cube_bindings: GlBindings(0),
                bindings: [ARRAY_REPEAT_VALUE; 32],
            };
            #[rustfmt::skip]
            let vertices: [Vertex3D; 8] = [
                Vertex3D { pos: Vec3::new(-1.0, -1.0,  1.0), texcoord: Vec2::new(0.0, 0.0), color: Vec4::new(1.0, 0.0, 0.0, 1.0) },
                Vertex3D { pos: Vec3::new( 1.0, -1.0,  1.0), texcoord: Vec2::new(1.0, 0.0), color: Vec4::new(0.0, 1.0, 0.0, 1.0) },
                Vertex3D { pos: Vec3::new( 1.0,  1.0,  1.0), texcoord: Vec2::new(1.0, 1.0), color: Vec4::new(0.0, 0.0, 1.0, 1.0) },
                Vertex3D { pos: Vec3::new(-1.0,  1.0,  1.0), texcoord: Vec2::new(0.0, 1.0), color: Vec4::new(1.0, 1.0, 0.0, 1.0) },
                Vertex3D { pos: Vec3::new(-1.0, -1.0, -1.0), texcoord: Vec2::new(0.0, 0.0), color: Vec4::new(1.0, 0.0, 1.0, 1.0) },
                Vertex3D { pos: Vec3::new( 1.0, -1.0, -1.0), texcoord: Vec2::new(1.0, 0.0), color: Vec4::new(0.0, 1.0, 1.0, 1.0) },
                Vertex3D { pos: Vec3::new( 1.0,  1.0, -1.0), texcoord: Vec2::new(1.0, 1.0), color: Vec4::new(1.0, 0.5, 0.0, 1.0) },
                Vertex3D { pos: Vec3::new(-1.0,  1.0, -1.0), texcoord: Vec2::new(0.0, 1.0), color: Vec4::new(0.5, 0.0, 0.5, 1.0) },
            ];
            let vertex_buffer = ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(&vertices)
            );
            #[rustfmt::skip]
            let indices: [u16; 36] = [
                0, 1, 2, 2, 3, 0, // Front face
                4, 5, 6, 6, 7, 4, // Back face
                0, 3, 7, 7, 4, 0, // Left face
                1, 5, 6, 6, 2, 1, // Right face
                3, 2, 6, 6, 7, 3, // Top face
                0, 1, 5, 5, 4, 0, // Bottom face
            ];
            let index_buffer = ctx.new_buffer(
                BufferType::IndexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(&indices)
            );
            let bindings = Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer: index_buffer,
                images: vec![],
            };
            p_drawer.insert_binding(bindings);
            p_drawer
        }

        fn insert_binding(&mut self, binding: Bindings) {
            if let Some(index) = self.bindings.iter().position(|p| p.is_none()) {
                self.bindings[index] = Some(binding);
            } else {
                eprintln!("Invalid bindings: No available slot.");
            }
        }
        pub fn draw_cube(&mut self, x: f32, y: f32, z: f32, size: f32, ctx: &mut Context) {
            let pipe_id = self.pipelines.get(DrawMode::Triangles, true);
            let pipeline = self.pipelines.get_quad_mut_pipeline(pipe_id);
            ctx.apply_pipeline(&pipeline.pipeline);
            ctx.apply_bindings(
                &self.bindings[self.cube_bindings.0].as_ref().expect("Invalid cube bindings")
            );
            let mvp = Mat4::from_translation(vec3(x, y, z)) * Mat4::from_scale(Vec3::splat(0.5));
            pipeline.set_uniform("mvp", mvp);
            ctx.apply_uniforms_from_bytes(
                pipeline.uniforms_data.as_ptr(),
                pipeline.uniforms_data.len()
            );
            ctx.draw(0, 36, 1);
        }
    }
}
