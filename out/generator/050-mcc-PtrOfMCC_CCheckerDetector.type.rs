pub type PtrOfMCC_CCheckerDetector = core::Ptr<dyn crate::mcc::MCC_CCheckerDetector>;

ptr_extern! { dyn crate::mcc::MCC_CCheckerDetector,
	cv_PtrOfMCC_CCheckerDetector_delete, cv_PtrOfMCC_CCheckerDetector_get_inner_ptr, cv_PtrOfMCC_CCheckerDetector_get_inner_ptr_mut
}

impl PtrOfMCC_CCheckerDetector {
	#[inline] pub fn as_raw_PtrOfMCC_CCheckerDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMCC_CCheckerDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::mcc::MCC_CCheckerDetectorConst for PtrOfMCC_CCheckerDetector {
	#[inline] fn as_raw_MCC_CCheckerDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::mcc::MCC_CCheckerDetector for PtrOfMCC_CCheckerDetector {
	#[inline] fn as_raw_mut_MCC_CCheckerDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfMCC_CCheckerDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfMCC_CCheckerDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

