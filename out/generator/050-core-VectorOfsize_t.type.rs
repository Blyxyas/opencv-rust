pub type VectorOfsize_t = core::Vector<size_t>;

impl VectorOfsize_t {
	pub fn as_raw_VectorOfsize_t(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfsize_t(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { size_t, *const c_void, *mut c_void,
	cv_VectorOfsize_t_new, cv_VectorOfsize_t_delete,
	cv_VectorOfsize_t_len, cv_VectorOfsize_t_is_empty,
	cv_VectorOfsize_t_capacity, cv_VectorOfsize_t_resize,
	cv_VectorOfsize_t_shrink_to_fit,
	cv_VectorOfsize_t_reserve, cv_VectorOfsize_t_remove,
	cv_VectorOfsize_t_swap, cv_VectorOfsize_t_clear,
	cv_VectorOfsize_t_get, cv_VectorOfsize_t_set,
	cv_VectorOfsize_t_push, cv_VectorOfsize_t_insert,
}
vector_copy_non_bool! { size_t, *const c_void, *mut c_void,
	cv_VectorOfsize_t_data, cv_VectorOfsize_t_data_mut, cv_VectorOfsize_t_from_slice,
	cv_VectorOfsize_t_clone,
}

unsafe impl Send for core::Vector<size_t> {}

