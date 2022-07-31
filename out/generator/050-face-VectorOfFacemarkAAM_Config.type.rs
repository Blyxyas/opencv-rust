pub type VectorOfFacemarkAAM_Config = core::Vector<crate::face::FacemarkAAM_Config>;

impl VectorOfFacemarkAAM_Config {
	pub fn as_raw_VectorOfFacemarkAAM_Config(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfFacemarkAAM_Config(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::face::FacemarkAAM_Config, *const c_void, *mut c_void,
	cv_VectorOfFacemarkAAM_Config_new, cv_VectorOfFacemarkAAM_Config_delete,
	cv_VectorOfFacemarkAAM_Config_len, cv_VectorOfFacemarkAAM_Config_is_empty,
	cv_VectorOfFacemarkAAM_Config_capacity, cv_VectorOfFacemarkAAM_Config_resize,
	cv_VectorOfFacemarkAAM_Config_shrink_to_fit,
	cv_VectorOfFacemarkAAM_Config_reserve, cv_VectorOfFacemarkAAM_Config_remove,
	cv_VectorOfFacemarkAAM_Config_swap, cv_VectorOfFacemarkAAM_Config_clear,
	cv_VectorOfFacemarkAAM_Config_get, cv_VectorOfFacemarkAAM_Config_set,
	cv_VectorOfFacemarkAAM_Config_push, cv_VectorOfFacemarkAAM_Config_insert,
}
vector_non_copy_or_bool! { crate::face::FacemarkAAM_Config }

unsafe impl Send for core::Vector<crate::face::FacemarkAAM_Config> {}

