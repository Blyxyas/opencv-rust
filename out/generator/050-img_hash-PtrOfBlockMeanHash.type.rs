pub type PtrOfBlockMeanHash = core::Ptr<crate::img_hash::BlockMeanHash>;

ptr_extern! { crate::img_hash::BlockMeanHash,
	cv_PtrOfBlockMeanHash_delete, cv_PtrOfBlockMeanHash_get_inner_ptr, cv_PtrOfBlockMeanHash_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::img_hash::BlockMeanHash, cv_PtrOfBlockMeanHash_new }

impl PtrOfBlockMeanHash {
	#[inline] pub fn as_raw_PtrOfBlockMeanHash(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBlockMeanHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::BlockMeanHashTraitConst for PtrOfBlockMeanHash {
	#[inline] fn as_raw_BlockMeanHash(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::BlockMeanHashTrait for PtrOfBlockMeanHash {
	#[inline] fn as_raw_mut_BlockMeanHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBlockMeanHash {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBlockMeanHash {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTraitConst for PtrOfBlockMeanHash {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::ImgHashBaseTrait for PtrOfBlockMeanHash {
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

