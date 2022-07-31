pub type PtrOfRidgeDetectionFilter = core::Ptr<dyn crate::ximgproc::RidgeDetectionFilter>;

ptr_extern! { dyn crate::ximgproc::RidgeDetectionFilter,
	cv_PtrOfRidgeDetectionFilter_delete, cv_PtrOfRidgeDetectionFilter_get_inner_ptr, cv_PtrOfRidgeDetectionFilter_get_inner_ptr_mut
}

impl PtrOfRidgeDetectionFilter {
	#[inline] pub fn as_raw_PtrOfRidgeDetectionFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRidgeDetectionFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::RidgeDetectionFilterConst for PtrOfRidgeDetectionFilter {
	#[inline] fn as_raw_RidgeDetectionFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::RidgeDetectionFilter for PtrOfRidgeDetectionFilter {
	#[inline] fn as_raw_mut_RidgeDetectionFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRidgeDetectionFilter {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRidgeDetectionFilter {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

