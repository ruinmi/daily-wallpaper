use crate::wallpaper::Wallpaper;
use reqwest;
use reqwest::Client;
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path};
use std::process::Command;
use std::string::ToString;
use tauri::path::BaseDirectory;
use tauri::{Manager};

#[tauri::command]
pub fn open_file_in_folder(path: &str) {
    let base_dir = BaseDirectory::Public.variable();
    let absolute_path = Path::new(&base_dir).join(path.strip_prefix('/').unwrap_or(path));
    let sanitized_path = absolute_path
        .to_str()
        .expect("Failed to convert path to string")
        .replace("/", "\\");
    Command::new("explorer")
        .args(&["/select,", &sanitized_path])
        .spawn()
        .expect("Failed to open folder and select file");
}

#[tauri::command]
pub fn open_folder(path: &str) {
    let base_dir = BaseDirectory::Public.variable();
    let absolute_path = Path::new(&base_dir).join(path.strip_prefix('/').unwrap_or(path));
    let sanitized_path = absolute_path
        .to_str()
        .expect("Failed to convert path to string")
        .replace("/", "\\");
    Command::new("explorer")
        .arg(sanitized_path)
        .spawn()
        .expect("Failed to open folder");
}

#[tauri::command]
pub fn set_wallpaper_command(path: &str) -> Result<String, String> {
    match wallpaper::set_from_path(path) {
        Ok(_) => {
            Ok("Wallpaper set successfully.".to_string())
        },
        Err(err) => Err(err.to_string()),
    }
}

const URL_PREFIX: &str = "https://www.pexels.com/en-us/api/v3/search/photos";
#[tauri::command]
pub async fn fetch_wallpaper(
    app: tauri::AppHandle,
    saved_page: u32,
    page: u32,
    per_page: u32,
    keyword: String,
    orientation: String,
    color: String,
) -> Result<Vec<Wallpaper>, String> {
    let client = Client::new();
    let url = format!(
        "{URL_PREFIX}?page={}&per_page={}&query={}&orientation={}&size=all&color={}&seo_tags=true",
        page, per_page, keyword, orientation, color
    );

    println!("Fetching {per_page} wallpapers from URL: {url}");

    let response = client
        .get(url)
        .header("Secret-Key", "H2jk9uKnhRmL6WPwh89zBezWvr")
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let json_value: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;
    let data_json = json_value["data"].to_string();
    let mut wallpapers: Vec<Wallpaper> = serde_json::from_str(&data_json)
        .map_err(|e| format!("Failed to deserialize wallpapers: {} -- {}", e, data_json))?;

    let app_data_dir = app
        .path()
        .app_data_dir()
        .unwrap();

    let wallpaper_dir = app_data_dir.join("wallpapers").join(&keyword).join(saved_page.to_string());
    if !wallpaper_dir.exists() {
        fs::create_dir_all(&wallpaper_dir)
            .map_err(|e| format!("Failed to create wallpaper directory: {}", e))?;
    }

    let mut created_files = Vec::new(); // 记录已成功创建的文件路径
    let mut retained_wallpapers = Vec::new(); // 保存需要处理的壁纸
    for wallpaper in wallpapers.iter_mut() {
        let sanitized_title = wallpaper
            .attributes
            .title
            .replace(|c: char| c.is_control(), ""); // 去除控制字符
        let file_path = wallpaper_dir.join(format!("{}.jpg", sanitized_title));

        if !file_path.exists() {
            // 文件不存在，保留壁纸并设置路径
            wallpaper.attributes.image.local_file_path = file_path.to_string_lossy().to_string();
            retained_wallpapers.push(wallpaper.clone());
        }
    }
    // 下载和保存保留的壁纸
    for wallpaper in retained_wallpapers.iter_mut() {
        let img_link = wallpaper.attributes.image.download_link.as_str();
        let file_path = Path::new(&wallpaper.attributes.image.local_file_path);

        if let Err(e) = download_and_save_image(&client, img_link, file_path).await {
            // 下载失败时，删除已生成的文件
            for created_file in &created_files {
                if fs::remove_file(created_file).is_err() {
                    eprintln!("Failed to remove file: {:?}", created_file);
                }
            }
            return Err(format!(
                "Failed to download image: {}. All created files have been removed.",
                e
            ));
        }

        // 成功下载后记录文件路径
        created_files.push(file_path.to_path_buf());
    }

    Ok(retained_wallpapers)
}

async fn download_and_save_image(
    client: &Client,
    url: &str,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch image: {}", e))?;
    let content = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read image content: {}", e))?;

    let mut file = File::create(output_path)
        .map_err(|e| format!("Failed to create file: {} -- {}", e, output_path.to_string_lossy()))?;
    file.write_all(&content)
        .map_err(|e| format!("Failed to write image content: {}", e))?;

    Ok(())
}