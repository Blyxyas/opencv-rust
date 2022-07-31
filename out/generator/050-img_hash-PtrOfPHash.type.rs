pub type PtrOfPHash = core::Ptr<crate::img_hash::PHash>;

ptr_extern! { crate::img_hash::PHash,
	cv_PtrOfPHash_delete, cv_PtrOfPHash_get_inner_ptr, cv_PtrOfPHash_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::img_hash::PHash, cv_PtrOfPHash_new }

impl PtrOfPHash {
	#[inline] pub fn as_raw_PtrOfPHash(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::PHashTraitConst for PtrOfPHash {
	#[inline] fn as_raw_PHash(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::PHashTrait for PtrOfPHash {
	#[inline] fn as_raw_mut_PHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfPHash {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfPHash {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTraitConst for PtrOfPHash {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::ImgHashBaseTrait for PtrOfPHash {
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

