
use crate::mini_client::CallResult;
use crate::mini_client::GoogleStorageClient;


pub struct GCSClient {
  raw_client: *mut GoogleStorageClient,
}

pub enum GCSClientAPI {
    GrpcDirectpath = crate::mini_client::ClientAPI_GRPC_DIRECTPATH as isize,
    GrpcNoDirectpath = crate::mini_client::ClientAPI_GRPC_NO_DIRECTPATH as isize,
    Json = crate::mini_client::ClientAPI_JSON as isize,
}

impl GCSClient {
  pub fn new(api: GCSClientAPI, project_id: String) -> Result<Self, String> {
    let project = std::ffi::CString::new(project_id).unwrap();
    let raw_client = unsafe { crate::mini_client::CreateGCSClient(api as u32, project.as_ptr()) };
    Ok(GCSClient { raw_client })
  }

  pub fn read_object(&self, bucket_id: String, object_id: String) -> CallResult {
    let bucket = std::ffi::CString::new(bucket_id).unwrap();
    let object = std::ffi::CString::new(object_id).unwrap();
    unsafe { crate::mini_client::ReadObject(self.raw_client, bucket.as_ptr(), object.as_ptr()) }
  }

  pub fn start_resumable_write(&self, bucket_id: String, object_id: String) -> CallResult {
    let bucket = std::ffi::CString::new(bucket_id).unwrap();
    let object = std::ffi::CString::new(object_id).unwrap();
    unsafe { crate::mini_client::StartResumableWrite(self.raw_client, bucket.as_ptr(), object.as_ptr()) }
  }

  pub fn query_write_status(&self, upload_id: String) -> CallResult {
    let upload_id = std::ffi::CString::new(upload_id).unwrap();
    unsafe { crate::mini_client::QueryWriteStatus(self.raw_client, upload_id.as_ptr()) }
  }
}

impl CallResult {
  pub fn error_code(&self) -> String {
    let s = match unsafe{std::ffi::CStr::from_ptr(self.error_code.as_ptr())}.to_str() {
      Ok(valid_str) => valid_str,
      Err(utf8_failure) => panic!("Failure decoding string {}", utf8_failure),
    };
    String::from(s)
  }

  pub fn upload_id(&self) -> String {
    let s = match unsafe{std::ffi::CStr::from_ptr(self.upload_id.as_ptr())}.to_str() {
      Ok(valid_str) => valid_str,
      Err(utf8_failure) => panic!("Failure decoding string {}", utf8_failure),
    };
    String::from(s)
  }
}


impl Drop for GCSClient {
  fn drop(&mut self) {
    unsafe { crate::mini_client::DestroyGCSClient(self.raw_client) }
  }
}

unsafe impl Send for GCSClient {}
unsafe impl Sync for GCSClient {}
