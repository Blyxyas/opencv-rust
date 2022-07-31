pub type PtrOfRadialVarianceHash = core::Ptr<crate::img_hash::RadialVarianceHash>;

ptr_extern! { crate::img_hash::RadialVarianceHash,
	cv_PtrOfRadialVarianceHash_delete, cv_PtrOfRadialVarianceHash_get_inner_ptr, cv_PtrOfRadialVarianceHash_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::img_hash::RadialVarianceHash, cv_PtrOfRadialVarianceHash_new }

impl PtrOfRadialVarianceHash {
	#[inline] pub fn as_raw_PtrOfRadialVarianceHash(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRadialVarianceHash(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::img_hash::RadialVarianceHashTraitConst for PtrOfRadialVarianceHash {
	#[inline] fn as_raw_RadialVarianceHash(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::RadialVarianceHashTrait for PtrOfRadialVarianceHash {
	#[inline] fn as_raw_mut_RadialVarianceHash(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRadialVarianceHash {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRadialVarianceHash {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::img_hash::ImgHashBaseTraitConst for PtrOfRadialVarianceHash {
	#[inline] fn as_raw_ImgHashBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::img_hash::ImgHashBaseTrait for PtrOfRadialVarianceHash {
	#[inline] fn as_raw_mut_ImgHashBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

