pub type VectorOfFacemarkAAM_Model_Texture = core::Vector<crate::face::FacemarkAAM_Model_Texture>;

impl VectorOfFacemarkAAM_Model_Texture {
	pub fn as_raw_VectorOfFacemarkAAM_Model_Texture(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfFacemarkAAM_Model_Texture(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::face::FacemarkAAM_Model_Texture, *const c_void, *mut c_void,
	cv_VectorOfFacemarkAAM_Model_Texture_new, cv_VectorOfFacemarkAAM_Model_Texture_delete,
	cv_VectorOfFacemarkAAM_Model_Texture_len, cv_VectorOfFacemarkAAM_Model_Texture_is_empty,
	cv_VectorOfFacemarkAAM_Model_Texture_capacity, cv_VectorOfFacemarkAAM_Model_Texture_resize,
	cv_VectorOfFacemarkAAM_Model_Texture_shrink_to_fit,
	cv_VectorOfFacemarkAAM_Model_Texture_reserve, cv_VectorOfFacemarkAAM_Model_Texture_remove,
	cv_VectorOfFacemarkAAM_Model_Texture_swap, cv_VectorOfFacemarkAAM_Model_Texture_clear,
	cv_VectorOfFacemarkAAM_Model_Texture_get, cv_VectorOfFacemarkAAM_Model_Texture_set,
	cv_VectorOfFacemarkAAM_Model_Texture_push, cv_VectorOfFacemarkAAM_Model_Texture_insert,
}
vector_non_copy_or_bool! { crate::face::FacemarkAAM_Model_Texture }

unsafe impl Send for core::Vector<crate::face::FacemarkAAM_Model_Texture> {}

