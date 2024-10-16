use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Jobs(pub Vec<Job>);

pub struct Job {
    pub name: String,
    pub job_type: JobType,
}

#[derive(Clone)]
pub enum JobType {
    PlantHarvest(Entity),
}