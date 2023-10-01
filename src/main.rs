use bevy::{
    core_pipeline::{
        clear_color::ClearColorConfig, core_3d,
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor,
            CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, MultisampleState,
            Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment,
            RenderPassDescriptor, RenderPipelineDescriptor, TextureFormat,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
        RenderApp,
    },
};

// 1. This contains global data used by the render pipeline. This will be created once on startup.
#[derive(Resource)]
struct BasicTrianglePipeline {
    layout: BindGroupLayout,
    // sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for BasicTrianglePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        // We need to define the bind group layout used for our pipeline
        let layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("basic_triangle_bind_group_layout"),
            entries: &[
                // The screen texture
                // BindGroupLayoutEntry {
                //     binding: 0,
                //     visibility: ShaderStages::FRAGMENT,
                //     ty: BindingType::Texture {
                //         sample_type: TextureSampleType::Float { filterable: true },
                //         view_dimension: TextureViewDimension::D2,
                //         multisampled: false,
                //     },
                //     count: None,
                // },
            ],
        });

        // Get the shader handle
        let shader = world
            .resource::<AssetServer>()
            .load("shaders/basic_triangle.wgsl");

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("post_process_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following property are not important for this effect so just use the default values.
                // This struct doesn't have the Default trai implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        Self {
            layout,
            // sampler,
            pipeline_id,
        }
    }
}

// 2. The post process node used for the render graph
#[derive(Default)]
struct BasicTriangleNode;
impl BasicTriangleNode {
    pub const NAME: &'static str = "post_process";
}

// The ViewNode trait is required by the ViewNodeRunner
impl ViewNode for BasicTriangleNode {
    // The node needs a query to gather data from the ECS in order to do its rendering,
    // but it's not a normal system so we need to define it manually.
    //
    // This query will only run on the view entity
    type ViewQuery = &'static ViewTarget;

    // Runs the node logic
    // This is where you encode draw commands.
    //
    // This will run on every view on which the graph is running.
    // If you don't want your effect to run on every camera,
    // you'll need to make sure you have a marker component as part of [`ViewQuery`]
    // to identify which camera(s) should run the effect.
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        view_target: QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        // Get the pipeline resource that contains the global data we need
        // to create the render pipeline
        let post_process_pipeline = world.resource::<BasicTrianglePipeline>();

        // The pipeline cache is a cache of all previously created pipelines.
        // It is required to avoid creating a new pipeline each frame,
        // which is expensive due to shader compilation.
        let pipeline_cache = world.resource::<PipelineCache>();

        // Get the pipeline from the cache
        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        // This will start a new "post process write", obtaining two texture
        // views from the view target - a `source` and a `destination`.
        // `source` is the "current" main texture and you _must_ write into
        // `destination` because calling `post_process_write()` on the
        // [`ViewTarget`] will internally flip the [`ViewTarget`]'s main
        // texture to the `destination` texture. Failing to do so will cause
        // the current main texture information to be lost.
        let post_process = view_target.post_process_write();

        // *** TODO - Move this - not sure how?

        // The bind_group gets created each frame.
        //
        // Normally, you would create a bind_group in the Queue set,
        // but this doesn't work with the post_process_write().
        // The reason it doesn't work is because each post_process_write will alternate the source/destination.
        // The only way to have the correct source/destination for the bind_group
        // is to make sure you get it during the node execution.
        let bind_group = render_context
            .render_device()
            .create_bind_group(&BindGroupDescriptor {
                label: Some("basic_triangle_bind_group"),
                layout: &post_process_pipeline.layout,
                // It's important for this to match the BindGroupLayout defined in the PostProcessPipeline
                entries: &[
                    // BindGroupEntry {
                    // binding: 0,
                    // // Make sure to use the source view
                    // resource: BindingResource::TextureView(post_process.source),
                    // }
                ],
            });

        // Begin the render pass
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("basic_triangle_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                // We need to specify the post process destination view here
                // to make sure we write to the appropriate texture.
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
        });

        // This is mostly just wgpu boilerplate for drawing a fullscreen triangle,
        // using the pipeline/bind_group created above
        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

// 3. The plugin that adds the node to the render graph
struct BasicTrianglePlugin;

impl Plugin for BasicTrianglePlugin {
    fn build(&self, app: &mut App) {
        // We need to get the render app from the main app
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            // Bevy's renderer uses a render graph which is a collection of nodes in a directed acyclic graph.
            // It currently runs on each view/camera and executes each node in the specified order.
            // It will make sure that any node that needs a dependency from another node
            // only runs when that dependency is done.
            //
            // Each node can execute arbitrary work, but it generally runs at least one render pass.
            // A node only has access to the render world, so if you need data from the main world
            // you need to extract it manually or with the plugin like above.
            // Add a [`Node`] to the [`RenderGraph`]
            // The Node needs to impl FromWorld
            //
            // The [`ViewNodeRunner`] is a special [`Node`] that will automatically run the node for each view
            // matching the [`ViewQuery`]
            .add_render_graph_node::<ViewNodeRunner<BasicTriangleNode>>(
                // Specify the name of the graph, in this case we want the graph for 3d
                core_3d::graph::NAME,
                // It also needs the name of the node
                BasicTriangleNode::NAME,
            )
            .add_render_graph_edges(
                core_3d::graph::NAME,
                // Specify the node ordering.
                // This will automatically create all required node edges to enforce the given ordering.
                &[
                    core_3d::graph::node::TONEMAPPING,
                    BasicTriangleNode::NAME,
                    core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            );
    }

    fn finish(&self, app: &mut App) {
        // We need to get the render app from the main app
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            // Initialize the pipeline
            .init_resource::<BasicTrianglePipeline>();
    }
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
            .looking_at(Vec3::default(), Vec3::Y),
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::WHITE),
            ..default()
        },
        ..default()
    },));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BasicTrianglePlugin))
        .add_systems(Startup, setup)
        .run();
}
