use std::marker::PhantomData;

use bevy::{prelude::*, render::renderer::RenderDevice};

use crate::{
    extract_shaders, pipeline_cache::AppPipelineCache, process_pipeline_queue_system,
    traits::ComputeWorker, worker::AppComputeWorker,
};

/// The main plugin. Always include it if you want to use `bevy_app_compute`
pub struct AppComputePlugin;

impl Plugin for AppComputePlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        let render_device = app.world().resource::<RenderDevice>().clone();

        app.insert_resource(AppPipelineCache::new(render_device))
            .add_systems(PreUpdate, extract_shaders)
            .add_systems(Update, process_pipeline_queue_system);
    }
}

/// Plugin to initialise your [`AppComputeWorker<W>`] structs.
pub struct AppComputeWorkerPlugin<W: ComputeWorker> {
    _phantom: PhantomData<W>,
}

impl<W: ComputeWorker> Default for AppComputeWorkerPlugin<W> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<W: ComputeWorker> Plugin for AppComputeWorkerPlugin<W> {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        // NOTE: It is up to YOU to call W::build(&mut world) somewhere in your code.

        app.add_systems(
            Update,
            AppComputeWorker::<W>::extract_pipelines.run_if(resource_exists::<AppComputeWorker<W>>),
        )
        .add_systems(
            PostUpdate,
            (AppComputeWorker::<W>::unmap_all, AppComputeWorker::<W>::run)
                .run_if(resource_exists::<AppComputeWorker<W>>)
                .chain(),
        );
    }
}
