pub type PtrOfNullDeblurer = core::Ptr<crate::videostab::NullDeblurer>;

ptr_extern! { crate::videostab::NullDeblurer,
	cv_PtrOfNullDeblurer_delete, cv_PtrOfNullDeblurer_get_inner_ptr, cv_PtrOfNullDeblurer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::NullDeblurer, cv_PtrOfNullDeblurer_new }

impl PtrOfNullDeblurer {
	#[inline] pub fn as_raw_PtrOfNullDeblurer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNullDeblurer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::NullDeblurerTraitConst for PtrOfNullDeblurer {
	#[inline] fn as_raw_NullDeblurer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::NullDeblurerTrait for PtrOfNullDeblurer {
	#[inline] fn as_raw_mut_NullDeblurer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::DeblurerBaseConst for PtrOfNullDeblurer {
	#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::DeblurerBase for PtrOfNullDeblurer {
	#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfNullDeblurer, core::Ptr<dyn crate::videostab::DeblurerBase>,
	cv_PtrOfNullDeblurer_to_PtrOfDeblurerBase,
}

