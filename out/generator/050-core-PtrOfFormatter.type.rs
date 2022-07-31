pub type PtrOfFormatter = core::Ptr<dyn core::Formatter>;

ptr_extern! { dyn core::Formatter,
	cv_PtrOfFormatter_delete, cv_PtrOfFormatter_get_inner_ptr, cv_PtrOfFormatter_get_inner_ptr_mut
}

impl PtrOfFormatter {
	#[inline] pub fn as_raw_PtrOfFormatter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFormatter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::FormatterConst for PtrOfFormatter {
	#[inline] fn as_raw_Formatter(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::Formatter for PtrOfFormatter {
	#[inline] fn as_raw_mut_Formatter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

