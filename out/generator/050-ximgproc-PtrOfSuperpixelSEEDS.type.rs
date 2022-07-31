pub type PtrOfSuperpixelSEEDS = core::Ptr<dyn crate::ximgproc::SuperpixelSEEDS>;

ptr_extern! { dyn crate::ximgproc::SuperpixelSEEDS,
	cv_PtrOfSuperpixelSEEDS_delete, cv_PtrOfSuperpixelSEEDS_get_inner_ptr, cv_PtrOfSuperpixelSEEDS_get_inner_ptr_mut
}

impl PtrOfSuperpixelSEEDS {
	#[inline] pub fn as_raw_PtrOfSuperpixelSEEDS(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperpixelSEEDS(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::SuperpixelSEEDSConst for PtrOfSuperpixelSEEDS {
	#[inline] fn as_raw_SuperpixelSEEDS(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SuperpixelSEEDS for PtrOfSuperpixelSEEDS {
	#[inline] fn as_raw_mut_SuperpixelSEEDS(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSuperpixelSEEDS {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSuperpixelSEEDS {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

