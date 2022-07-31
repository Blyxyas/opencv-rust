pub type VectorOfGShape = core::Vector<crate::gapi::GShape>;

impl VectorOfGShape {
	pub fn as_raw_VectorOfGShape(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfGShape(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::gapi::GShape, *const c_void, *mut c_void,
	cv_VectorOfGShape_new, cv_VectorOfGShape_delete,
	cv_VectorOfGShape_len, cv_VectorOfGShape_is_empty,
	cv_VectorOfGShape_capacity, cv_VectorOfGShape_resize,
	cv_VectorOfGShape_shrink_to_fit,
	cv_VectorOfGShape_reserve, cv_VectorOfGShape_remove,
	cv_VectorOfGShape_swap, cv_VectorOfGShape_clear,
	cv_VectorOfGShape_get, cv_VectorOfGShape_set,
	cv_VectorOfGShape_push, cv_VectorOfGShape_insert,
}
vector_copy_non_bool! { crate::gapi::GShape, *const c_void, *mut c_void,
	cv_VectorOfGShape_data, cv_VectorOfGShape_data_mut, cv_VectorOfGShape_from_slice,
	cv_VectorOfGShape_clone,
}

unsafe impl Send for core::Vector<crate::gapi::GShape> {}

