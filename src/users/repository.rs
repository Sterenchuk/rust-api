// db.rs

use crate::users::model::User;
use crate::helpers::errors::AppError;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const FILE_PATH: &str = "./data/store.json";

fn ensure_data_folder() -> Result<(), AppError> {
    std::fs::create_dir_all("./data")?;
    Ok(())
}

fn read_users() -> Result<Vec<User>, AppError> {
    ensure_data_folder()?;

    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(FILE_PATH)?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;

    if data.trim().is_empty() {
        Ok(Vec::new())
    } else {
        Ok(serde_json::from_str(&data)?)
    }
}

fn write_users(users: &[User]) -> Result<(), AppError> {
    ensure_data_folder()?;

    let data = serde_json::to_string_pretty(users)?;
    let mut file = File::create(FILE_PATH)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn get_all_users() -> Result<Vec<User>, AppError> {
    read_users()
}

pub fn get_user(id: &str) -> Result<User, AppError> {
    let users = read_users()?;
    users
        .into_iter()
        .find(|u| u.id.to_string() == id)
        .ok_or(AppError::NotFound)
}

pub fn create_user(user: User) -> Result<User, AppError_
