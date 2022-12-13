/* automatically generated by rust-bindgen 0.60.1 */

pub const ClientAPI_GRPC_DIRECTPATH: ClientAPI = 0;
pub const ClientAPI_GRPC_NO_DIRECTPATH: ClientAPI = 1;
pub const ClientAPI_JSON: ClientAPI = 2;
pub type ClientAPI = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GoogleStorageClient {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CallResult {
    pub success: bool,
    pub bytes_received: ::std::os::raw::c_ulong,
    pub error_code: [::std::os::raw::c_char; 25usize],
    pub upload_id: [::std::os::raw::c_char; 1024usize],
    pub host_ip: [::std::os::raw::c_char; 255usize],
    pub bitpusher_cell: [::std::os::raw::c_char; 32usize],
    pub bitpusher_task_no: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CallResult() {
    assert_eq!(
        ::std::mem::size_of::<CallResult>(),
        1360usize,
        concat!("Size of: ", stringify!(CallResult))
    );
    assert_eq!(
        ::std::mem::align_of::<CallResult>(),
        8usize,
        concat!("Alignment of ", stringify!(CallResult))
    );
    fn test_field_success() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<CallResult>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).success) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(CallResult),
                "::",
                stringify!(success)
            )
        );
    }
    test_field_success();
    fn test_field_bytes_received() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<CallResult>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bytes_received) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(CallResult),
                "::",
                stringify!(bytes_received)
            )
        );
    }
    test_field_bytes_received();
    fn test_field_error_code() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<CallResult>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).error_code) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(CallResult),
                "::",
                stringify!(error_code)
            )
        );
    }
    test_field_error_code();
    fn test_field_upload_id() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<CallResult>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).upload_id) as usize - ptr as usize
            },
            41usize,
            concat!(
                "Offset of field: ",
                stringify!(CallResult),
                "::",
                stringify!(upload_id)
            )
        );
    }
    test_field_upload_id();
    fn test_field_host_ip() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<CallResult>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).host_ip) as usize - ptr as usize
            },
            1065usize,
            concat!(
                "Offset of field: ",
                stringify!(CallResult),
                "::",
                stringify!(host_ip)
            )
        );
    }
    test_field_host_ip();
    fn test_field_bitpusher_cell() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<CallResult>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bitpusher_cell) as usize - ptr as usize
            },
            1320usize,
            concat!(
                "Offset of field: ",
                stringify!(CallResult),
                "::",
                stringify!(bitpusher_cell)
            )
        );
    }
    test_field_bitpusher_cell();
    fn test_field_bitpusher_task_no() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<CallResult>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bitpusher_task_no) as usize - ptr as usize
            },
            1352usize,
            concat!(
                "Offset of field: ",
                stringify!(CallResult),
                "::",
                stringify!(bitpusher_task_no)
            )
        );
    }
    test_field_bitpusher_task_no();
}
extern "C" {
    pub fn CreateGCSClient(
        client_api: ClientAPI,
        project: *const ::std::os::raw::c_char,
        universe: *const ::std::os::raw::c_char,
    ) -> *mut GoogleStorageClient;
}
extern "C" {
    pub fn DestroyGCSClient(client: *mut GoogleStorageClient);
}
extern "C" {
    pub fn ReadObject(
        client: *mut GoogleStorageClient,
        bucket: *const ::std::os::raw::c_char,
        obj: *const ::std::os::raw::c_char,
    ) -> CallResult;
}
extern "C" {
    pub fn StartResumableWrite(
        client: *mut GoogleStorageClient,
        bucket: *const ::std::os::raw::c_char,
        obj: *const ::std::os::raw::c_char,
    ) -> CallResult;
}
extern "C" {
    pub fn QueryWriteStatus(
        client: *mut GoogleStorageClient,
        upload_id: *const ::std::os::raw::c_char,
    ) -> CallResult;
}
extern "C" {
    pub fn DeleteWrite(
        client: *mut GoogleStorageClient,
        upload_id: *const ::std::os::raw::c_char,
    ) -> CallResult;
}
extern "C" {
    pub fn ResumableWriteObject(
        client: *mut GoogleStorageClient,
        bucket: *const ::std::os::raw::c_char,
        obj: *const ::std::os::raw::c_char,
        object_len: ::std::os::raw::c_ulong,
    ) -> CallResult;
}
extern "C" {
    pub fn NonResumableWriteObject(
        client: *mut GoogleStorageClient,
        bucket: *const ::std::os::raw::c_char,
        obj: *const ::std::os::raw::c_char,
        object_len: ::std::os::raw::c_ulong,
    ) -> CallResult;
}
extern "C" {
    pub fn RangeRead(
        client: *mut GoogleStorageClient,
        bucket: *const ::std::os::raw::c_char,
        obj: *const ::std::os::raw::c_char,
        read_offset: ::std::os::raw::c_ulong,
        read_length: ::std::os::raw::c_ulong,
    ) -> CallResult;
}
