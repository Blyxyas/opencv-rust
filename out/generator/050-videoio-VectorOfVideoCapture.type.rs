pub type VectorOfVideoCapture = core::Vector<crate::videoio::VideoCapture>;

impl VectorOfVideoCapture {
	pub fn as_raw_VectorOfVideoCapture(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVideoCapture(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::videoio::VideoCapture, *const c_void, *mut c_void,
	cv_VectorOfVideoCapture_new, cv_VectorOfVideoCapture_delete,
	cv_VectorOfVideoCapture_len, cv_VectorOfVideoCapture_is_empty,
	cv_VectorOfVideoCapture_capacity, cv_VectorOfVideoCapture_resize,
	cv_VectorOfVideoCapture_shrink_to_fit,
	cv_VectorOfVideoCapture_reserve, cv_VectorOfVideoCapture_remove,
	cv_VectorOfVideoCapture_swap, cv_VectorOfVideoCapture_clear,
	cv_VectorOfVideoCapture_get, cv_VectorOfVideoCapture_set,
	cv_VectorOfVideoCapture_push, cv_VectorOfVideoCapture_insert,
}
vector_non_copy_or_bool! { crate::videoio::VideoCapture }

unsafe impl Send for core::Vector<crate::videoio::VideoCapture> {}

