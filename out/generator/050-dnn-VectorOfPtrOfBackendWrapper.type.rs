pub type VectorOfPtrOfBackendWrapper = core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>>;

impl VectorOfPtrOfBackendWrapper {
	pub fn as_raw_VectorOfPtrOfBackendWrapper(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfPtrOfBackendWrapper(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Ptr<dyn crate::dnn::BackendWrapper>, *const c_void, *mut c_void,
	cv_VectorOfPtrOfBackendWrapper_new, cv_VectorOfPtrOfBackendWrapper_delete,
	cv_VectorOfPtrOfBackendWrapper_len, cv_VectorOfPtrOfBackendWrapper_is_empty,
	cv_VectorOfPtrOfBackendWrapper_capacity, cv_VectorOfPtrOfBackendWrapper_resize,
	cv_VectorOfPtrOfBackendWrapper_shrink_to_fit,
	cv_VectorOfPtrOfBackendWrapper_reserve, cv_VectorOfPtrOfBackendWrapper_remove,
	cv_VectorOfPtrOfBackendWrapper_swap, cv_VectorOfPtrOfBackendWrapper_clear,
	cv_VectorOfPtrOfBackendWrapper_get, cv_VectorOfPtrOfBackendWrapper_set,
	cv_VectorOfPtrOfBackendWrapper_push, cv_VectorOfPtrOfBackendWrapper_insert,
}
vector_non_copy_or_bool! { core::Ptr<dyn crate::dnn::BackendWrapper> }

unsafe impl Send for core::Vector<core::Ptr<dyn crate::dnn::BackendWrapper>> {}

