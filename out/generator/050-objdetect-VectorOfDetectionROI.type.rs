pub type VectorOfDetectionROI = core::Vector<crate::objdetect::DetectionROI>;

impl VectorOfDetectionROI {
	pub fn as_raw_VectorOfDetectionROI(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfDetectionROI(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::objdetect::DetectionROI, *const c_void, *mut c_void,
	cv_VectorOfDetectionROI_new, cv_VectorOfDetectionROI_delete,
	cv_VectorOfDetectionROI_len, cv_VectorOfDetectionROI_is_empty,
	cv_VectorOfDetectionROI_capacity, cv_VectorOfDetectionROI_resize,
	cv_VectorOfDetectionROI_shrink_to_fit,
	cv_VectorOfDetectionROI_reserve, cv_VectorOfDetectionROI_remove,
	cv_VectorOfDetectionROI_swap, cv_VectorOfDetectionROI_clear,
	cv_VectorOfDetectionROI_get, cv_VectorOfDetectionROI_set,
	cv_VectorOfDetectionROI_push, cv_VectorOfDetectionROI_insert,
}
vector_non_copy_or_bool! { crate::objdetect::DetectionROI }

unsafe impl Send for core::Vector<crate::objdetect::DetectionROI> {}

