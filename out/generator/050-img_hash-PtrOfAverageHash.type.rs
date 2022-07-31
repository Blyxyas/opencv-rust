pub type PtrOfAverageHash = core::Ptr<crate::img_hash::AverageHash>;

ptr_extern! { crate::img_hash::AverageHash,
	cv_PtrOfAverageHash_delete, cv_PtrOfAverageHash_get_inner_ptr, cv_PtrOfAverageHash_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::img_hash::AverageHash, cv_PtrOfAverageHash_new }

impl PtrOfAverageHash {
	#[inline] pub fn as_raw_PtrOfAverageHash(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAverageHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::AverageHashTraitConst for PtrOfAverageHash {
	#[inline] fn as_raw_AverageHash(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::AverageHashTrait for PtrOfAverageHash {
	#[inline] fn as_raw_mut_AverageHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAverageHash {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAverageHash {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTraitConst for PtrOfAverageHash {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::ImgHashBaseTrait for PtrOfAverageHash {
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

