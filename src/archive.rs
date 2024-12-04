use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::sync::RwLock;

#[napi]
pub struct Archive {
  pub(crate) inner: zip::ZipArchive<fs::File>,
}

pub struct ExtractTask {
  archive: RwLock<Reference<Archive>>,
  outdir: String,
}

#[napi]
impl Task for ExtractTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    let mut archive = self
      .archive
      .write()
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    archive
      .inner
      .extract(&self.outdir)
      .map_err(crate::Error::from)?;
    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

pub struct ReadFileTask {
  archive: RwLock<Reference<Archive>>,
  name: String,
}

#[napi]
impl Task for ReadFileTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let mut archive = self
      .archive
      .write()
      .map_err(|e| Error::new(Status::GenericFailure, format!("{e}")))?;
    let mut file = archive
      .inner
      .by_name(&self.name)
      .map_err(crate::Error::from)?;
    if !file.is_file() {
      return Err(crate::Error::Zip(zip::result::ZipError::FileNotFound).into());
    }
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(Buffer::from(output))
  }
}

#[napi]
impl Archive {
  #[napi]
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[napi]
  pub fn read_file(&mut self, name: String) -> crate::Result<Buffer> {
    let mut file = self.inner.by_name(&name)?;
    if !file.is_file() {
      return Err(zip::result::ZipError::FileNotFound.into());
    }
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(Buffer::from(data))
  }

  #[napi]
  pub fn read_file_async(
    &self,
    self_ref: Reference<Archive>,
    name: String,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<ReadFileTask> {
    AsyncTask::with_optional_signal(
      ReadFileTask {
        archive: RwLock::new(self_ref),
        name,
      },
      signal,
    )
  }

  #[napi]
  pub fn extract(&mut self, outdir: String) -> crate::Result<()> {
    self.inner.extract(&outdir)?;
    Ok(())
  }

  #[napi]
  pub fn extract_async(
    &self,
    self_ref: Reference<Archive>,
    outdir: String,
    signal: Option<AbortSignal>,
  ) -> AsyncTask<ExtractTask> {
    AsyncTask::with_optional_signal(
      ExtractTask {
        archive: RwLock::new(self_ref),
        outdir,
      },
      signal,
    )
  }

  #[napi]
  pub fn file_names(&self) -> Vec<String> {
    self
      .inner
      .file_names()
      .map(|x| x.to_string())
      .collect::<Vec<_>>()
  }
}

pub struct OpenArchiveTask {
  path: String,
}

#[napi]
impl Task for OpenArchiveTask {
  type Output = Archive;
  type JsValue = Archive;

  fn compute(&mut self) -> Result<Self::Output> {
    let filepath = Path::new(&self.path);
    let file = fs::File::open(filepath)?;
    let inner = zip::ZipArchive::new(file).map_err(crate::Error::from)?;
    Ok(Archive { inner })
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn open_archive(path: String) -> crate::Result<Archive> {
  let filepath = Path::new(&path);
  let file = fs::File::open(filepath)?;
  let inner = zip::ZipArchive::new(file)?;
  Ok(Archive { inner })
}

#[napi]
pub fn open_archive_async(path: String, signal: Option<AbortSignal>) -> AsyncTask<OpenArchiveTask> {
  AsyncTask::with_optional_signal(OpenArchiveTask { path }, signal)
}
