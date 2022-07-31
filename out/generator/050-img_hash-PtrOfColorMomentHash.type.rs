pub type PtrOfColorMomentHash = core::Ptr<crate::img_hash::ColorMomentHash>;

ptr_extern! { crate::img_hash::ColorMomentHash,
	cv_PtrOfColorMomentHash_delete, cv_PtrOfColorMomentHash_get_inner_ptr, cv_PtrOfColorMomentHash_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::img_hash::ColorMomentHash, cv_PtrOfColorMomentHash_new }

impl PtrOfColorMomentHash {
	#[inline] pub fn as_raw_PtrOfColorMomentHash(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfColorMomentHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::ColorMomentHashTraitConst for PtrOfColorMomentHash {
	#[inline] fn as_raw_ColorMomentHash(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::ColorMomentHashTrait for PtrOfColorMomentHash {
	#[inline] fn as_raw_mut_ColorMomentHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfColorMomentHash {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfColorMomentHash {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTraitConst for PtrOfColorMomentHash {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::ImgHashBaseTrait for PtrOfColorMomentHash {
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

