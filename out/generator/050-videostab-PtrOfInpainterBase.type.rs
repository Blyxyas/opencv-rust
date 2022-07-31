pub type PtrOfInpainterBase = core::Ptr<dyn crate::videostab::InpainterBase>;

ptr_extern! { dyn crate::videostab::InpainterBase,
	cv_PtrOfInpainterBase_delete, cv_PtrOfInpainterBase_get_inner_ptr, cv_PtrOfInpainterBase_get_inner_ptr_mut
}

impl PtrOfInpainterBase {
	#[inline] pub fn as_raw_PtrOfInpainterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfInpainterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::InpainterBaseConst for PtrOfInpainterBase {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBase for PtrOfInpainterBase {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

