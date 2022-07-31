pub type PtrOfSuperpixelLSC = core::Ptr<dyn crate::ximgproc::SuperpixelLSC>;

ptr_extern! { dyn crate::ximgproc::SuperpixelLSC,
	cv_PtrOfSuperpixelLSC_delete, cv_PtrOfSuperpixelLSC_get_inner_ptr, cv_PtrOfSuperpixelLSC_get_inner_ptr_mut
}

impl PtrOfSuperpixelLSC {
	#[inline] pub fn as_raw_PtrOfSuperpixelLSC(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperpixelLSC(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::SuperpixelLSCConst for PtrOfSuperpixelLSC {
	#[inline] fn as_raw_SuperpixelLSC(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SuperpixelLSC for PtrOfSuperpixelLSC {
	#[inline] fn as_raw_mut_SuperpixelLSC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSuperpixelLSC {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSuperpixelLSC {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

