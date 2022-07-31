pub type PtrOfFreeType2 = core::Ptr<dyn crate::freetype::FreeType2>;

ptr_extern! { dyn crate::freetype::FreeType2,
	cv_PtrOfFreeType2_delete, cv_PtrOfFreeType2_get_inner_ptr, cv_PtrOfFreeType2_get_inner_ptr_mut
}

impl PtrOfFreeType2 {
	#[inline] pub fn as_raw_PtrOfFreeType2(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFreeType2(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::freetype::FreeType2Const for PtrOfFreeType2 {
	#[inline] fn as_raw_FreeType2(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::freetype::FreeType2 for PtrOfFreeType2 {
	#[inline] fn as_raw_mut_FreeType2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFreeType2 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFreeType2 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

