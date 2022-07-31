pub type VectorOfDTrees_Node = core::Vector<crate::ml::DTrees_Node>;

impl VectorOfDTrees_Node {
	pub fn as_raw_VectorOfDTrees_Node(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfDTrees_Node(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::ml::DTrees_Node, *const c_void, *mut c_void,
	cv_VectorOfDTrees_Node_new, cv_VectorOfDTrees_Node_delete,
	cv_VectorOfDTrees_Node_len, cv_VectorOfDTrees_Node_is_empty,
	cv_VectorOfDTrees_Node_capacity, cv_VectorOfDTrees_Node_resize,
	cv_VectorOfDTrees_Node_shrink_to_fit,
	cv_VectorOfDTrees_Node_reserve, cv_VectorOfDTrees_Node_remove,
	cv_VectorOfDTrees_Node_swap, cv_VectorOfDTrees_Node_clear,
	cv_VectorOfDTrees_Node_get, cv_VectorOfDTrees_Node_set,
	cv_VectorOfDTrees_Node_push, cv_VectorOfDTrees_Node_insert,
}
vector_non_copy_or_bool! { crate::ml::DTrees_Node }

unsafe impl Send for core::Vector<crate::ml::DTrees_Node> {}

