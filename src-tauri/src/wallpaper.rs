use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Wallpaper {
    pub attributes: Attributes,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Attributes {
    pub description: String,
    pub id: u32,
    pub title: String,
    pub image: Image,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Image {
    pub download_link: String,
    #[serde(skip_deserializing)]
    pub local_file_path: String,
}
