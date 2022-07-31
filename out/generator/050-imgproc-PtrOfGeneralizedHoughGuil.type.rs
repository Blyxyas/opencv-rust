pub type PtrOfGeneralizedHoughGuil = core::Ptr<dyn crate::imgproc::GeneralizedHoughGuil>;

ptr_extern! { dyn crate::imgproc::GeneralizedHoughGuil,
	cv_PtrOfGeneralizedHoughGuil_delete, cv_PtrOfGeneralizedHoughGuil_get_inner_ptr, cv_PtrOfGeneralizedHoughGuil_get_inner_ptr_mut
}

impl PtrOfGeneralizedHoughGuil {
	#[inline] pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughGuil(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::imgproc::GeneralizedHoughGuilConst for PtrOfGeneralizedHoughGuil {
	#[inline] fn as_raw_GeneralizedHoughGuil(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::GeneralizedHoughGuil for PtrOfGeneralizedHoughGuil {
	#[inline] fn as_raw_mut_GeneralizedHoughGuil(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfGeneralizedHoughGuil {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfGeneralizedHoughGuil {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::imgproc::GeneralizedHoughConst for PtrOfGeneralizedHoughGuil {
	#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughGuil {
	#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

