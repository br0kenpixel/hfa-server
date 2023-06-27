use crate::{
    config::Method,
    responses::{FileMeta, FileType},
    CLI, CONFIG,
};
use bytes::BytesMut;
use rocket::{data::ToByteUnit, fs::NamedFile, get, http::Status, put, serde::json::Json, Data};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tokio::io::{AsyncReadExt, AsyncSeekExt};

#[get("/")]
pub fn index() -> Result<String, Status> {
    if !CONFIG.get().unwrap().allow_index {
        return Err(Status::NotFound);
    }

    Ok(format!("hfa-server v{}", env!("CARGO_PKG_VERSION")))
}

#[get("/list/<path..>")]
pub fn list(path: PathBuf) -> Result<Json<Vec<FileMeta>>, Status> {
    let config = CONFIG.get().unwrap();
    let cli = CLI.get().unwrap();

    if config.blocked_methods.contains(&Method::List) {
        return Err(Status::MethodNotAllowed);
    }

    let path = Path::new(&cli.dir).join(path);
    if !path.exists() {
        return Err(Status::NotFound);
    }
    if !path.is_dir() {
        return Err(Status::BadRequest);
    }

    let entries = fs::read_dir(path)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            let kind: FileType = entry.file_type().unwrap().into();
            let meta = entry.metadata().unwrap();
            let name = entry.file_name().into_string().unwrap();
            let modified = meta.modified().unwrap();
            let accessed = meta.accessed().unwrap();
            let created = meta.created().unwrap();

            let modified = modified
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let accessed = accessed
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let created = created
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            FileMeta {
                name,
                kind,
                size: meta.len(),
                modified,
                accessed,
                created,
            }
        })
        .collect::<Vec<_>>();

    Ok(Json(entries))
}

#[get("/get/<path..>")]
pub async fn download(path: PathBuf) -> Result<NamedFile, Status> {
    let config = CONFIG.get().unwrap();
    let cli = CLI.get().unwrap();

    if config.blocked_methods.contains(&Method::Get) {
        return Err(Status::MethodNotAllowed);
    }

    let path = Path::new(&cli.dir).join(path);
    if !path.exists() {
        return Err(Status::NotFound);
    }

    NamedFile::open(path)
        .await
        .map_err(|_| Status::InternalServerError)
}

#[put("/put/<path..>", data = "<data>")]
pub async fn upload(path: PathBuf, data: Data<'_>) -> Status {
    let config = CONFIG.get().unwrap();
    let cli = CLI.get().unwrap();

    if config.blocked_methods.contains(&Method::Put) {
        return Status::MethodNotAllowed;
    }
    let stream = data.open(config.max_upload_size.bytes());

    stream
        .into_file(Path::new(&cli.dir).join(path))
        .await
        .map_err(|_| Status::InternalServerError)
        .unwrap();
    Status::Ok
}

#[get("/getpart/<start>/<amount>/<path..>")]
pub async fn download_part(path: PathBuf, start: u64, amount: u64) -> Result<Vec<u8>, Status> {
    let config = CONFIG.get().unwrap();
    let cli = CLI.get().unwrap();

    if config.blocked_methods.contains(&Method::Get) {
        return Err(Status::MethodNotAllowed);
    }

    let path = Path::new(&cli.dir).join(path);
    if !path.exists() {
        return Err(Status::NotFound);
    }

    let meta = fs::metadata(&path).unwrap();
    let size = meta.len();
    if amount == 0 {
        return Err(Status::BadRequest);
    }
    if start > size {
        return Err(Status::RangeNotSatisfiable);
    }
    if start + amount > size {
        return Err(Status::RangeNotSatisfiable);
    }

    let mut file = NamedFile::open(path)
        .await
        .map_err(|_| Status::InternalServerError)?;

    file.seek(std::io::SeekFrom::Start(start))
        .await
        .map_err(|_| Status::InternalServerError)?;

    let mut buf = BytesMut::with_capacity(amount as usize);
    file.read_buf(&mut buf)
        .await
        .map_err(|_| Status::InternalServerError)?;
    let buf = buf.to_vec();
    println!("buf: {:?}", buf);

    Ok(buf)
}
