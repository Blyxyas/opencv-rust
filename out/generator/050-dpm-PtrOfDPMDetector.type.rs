pub type PtrOfDPMDetector = core::Ptr<dyn crate::dpm::DPMDetector>;

ptr_extern! { dyn crate::dpm::DPMDetector,
	cv_PtrOfDPMDetector_delete, cv_PtrOfDPMDetector_get_inner_ptr, cv_PtrOfDPMDetector_get_inner_ptr_mut
}

impl PtrOfDPMDetector {
	#[inline] pub fn as_raw_PtrOfDPMDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDPMDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dpm::DPMDetectorConst for PtrOfDPMDetector {
	#[inline] fn as_raw_DPMDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dpm::DPMDetector for PtrOfDPMDetector {
	#[inline] fn as_raw_mut_DPMDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

