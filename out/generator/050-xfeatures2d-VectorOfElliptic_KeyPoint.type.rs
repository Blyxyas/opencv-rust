pub type VectorOfElliptic_KeyPoint = core::Vector<crate::xfeatures2d::Elliptic_KeyPoint>;

impl VectorOfElliptic_KeyPoint {
	pub fn as_raw_VectorOfElliptic_KeyPoint(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfElliptic_KeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::xfeatures2d::Elliptic_KeyPoint, *const c_void, *mut c_void,
	cv_VectorOfElliptic_KeyPoint_new, cv_VectorOfElliptic_KeyPoint_delete,
	cv_VectorOfElliptic_KeyPoint_len, cv_VectorOfElliptic_KeyPoint_is_empty,
	cv_VectorOfElliptic_KeyPoint_capacity, cv_VectorOfElliptic_KeyPoint_resize,
	cv_VectorOfElliptic_KeyPoint_shrink_to_fit,
	cv_VectorOfElliptic_KeyPoint_reserve, cv_VectorOfElliptic_KeyPoint_remove,
	cv_VectorOfElliptic_KeyPoint_swap, cv_VectorOfElliptic_KeyPoint_clear,
	cv_VectorOfElliptic_KeyPoint_get, cv_VectorOfElliptic_KeyPoint_set,
	cv_VectorOfElliptic_KeyPoint_push, cv_VectorOfElliptic_KeyPoint_insert,
}
vector_non_copy_or_bool! { crate::xfeatures2d::Elliptic_KeyPoint }

unsafe impl Send for core::Vector<crate::xfeatures2d::Elliptic_KeyPoint> {}

