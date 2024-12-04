use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::io::{Read, Write};
use std::path::Path;
use std::{fs, io};

#[napi]
pub enum CompressionMethod {
  Stored,
  Deflated,
  Deflate64,
  Bzip2,
  Aes,
  Zstd,
  Lzma,
  Xz,
}

impl From<CompressionMethod> for zip::CompressionMethod {
  fn from(value: CompressionMethod) -> Self {
    match value {
      CompressionMethod::Stored => zip::CompressionMethod::Stored,
      CompressionMethod::Deflated => zip::CompressionMethod::Deflated,
      CompressionMethod::Deflate64 => zip::CompressionMethod::Deflate64,
      CompressionMethod::Bzip2 => zip::CompressionMethod::Bzip2,
      CompressionMethod::Aes => zip::CompressionMethod::Aes,
      CompressionMethod::Zstd => zip::CompressionMethod::Zstd,
      CompressionMethod::Lzma => zip::CompressionMethod::Lzma,
      CompressionMethod::Xz => zip::CompressionMethod::Xz,
    }
  }
}

#[napi(object)]
#[derive(Clone)]
pub struct WriteFileOptions {
  pub compression_method: Option<CompressionMethod>,
  pub compression_level: Option<i64>,
  pub permissions: Option<u32>,
  pub large_file: Option<bool>,
}

impl From<WriteFileOptions> for zip::write::SimpleFileOptions {
  fn from(value: WriteFileOptions) -> Self {
    let mut options =
      zip::write::SimpleFileOptions::default().compression_level(value.compression_level);
    if let Some(compression_method) = value.compression_method {
      options = options.compression_method(zip::CompressionMethod::from(compression_method));
    }
    if let Some(permissions) = value.permissions {
      options = options.unix_permissions(permissions);
    }
    if let Some(large_file) = value.large_file {
      options = options.large_file(large_file);
    }
    options
  }
}

fn zip_dir<T>(
  it: &mut dyn Iterator<Item = walkdir::DirEntry>,
  prefix: &Path,
  writer: T,
  options: Option<WriteFileOptions>,
) -> crate::Result<()>
where
  T: Write + io::Seek,
{
  let mut zip = zip::ZipWriter::new(writer);
  let prefix = Path::new(prefix);
  let mut buffer = Vec::new();
  let options = options
    .map(zip::write::SimpleFileOptions::from)
    .unwrap_or_default();
  for entry in it {
    let path = entry.path();
    let name = path.strip_prefix(prefix).unwrap();
    let path_as_string = name.to_str().map(str::to_owned).unwrap();
    if path.is_file() {
      zip.start_file(path_as_string, options)?;
      let mut f = fs::File::open(path)?;

      f.read_to_end(&mut buffer)?;
      zip.write_all(&buffer)?;
      buffer.clear();
    } else if !name.as_os_str().is_empty() {
      zip.add_directory(path_as_string, options)?;
    }
  }
  zip.finish()?;
  Ok(())
}

pub struct WriteTask {
  src_dir: String,
  dst: String,
  options: Option<WriteFileOptions>,
}

#[napi]
impl Task for WriteTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    let src_path = Path::new(&self.src_dir);
    if !src_path.is_dir() {
      return Err(crate::Error::Zip(zip::result::ZipError::FileNotFound).into());
    }
    let dst_filepath = Path::new(&self.dst);
    let dst_file = fs::File::create(dst_filepath)?;

    let walkdir = walkdir::WalkDir::new(src_path);
    let it = walkdir.into_iter();

    zip_dir(
      &mut it.filter_map(|e| e.ok()),
      src_path,
      dst_file,
      self.options.clone(),
    )?;
    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

#[napi]
pub fn write(src_dir: String, dst: String, options: Option<WriteFileOptions>) -> crate::Result<()> {
  let src_path = Path::new(&src_dir);
  if !src_path.is_dir() {
    return Err(zip::result::ZipError::FileNotFound.into());
  }
  let dst_filepath = Path::new(&dst);
  let dst_file = fs::File::create(dst_filepath)?;

  let walkdir = walkdir::WalkDir::new(src_path);
  let it = walkdir.into_iter();

  zip_dir(&mut it.filter_map(|e| e.ok()), src_path, dst_file, options)?;

  Ok(())
}

#[napi]
pub fn write_async(
  src_dir: String,
  dst: String,
  options: Option<WriteFileOptions>,
  signal: Option<AbortSignal>,
) -> AsyncTask<WriteTask> {
  AsyncTask::with_optional_signal(
    WriteTask {
      src_dir,
      dst,
      options,
    },
    signal,
  )
}
