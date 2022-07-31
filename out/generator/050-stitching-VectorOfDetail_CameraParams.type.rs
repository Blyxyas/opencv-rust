pub type VectorOfDetail_CameraParams = core::Vector<crate::stitching::Detail_CameraParams>;

impl VectorOfDetail_CameraParams {
	pub fn as_raw_VectorOfDetail_CameraParams(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfDetail_CameraParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::stitching::Detail_CameraParams, *const c_void, *mut c_void,
	cv_VectorOfDetail_CameraParams_new, cv_VectorOfDetail_CameraParams_delete,
	cv_VectorOfDetail_CameraParams_len, cv_VectorOfDetail_CameraParams_is_empty,
	cv_VectorOfDetail_CameraParams_capacity, cv_VectorOfDetail_CameraParams_resize,
	cv_VectorOfDetail_CameraParams_shrink_to_fit,
	cv_VectorOfDetail_CameraParams_reserve, cv_VectorOfDetail_CameraParams_remove,
	cv_VectorOfDetail_CameraParams_swap, cv_VectorOfDetail_CameraParams_clear,
	cv_VectorOfDetail_CameraParams_get, cv_VectorOfDetail_CameraParams_set,
	cv_VectorOfDetail_CameraParams_push, cv_VectorOfDetail_CameraParams_insert,
}
vector_non_copy_or_bool! { crate::stitching::Detail_CameraParams }

unsafe impl Send for core::Vector<crate::stitching::Detail_CameraParams> {}

