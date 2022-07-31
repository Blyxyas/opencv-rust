pub type VectorOfVideoCaptureAPIs = core::Vector<crate::videoio::VideoCaptureAPIs>;

impl VectorOfVideoCaptureAPIs {
	pub fn as_raw_VectorOfVideoCaptureAPIs(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVideoCaptureAPIs(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::videoio::VideoCaptureAPIs, *const c_void, *mut c_void,
	cv_VectorOfVideoCaptureAPIs_new, cv_VectorOfVideoCaptureAPIs_delete,
	cv_VectorOfVideoCaptureAPIs_len, cv_VectorOfVideoCaptureAPIs_is_empty,
	cv_VectorOfVideoCaptureAPIs_capacity, cv_VectorOfVideoCaptureAPIs_resize,
	cv_VectorOfVideoCaptureAPIs_shrink_to_fit,
	cv_VectorOfVideoCaptureAPIs_reserve, cv_VectorOfVideoCaptureAPIs_remove,
	cv_VectorOfVideoCaptureAPIs_swap, cv_VectorOfVideoCaptureAPIs_clear,
	cv_VectorOfVideoCaptureAPIs_get, cv_VectorOfVideoCaptureAPIs_set,
	cv_VectorOfVideoCaptureAPIs_push, cv_VectorOfVideoCaptureAPIs_insert,
}
vector_copy_non_bool! { crate::videoio::VideoCaptureAPIs, *const c_void, *mut c_void,
	cv_VectorOfVideoCaptureAPIs_data, cv_VectorOfVideoCaptureAPIs_data_mut, cv_VectorOfVideoCaptureAPIs_from_slice,
	cv_VectorOfVideoCaptureAPIs_clone,
}

unsafe impl Send for core::Vector<crate::videoio::VideoCaptureAPIs> {}

