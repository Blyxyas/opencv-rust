pub type VectorOfRotatedRect = core::Vector<core::RotatedRect>;

impl VectorOfRotatedRect {
	pub fn as_raw_VectorOfRotatedRect(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfRotatedRect(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::RotatedRect, *const c_void, *mut c_void,
	cv_VectorOfRotatedRect_new, cv_VectorOfRotatedRect_delete,
	cv_VectorOfRotatedRect_len, cv_VectorOfRotatedRect_is_empty,
	cv_VectorOfRotatedRect_capacity, cv_VectorOfRotatedRect_resize,
	cv_VectorOfRotatedRect_shrink_to_fit,
	cv_VectorOfRotatedRect_reserve, cv_VectorOfRotatedRect_remove,
	cv_VectorOfRotatedRect_swap, cv_VectorOfRotatedRect_clear,
	cv_VectorOfRotatedRect_get, cv_VectorOfRotatedRect_set,
	cv_VectorOfRotatedRect_push, cv_VectorOfRotatedRect_insert,
}
vector_non_copy_or_bool! { core::RotatedRect }

unsafe impl Send for core::Vector<core::RotatedRect> {}

