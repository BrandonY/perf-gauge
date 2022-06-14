
use crate::mini_client::CreateGCSClient;
use crate::mini_client::CallResult;
use crate::mini_client::GoogleStorageClient;
use crate::mini_client::ReadObject;


pub struct GCSClient {
  raw_client: *mut GoogleStorageClient,
}

impl GCSClient {
  pub fn new(project_id: String) -> Result<Self, String> {
    let project = std::ffi::CString::new(project_id).unwrap();
    let raw_client = unsafe { crate::mini_client::CreateGCSClient(project.as_ptr()) };
    Ok(GCSClient { raw_client })
  }

  pub fn read_object(&self, bucket_id: String, object_id: String) -> CallResult {
    let bucket = std::ffi::CString::new(bucket_id).unwrap();
    let object = std::ffi::CString::new(object_id).unwrap();
    unsafe { crate::mini_client::ReadObject(self.raw_client, bucket.as_ptr(), object.as_ptr()) }
  }
}


impl Drop for GCSClient {
  fn drop(&mut self) {
    unsafe { crate::mini_client::DestroyGCSClient(self.raw_client) }
  }
}

unsafe impl Send for GCSClient {}
unsafe impl Sync for GCSClient {}
