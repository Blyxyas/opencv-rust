pub type PtrOfMarrHildrethHash = core::Ptr<crate::img_hash::MarrHildrethHash>;

ptr_extern! { crate::img_hash::MarrHildrethHash,
	cv_PtrOfMarrHildrethHash_delete, cv_PtrOfMarrHildrethHash_get_inner_ptr, cv_PtrOfMarrHildrethHash_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::img_hash::MarrHildrethHash, cv_PtrOfMarrHildrethHash_new }

impl PtrOfMarrHildrethHash {
	#[inline] pub fn as_raw_PtrOfMarrHildrethHash(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMarrHildrethHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::MarrHildrethHashTraitConst for PtrOfMarrHildrethHash {
	#[inline] fn as_raw_MarrHildrethHash(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::MarrHildrethHashTrait for PtrOfMarrHildrethHash {
	#[inline] fn as_raw_mut_MarrHildrethHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfMarrHildrethHash {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfMarrHildrethHash {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTraitConst for PtrOfMarrHildrethHash {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::ImgHashBaseTrait for PtrOfMarrHildrethHash {
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

