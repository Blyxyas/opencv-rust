pub type PtrOfTonemapReinhard = core::Ptr<dyn crate::photo::TonemapReinhard>;

ptr_extern! { dyn crate::photo::TonemapReinhard,
	cv_PtrOfTonemapReinhard_delete, cv_PtrOfTonemapReinhard_get_inner_ptr, cv_PtrOfTonemapReinhard_get_inner_ptr_mut
}

impl PtrOfTonemapReinhard {
	#[inline] pub fn as_raw_PtrOfTonemapReinhard(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTonemapReinhard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::photo::TonemapReinhardConst for PtrOfTonemapReinhard {
	#[inline] fn as_raw_TonemapReinhard(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::TonemapReinhard for PtrOfTonemapReinhard {
	#[inline] fn as_raw_mut_TonemapReinhard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfTonemapReinhard {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfTonemapReinhard {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::photo::TonemapConst for PtrOfTonemapReinhard {
	#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::Tonemap for PtrOfTonemapReinhard {
	#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

