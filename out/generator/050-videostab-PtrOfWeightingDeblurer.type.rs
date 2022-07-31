pub type PtrOfWeightingDeblurer = core::Ptr<crate::videostab::WeightingDeblurer>;

ptr_extern! { crate::videostab::WeightingDeblurer,
	cv_PtrOfWeightingDeblurer_delete, cv_PtrOfWeightingDeblurer_get_inner_ptr, cv_PtrOfWeightingDeblurer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::WeightingDeblurer, cv_PtrOfWeightingDeblurer_new }

impl PtrOfWeightingDeblurer {
	#[inline] pub fn as_raw_PtrOfWeightingDeblurer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfWeightingDeblurer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::WeightingDeblurerTraitConst for PtrOfWeightingDeblurer {
	#[inline] fn as_raw_WeightingDeblurer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::WeightingDeblurerTrait for PtrOfWeightingDeblurer {
	#[inline] fn as_raw_mut_WeightingDeblurer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::DeblurerBaseConst for PtrOfWeightingDeblurer {
	#[inline] fn as_raw_DeblurerBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::DeblurerBase for PtrOfWeightingDeblurer {
	#[inline] fn as_raw_mut_DeblurerBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfWeightingDeblurer, core::Ptr<dyn crate::videostab::DeblurerBase>,
	cv_PtrOfWeightingDeblurer_to_PtrOfDeblurerBase,
}

