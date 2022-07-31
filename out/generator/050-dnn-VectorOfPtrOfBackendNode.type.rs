pub type VectorOfPtrOfBackendNode = core::Vector<core::Ptr<crate::dnn::BackendNode>>;

impl VectorOfPtrOfBackendNode {
	pub fn as_raw_VectorOfPtrOfBackendNode(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfPtrOfBackendNode(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Ptr<crate::dnn::BackendNode>, *const c_void, *mut c_void,
	cv_VectorOfPtrOfBackendNode_new, cv_VectorOfPtrOfBackendNode_delete,
	cv_VectorOfPtrOfBackendNode_len, cv_VectorOfPtrOfBackendNode_is_empty,
	cv_VectorOfPtrOfBackendNode_capacity, cv_VectorOfPtrOfBackendNode_resize,
	cv_VectorOfPtrOfBackendNode_shrink_to_fit,
	cv_VectorOfPtrOfBackendNode_reserve, cv_VectorOfPtrOfBackendNode_remove,
	cv_VectorOfPtrOfBackendNode_swap, cv_VectorOfPtrOfBackendNode_clear,
	cv_VectorOfPtrOfBackendNode_get, cv_VectorOfPtrOfBackendNode_set,
	cv_VectorOfPtrOfBackendNode_push, cv_VectorOfPtrOfBackendNode_insert,
}
vector_non_copy_or_bool! { core::Ptr<crate::dnn::BackendNode> }

unsafe impl Send for core::Vector<core::Ptr<crate::dnn::BackendNode>> {}

