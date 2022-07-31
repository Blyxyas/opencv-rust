pub type VectorOfDPMDetector_ObjectDetection = core::Vector<crate::dpm::DPMDetector_ObjectDetection>;

impl VectorOfDPMDetector_ObjectDetection {
	pub fn as_raw_VectorOfDPMDetector_ObjectDetection(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfDPMDetector_ObjectDetection(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::dpm::DPMDetector_ObjectDetection, *const c_void, *mut c_void,
	cv_VectorOfDPMDetector_ObjectDetection_new, cv_VectorOfDPMDetector_ObjectDetection_delete,
	cv_VectorOfDPMDetector_ObjectDetection_len, cv_VectorOfDPMDetector_ObjectDetection_is_empty,
	cv_VectorOfDPMDetector_ObjectDetection_capacity, cv_VectorOfDPMDetector_ObjectDetection_resize,
	cv_VectorOfDPMDetector_ObjectDetection_shrink_to_fit,
	cv_VectorOfDPMDetector_ObjectDetection_reserve, cv_VectorOfDPMDetector_ObjectDetection_remove,
	cv_VectorOfDPMDetector_ObjectDetection_swap, cv_VectorOfDPMDetector_ObjectDetection_clear,
	cv_VectorOfDPMDetector_ObjectDetection_get, cv_VectorOfDPMDetector_ObjectDetection_set,
	cv_VectorOfDPMDetector_ObjectDetection_push, cv_VectorOfDPMDetector_ObjectDetection_insert,
}
vector_non_copy_or_bool! { crate::dpm::DPMDetector_ObjectDetection }

unsafe impl Send for core::Vector<crate::dpm::DPMDetector_ObjectDetection> {}

