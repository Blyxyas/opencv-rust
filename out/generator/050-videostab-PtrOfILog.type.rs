pub type PtrOfILog = core::Ptr<dyn crate::videostab::ILog>;

ptr_extern! { dyn crate::videostab::ILog,
	cv_PtrOfILog_delete, cv_PtrOfILog_get_inner_ptr, cv_PtrOfILog_get_inner_ptr_mut
}

impl PtrOfILog {
	#[inline] pub fn as_raw_PtrOfILog(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfILog(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::ILogConst for PtrOfILog {
	#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ILog for PtrOfILog {
	#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

