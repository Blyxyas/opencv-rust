pub type VectorOfGPCPatchDescriptor = core::Vector<crate::optflow::GPCPatchDescriptor>;

impl VectorOfGPCPatchDescriptor {
	pub fn as_raw_VectorOfGPCPatchDescriptor(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfGPCPatchDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::optflow::GPCPatchDescriptor, *const c_void, *mut c_void,
	cv_VectorOfGPCPatchDescriptor_new, cv_VectorOfGPCPatchDescriptor_delete,
	cv_VectorOfGPCPatchDescriptor_len, cv_VectorOfGPCPatchDescriptor_is_empty,
	cv_VectorOfGPCPatchDescriptor_capacity, cv_VectorOfGPCPatchDescriptor_resize,
	cv_VectorOfGPCPatchDescriptor_shrink_to_fit,
	cv_VectorOfGPCPatchDescriptor_reserve, cv_VectorOfGPCPatchDescriptor_remove,
	cv_VectorOfGPCPatchDescriptor_swap, cv_VectorOfGPCPatchDescriptor_clear,
	cv_VectorOfGPCPatchDescriptor_get, cv_VectorOfGPCPatchDescriptor_set,
	cv_VectorOfGPCPatchDescriptor_push, cv_VectorOfGPCPatchDescriptor_insert,
}
vector_non_copy_or_bool! { crate::optflow::GPCPatchDescriptor }

unsafe impl Send for core::Vector<crate::optflow::GPCPatchDescriptor> {}

