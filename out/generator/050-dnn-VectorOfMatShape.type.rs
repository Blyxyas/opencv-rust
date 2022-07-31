pub type VectorOfMatShape = core::Vector<crate::dnn::MatShape>;

impl VectorOfMatShape {
	pub fn as_raw_VectorOfMatShape(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfMatShape(&mut self) -> *mut c_void { self.as_raw_mut() }
}

