pub type PtrOfNullLog = core::Ptr<crate::videostab::NullLog>;

ptr_extern! { crate::videostab::NullLog,
	cv_PtrOfNullLog_delete, cv_PtrOfNullLog_get_inner_ptr, cv_PtrOfNullLog_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::NullLog, cv_PtrOfNullLog_new }

impl PtrOfNullLog {
	#[inline] pub fn as_raw_PtrOfNullLog(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNullLog(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::NullLogTraitConst for PtrOfNullLog {
	#[inline] fn as_raw_NullLog(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::NullLogTrait for PtrOfNullLog {
	#[inline] fn as_raw_mut_NullLog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ILogConst for PtrOfNullLog {
	#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ILog for PtrOfNullLog {
	#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfNullLog, core::Ptr<dyn crate::videostab::ILog>,
	cv_PtrOfNullLog_to_PtrOfILog,
}

