pub type PtrOfSuperres_SuperResolution = core::Ptr<dyn crate::superres::Superres_SuperResolution>;

ptr_extern! { dyn crate::superres::Superres_SuperResolution,
	cv_PtrOfSuperres_SuperResolution_delete, cv_PtrOfSuperres_SuperResolution_get_inner_ptr, cv_PtrOfSuperres_SuperResolution_get_inner_ptr_mut
}

impl PtrOfSuperres_SuperResolution {
	#[inline] pub fn as_raw_PtrOfSuperres_SuperResolution(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperres_SuperResolution(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::superres::Superres_SuperResolutionConst for PtrOfSuperres_SuperResolution {
	#[inline] fn as_raw_Superres_SuperResolution(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::Superres_SuperResolution for PtrOfSuperres_SuperResolution {
	#[inline] fn as_raw_mut_Superres_SuperResolution(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSuperres_SuperResolution {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSuperres_SuperResolution {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::superres::Superres_FrameSourceConst for PtrOfSuperres_SuperResolution {
	#[inline] fn as_raw_Superres_FrameSource(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::superres::Superres_FrameSource for PtrOfSuperres_SuperResolution {
	#[inline] fn as_raw_mut_Superres_FrameSource(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

