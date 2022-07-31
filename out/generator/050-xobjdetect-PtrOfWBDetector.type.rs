pub type PtrOfWBDetector = core::Ptr<dyn crate::xobjdetect::WBDetector>;

ptr_extern! { dyn crate::xobjdetect::WBDetector,
	cv_PtrOfWBDetector_delete, cv_PtrOfWBDetector_get_inner_ptr, cv_PtrOfWBDetector_get_inner_ptr_mut
}

impl PtrOfWBDetector {
	#[inline] pub fn as_raw_PtrOfWBDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfWBDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xobjdetect::WBDetectorConst for PtrOfWBDetector {
	#[inline] fn as_raw_WBDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xobjdetect::WBDetector for PtrOfWBDetector {
	#[inline] fn as_raw_mut_WBDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

