pub type VectorOfVectorOfMatShape = core::Vector<core::Vector<crate::dnn::MatShape>>;

impl VectorOfVectorOfMatShape {
	pub fn as_raw_VectorOfVectorOfMatShape(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfMatShape(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<crate::dnn::MatShape>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfMatShape_new, cv_VectorOfVectorOfMatShape_delete,
	cv_VectorOfVectorOfMatShape_len, cv_VectorOfVectorOfMatShape_is_empty,
	cv_VectorOfVectorOfMatShape_capacity, cv_VectorOfVectorOfMatShape_resize,
	cv_VectorOfVectorOfMatShape_shrink_to_fit,
	cv_VectorOfVectorOfMatShape_reserve, cv_VectorOfVectorOfMatShape_remove,
	cv_VectorOfVectorOfMatShape_swap, cv_VectorOfVectorOfMatShape_clear,
	cv_VectorOfVectorOfMatShape_get, cv_VectorOfVectorOfMatShape_set,
	cv_VectorOfVectorOfMatShape_push, cv_VectorOfVectorOfMatShape_insert,
}
vector_non_copy_or_bool! { core::Vector<crate::dnn::MatShape> }

unsafe impl Send for core::Vector<core::Vector<crate::dnn::MatShape>> {}

