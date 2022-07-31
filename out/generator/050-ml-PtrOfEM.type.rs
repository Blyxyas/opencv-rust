pub type PtrOfEM = core::Ptr<dyn crate::ml::EM>;

ptr_extern! { dyn crate::ml::EM,
	cv_PtrOfEM_delete, cv_PtrOfEM_get_inner_ptr, cv_PtrOfEM_get_inner_ptr_mut
}

impl PtrOfEM {
	#[inline] pub fn as_raw_PtrOfEM(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::EMConst for PtrOfEM {
	#[inline] fn as_raw_EM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::EM for PtrOfEM {
	#[inline] fn as_raw_mut_EM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfEM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfEM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ml::StatModelConst for PtrOfEM {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModel for PtrOfEM {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

