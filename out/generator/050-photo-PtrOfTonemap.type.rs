pub type PtrOfTonemap = core::Ptr<dyn crate::photo::Tonemap>;

ptr_extern! { dyn crate::photo::Tonemap,
	cv_PtrOfTonemap_delete, cv_PtrOfTonemap_get_inner_ptr, cv_PtrOfTonemap_get_inner_ptr_mut
}

impl PtrOfTonemap {
	#[inline] pub fn as_raw_PtrOfTonemap(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTonemap(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::photo::TonemapConst for PtrOfTonemap {
	#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::Tonemap for PtrOfTonemap {
	#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfTonemap {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfTonemap {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

