pub type PtrOfQuasiDenseStereo = core::Ptr<dyn crate::stereo::QuasiDenseStereo>;

ptr_extern! { dyn crate::stereo::QuasiDenseStereo,
	cv_PtrOfQuasiDenseStereo_delete, cv_PtrOfQuasiDenseStereo_get_inner_ptr, cv_PtrOfQuasiDenseStereo_get_inner_ptr_mut
}

impl PtrOfQuasiDenseStereo {
	#[inline] pub fn as_raw_PtrOfQuasiDenseStereo(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQuasiDenseStereo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stereo::QuasiDenseStereoConst for PtrOfQuasiDenseStereo {
	#[inline] fn as_raw_QuasiDenseStereo(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stereo::QuasiDenseStereo for PtrOfQuasiDenseStereo {
	#[inline] fn as_raw_mut_QuasiDenseStereo(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

