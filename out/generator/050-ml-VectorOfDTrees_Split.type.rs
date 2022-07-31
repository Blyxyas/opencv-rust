pub type VectorOfDTrees_Split = core::Vector<crate::ml::DTrees_Split>;

impl VectorOfDTrees_Split {
	pub fn as_raw_VectorOfDTrees_Split(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfDTrees_Split(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::ml::DTrees_Split, *const c_void, *mut c_void,
	cv_VectorOfDTrees_Split_new, cv_VectorOfDTrees_Split_delete,
	cv_VectorOfDTrees_Split_len, cv_VectorOfDTrees_Split_is_empty,
	cv_VectorOfDTrees_Split_capacity, cv_VectorOfDTrees_Split_resize,
	cv_VectorOfDTrees_Split_shrink_to_fit,
	cv_VectorOfDTrees_Split_reserve, cv_VectorOfDTrees_Split_remove,
	cv_VectorOfDTrees_Split_swap, cv_VectorOfDTrees_Split_clear,
	cv_VectorOfDTrees_Split_get, cv_VectorOfDTrees_Split_set,
	cv_VectorOfDTrees_Split_push, cv_VectorOfDTrees_Split_insert,
}
vector_non_copy_or_bool! { crate::ml::DTrees_Split }

unsafe impl Send for core::Vector<crate::ml::DTrees_Split> {}

