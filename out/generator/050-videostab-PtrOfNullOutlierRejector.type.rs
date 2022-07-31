pub type PtrOfNullOutlierRejector = core::Ptr<crate::videostab::NullOutlierRejector>;

ptr_extern! { crate::videostab::NullOutlierRejector,
	cv_PtrOfNullOutlierRejector_delete, cv_PtrOfNullOutlierRejector_get_inner_ptr, cv_PtrOfNullOutlierRejector_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::NullOutlierRejector, cv_PtrOfNullOutlierRejector_new }

impl PtrOfNullOutlierRejector {
	#[inline] pub fn as_raw_PtrOfNullOutlierRejector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNullOutlierRejector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::NullOutlierRejectorTraitConst for PtrOfNullOutlierRejector {
	#[inline] fn as_raw_NullOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::NullOutlierRejectorTrait for PtrOfNullOutlierRejector {
	#[inline] fn as_raw_mut_NullOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IOutlierRejectorConst for PtrOfNullOutlierRejector {
	#[inline] fn as_raw_IOutlierRejector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IOutlierRejector for PtrOfNullOutlierRejector {
	#[inline] fn as_raw_mut_IOutlierRejector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfNullOutlierRejector, core::Ptr<dyn crate::videostab::IOutlierRejector>,
	cv_PtrOfNullOutlierRejector_to_PtrOfIOutlierRejector,
}

