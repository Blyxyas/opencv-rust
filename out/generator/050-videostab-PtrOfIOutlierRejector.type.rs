pub type PtrOfIOutlierRejector = core::Ptr<dyn crate::videostab::IOutlierRejector>;

ptr_extern! { dyn crate::videostab::IOutlierRejector,
	cv_PtrOfIOutlierRejector_delete, cv_PtrOfIOutlierRejector_get_inner_ptr, cv_PtrOfIOutlierRejector_get_inner_ptr_mut
}

impl PtrOfIOutlierRejector {
	#[inline] pub fn as_raw_PtrOfIOutlierRejector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfIOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::IOutlierRejectorConst for PtrOfIOutlierRejector {
	#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IOutlierRejector for PtrOfIOutlierRejector {
	#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

