pub type PtrOfGeneralizedHoughBallard = core::Ptr<dyn crate::imgproc::GeneralizedHoughBallard>;

ptr_extern! { dyn crate::imgproc::GeneralizedHoughBallard,
	cv_PtrOfGeneralizedHoughBallard_delete, cv_PtrOfGeneralizedHoughBallard_get_inner_ptr, cv_PtrOfGeneralizedHoughBallard_get_inner_ptr_mut
}

impl PtrOfGeneralizedHoughBallard {
	#[inline] pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughBallard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::imgproc::GeneralizedHoughBallardConst for PtrOfGeneralizedHoughBallard {
	#[inline] fn as_raw_GeneralizedHoughBallard(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::GeneralizedHoughBallard for PtrOfGeneralizedHoughBallard {
	#[inline] fn as_raw_mut_GeneralizedHoughBallard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfGeneralizedHoughBallard {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfGeneralizedHoughBallard {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::imgproc::GeneralizedHoughConst for PtrOfGeneralizedHoughBallard {
	#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughBallard {
	#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

