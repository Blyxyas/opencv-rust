pub type PtrOfRFFeatureGetter = core::Ptr<dyn crate::ximgproc::RFFeatureGetter>;

ptr_extern! { dyn crate::ximgproc::RFFeatureGetter,
	cv_PtrOfRFFeatureGetter_delete, cv_PtrOfRFFeatureGetter_get_inner_ptr, cv_PtrOfRFFeatureGetter_get_inner_ptr_mut
}

impl PtrOfRFFeatureGetter {
	#[inline] pub fn as_raw_PtrOfRFFeatureGetter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRFFeatureGetter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::RFFeatureGetterConst for PtrOfRFFeatureGetter {
	#[inline] fn as_raw_RFFeatureGetter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::RFFeatureGetter for PtrOfRFFeatureGetter {
	#[inline] fn as_raw_mut_RFFeatureGetter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRFFeatureGetter {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRFFeatureGetter {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

