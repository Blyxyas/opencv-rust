pub type PtrOfFastLineDetector = core::Ptr<dyn crate::ximgproc::FastLineDetector>;

ptr_extern! { dyn crate::ximgproc::FastLineDetector,
	cv_PtrOfFastLineDetector_delete, cv_PtrOfFastLineDetector_get_inner_ptr, cv_PtrOfFastLineDetector_get_inner_ptr_mut
}

impl PtrOfFastLineDetector {
	#[inline] pub fn as_raw_PtrOfFastLineDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFastLineDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::FastLineDetectorConst for PtrOfFastLineDetector {
	#[inline] fn as_raw_FastLineDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::FastLineDetector for PtrOfFastLineDetector {
	#[inline] fn as_raw_mut_FastLineDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFastLineDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFastLineDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

