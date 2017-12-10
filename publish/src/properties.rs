use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct Progress {
    boot: f32,
    initialization: f32,
    launch: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    id: Uuid,
    name: String,
    version: String,
    pointer_lock: bool,
    pigeoneer: bool,
    progress: Progress,
}
