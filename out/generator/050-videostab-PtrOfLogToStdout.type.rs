pub type PtrOfLogToStdout = core::Ptr<crate::videostab::LogToStdout>;

ptr_extern! { crate::videostab::LogToStdout,
	cv_PtrOfLogToStdout_delete, cv_PtrOfLogToStdout_get_inner_ptr, cv_PtrOfLogToStdout_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::LogToStdout, cv_PtrOfLogToStdout_new }

impl PtrOfLogToStdout {
	#[inline] pub fn as_raw_PtrOfLogToStdout(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLogToStdout(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::LogToStdoutTraitConst for PtrOfLogToStdout {
	#[inline] fn as_raw_LogToStdout(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::LogToStdoutTrait for PtrOfLogToStdout {
	#[inline] fn as_raw_mut_LogToStdout(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ILogConst for PtrOfLogToStdout {
	#[inline] fn as_raw_ILog(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ILog for PtrOfLogToStdout {
	#[inline] fn as_raw_mut_ILog(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfLogToStdout, core::Ptr<dyn crate::videostab::ILog>,
	cv_PtrOfLogToStdout_to_PtrOfILog,
}

