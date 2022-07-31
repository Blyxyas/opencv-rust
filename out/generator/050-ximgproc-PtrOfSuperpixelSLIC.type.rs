pub type PtrOfSuperpixelSLIC = core::Ptr<dyn crate::ximgproc::SuperpixelSLIC>;

ptr_extern! { dyn crate::ximgproc::SuperpixelSLIC,
	cv_PtrOfSuperpixelSLIC_delete, cv_PtrOfSuperpixelSLIC_get_inner_ptr, cv_PtrOfSuperpixelSLIC_get_inner_ptr_mut
}

impl PtrOfSuperpixelSLIC {
	#[inline] pub fn as_raw_PtrOfSuperpixelSLIC(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSuperpixelSLIC(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::SuperpixelSLICConst for PtrOfSuperpixelSLIC {
	#[inline] fn as_raw_SuperpixelSLIC(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SuperpixelSLIC for PtrOfSuperpixelSLIC {
	#[inline] fn as_raw_mut_SuperpixelSLIC(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSuperpixelSLIC {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSuperpixelSLIC {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

