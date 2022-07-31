pub type VectorOfPtrOfLayer = core::Vector<core::Ptr<crate::dnn::Layer>>;

impl VectorOfPtrOfLayer {
	pub fn as_raw_VectorOfPtrOfLayer(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfPtrOfLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Ptr<crate::dnn::Layer>, *const c_void, *mut c_void,
	cv_VectorOfPtrOfLayer_new, cv_VectorOfPtrOfLayer_delete,
	cv_VectorOfPtrOfLayer_len, cv_VectorOfPtrOfLayer_is_empty,
	cv_VectorOfPtrOfLayer_capacity, cv_VectorOfPtrOfLayer_resize,
	cv_VectorOfPtrOfLayer_shrink_to_fit,
	cv_VectorOfPtrOfLayer_reserve, cv_VectorOfPtrOfLayer_remove,
	cv_VectorOfPtrOfLayer_swap, cv_VectorOfPtrOfLayer_clear,
	cv_VectorOfPtrOfLayer_get, cv_VectorOfPtrOfLayer_set,
	cv_VectorOfPtrOfLayer_push, cv_VectorOfPtrOfLayer_insert,
}
vector_non_copy_or_bool! { core::Ptr<crate::dnn::Layer> }

unsafe impl Send for core::Vector<core::Ptr<crate::dnn::Layer>> {}

