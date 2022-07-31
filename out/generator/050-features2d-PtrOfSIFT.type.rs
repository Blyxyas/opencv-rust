pub type PtrOfSIFT = core::Ptr<crate::features2d::SIFT>;

ptr_extern! { crate::features2d::SIFT,
	cv_PtrOfSIFT_delete, cv_PtrOfSIFT_get_inner_ptr, cv_PtrOfSIFT_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::features2d::SIFT, cv_PtrOfSIFT_new }

impl PtrOfSIFT {
	#[inline] pub fn as_raw_PtrOfSIFT(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSIFT(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::SIFTTraitConst for PtrOfSIFT {
	#[inline] fn as_raw_SIFT(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::SIFTTrait for PtrOfSIFT {
	#[inline] fn as_raw_mut_SIFT(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSIFT {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSIFT {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfSIFT {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfSIFT {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfSIFT, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfSIFT_to_PtrOfFeature2D,
}

