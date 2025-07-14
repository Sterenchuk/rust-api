//db
use crate::users::model::User;

//helpers
use crate::helpers::errors::AppError;

//std
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const FILE_PATH: &str = "./data/store.json";

fn read_users() -> Result<Vec<User>, AppError> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(FILE_PATH)?;

    let mut data = String::new();

    file.read_to_string(&mut data)?;

    // TEST this for errors
    if data.trim().is_empty() {
        Ok(Vec::new())
    } else {
        Ok(serde_json::from_str(&data)?)
    }
}

fn write_users(users: &[User]) -> Result<(), AppError> {
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

pub fn create_user(user: User) -> Result<User, AppError> {
    let mut users = read_users()?;
    users.push(user.clone());
    write_users(&users)?;
    Ok(user)
}

pub fn update_user(id: &str, updated_user: User) -> Result<User, AppError> {
    let mut users = read_users()?;
    let pos = users
        .iter()
        .position(|u| u.id.to_string() == id)
        .ok_or(AppError::NotFound)?;
    users[pos] = updated_user.clone();
    write_users(&users)?;
    Ok(updated_user)
}

pub fn delete_user(id: &str) -> Result<(), AppError> {
    let mut users = read_users()?;
    let len_before = users.len();
    users.retain(|u| u.id.to_string() != id);
    if users.len() == len_before {
        return Err(AppError::NotFound);
    }
    write_users(&users)?;
    Ok(())
}
