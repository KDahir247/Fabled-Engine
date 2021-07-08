use crate::component::{
    camera_component, light_component, model_component, render_component, skybox_component,
    window_component,
};
use crate::util::{camera, constant, texture};
use glam::Vec4Swizzles;
use shipyard::*;

pub fn begin_render_pass_system(
    setup: shipyard::UniqueView<render_component::RenderData>,
    camera: shipyard::UniqueView<camera_component::Camera>,
    lighting: shipyard::View<light_component::LightUniform>,
    depth_texture: shipyard::UniqueView<render_component::Texture>,
    model_render: shipyard::View<model_component::ModelRenderDetail>,
    skybox_render: shipyard::View<skybox_component::SkyBoxRenderDetail>,
) -> anyhow::Result<()> {
    let frame = setup.pass.swap_chain.get_current_frame()?.output;

    let mut render_command =
        setup
            .core
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Command"),
            });

    {
        let mut render_pass = render_command.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: constant::CLEAR_COLOR,
                        g: constant::CLEAR_COLOR,
                        b: constant::CLEAR_COLOR,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: false,
                }),
                stencil_ops: None,
            }),
        });

        model_render.fast_iter().for_each(|render| {
            render_pass.set_pipeline(&render.pipeline);
            if render.model.is_some() {
                for m in &render.model.as_ref().unwrap().meshes {
                    render_pass.set_bind_group(
                        0,
                        &render.model.as_ref().unwrap().materials[m.material_id].mat_group,
                        &[],
                    );

                    render_pass.set_bind_group(1, &camera.uniform.group, &[]);

                    lighting.fast_iter().for_each(|light| {
                        render_pass.set_bind_group(2, &light.group, &[]);
                    });

                    render_pass.set_vertex_buffer(0, m.vertex_buffer.slice(..));
                    render_pass
                        .set_index_buffer(m.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

                    render_pass.draw_indexed(0..m.indices, 0, 0..1);
                }
            }
        });

        skybox_render.fast_iter().for_each(|render| {
            render_pass.set_pipeline(&render.pipeline);
            render_pass.set_bind_group(0, &camera.uniform.group, &[]);
            //render_pass.draw(0..36, 0..1);
        });
    }

    setup.pass.queue.submit(Some(render_command.finish()));

    Ok(())
}

pub fn render_resize_system(
    window: shipyard::UniqueView<window_component::Window>,
    mut render: shipyard::UniqueViewMut<render_component::RenderData>,
    mut camera: shipyard::UniqueViewMut<camera_component::Camera>,
    mut depth_texture: shipyard::UniqueViewMut<render_component::Texture>,
) {
    let size = window.raw.inner_size();

    if size.width != 0 && size.height != 0 {
        render.info.swap_chain_desc.width = size.width;
        render.info.swap_chain_desc.height = size.height;

        render.pass.swap_chain = render
            .core
            .device
            .create_swap_chain(&render.setup.surface, &render.info.swap_chain_desc);

        camera.projection.aspect = size.width as f32 / size.height as f32;

        camera.uniform.raw.view_position = camera
            .orientation
            .transformation_matrix
            .w_axis
            .xyz()
            .extend(1.0);

        camera.uniform.raw.proj = camera::calc_proj_matrix(&camera.projection);
        camera.uniform.raw.view = camera::calc_camera_matrix(&camera.orientation);
        camera.uniform.raw.inv_proj = camera.uniform.raw.proj.inverse();

        *depth_texture = texture::create_depth_texture(&render.core.device, size);
    }
}
