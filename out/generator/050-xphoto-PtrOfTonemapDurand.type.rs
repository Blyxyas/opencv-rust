pub type PtrOfTonemapDurand = core::Ptr<dyn crate::xphoto::TonemapDurand>;

ptr_extern! { dyn crate::xphoto::TonemapDurand,
	cv_PtrOfTonemapDurand_delete, cv_PtrOfTonemapDurand_get_inner_ptr, cv_PtrOfTonemapDurand_get_inner_ptr_mut
}

impl PtrOfTonemapDurand {
	#[inline] pub fn as_raw_PtrOfTonemapDurand(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTonemapDurand(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xphoto::TonemapDurandConst for PtrOfTonemapDurand {
	#[inline] fn as_raw_TonemapDurand(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xphoto::TonemapDurand for PtrOfTonemapDurand {
	#[inline] fn as_raw_mut_TonemapDurand(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfTonemapDurand {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfTonemapDurand {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::photo::TonemapConst for PtrOfTonemapDurand {
	#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::Tonemap for PtrOfTonemapDurand {
	#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

