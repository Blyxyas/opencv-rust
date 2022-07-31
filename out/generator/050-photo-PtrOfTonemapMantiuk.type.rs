pub type PtrOfTonemapMantiuk = core::Ptr<dyn crate::photo::TonemapMantiuk>;

ptr_extern! { dyn crate::photo::TonemapMantiuk,
	cv_PtrOfTonemapMantiuk_delete, cv_PtrOfTonemapMantiuk_get_inner_ptr, cv_PtrOfTonemapMantiuk_get_inner_ptr_mut
}

impl PtrOfTonemapMantiuk {
	#[inline] pub fn as_raw_PtrOfTonemapMantiuk(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTonemapMantiuk(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::photo::TonemapMantiukConst for PtrOfTonemapMantiuk {
	#[inline] fn as_raw_TonemapMantiuk(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::TonemapMantiuk for PtrOfTonemapMantiuk {
	#[inline] fn as_raw_mut_TonemapMantiuk(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfTonemapMantiuk {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfTonemapMantiuk {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::photo::TonemapConst for PtrOfTonemapMantiuk {
	#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::Tonemap for PtrOfTonemapMantiuk {
	#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

