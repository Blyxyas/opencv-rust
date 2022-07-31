pub type PtrOfDeblurerBase = core::Ptr<dyn crate::videostab::DeblurerBase>;

ptr_extern! { dyn crate::videostab::DeblurerBase,
	cv_PtrOfDeblurerBase_delete, cv_PtrOfDeblurerBase_get_inner_ptr, cv_PtrOfDeblurerBase_get_inner_ptr_mut
}

impl PtrOfDeblurerBase {
	#[inline] pub fn as_raw_PtrOfDeblurerBase(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDeblurerBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::DeblurerBaseConst for PtrOfDeblurerBase {
	#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::DeblurerBase for PtrOfDeblurerBase {
	#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

