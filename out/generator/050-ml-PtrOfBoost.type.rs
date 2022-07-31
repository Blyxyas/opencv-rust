pub type PtrOfBoost = core::Ptr<dyn crate::ml::Boost>;

ptr_extern! { dyn crate::ml::Boost,
	cv_PtrOfBoost_delete, cv_PtrOfBoost_get_inner_ptr, cv_PtrOfBoost_get_inner_ptr_mut
}

impl PtrOfBoost {
	#[inline] pub fn as_raw_PtrOfBoost(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBoost(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::BoostConst for PtrOfBoost {
	#[inline] fn as_raw_Boost(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::Boost for PtrOfBoost {
	#[inline] fn as_raw_mut_Boost(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBoost {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBoost {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ml::DTreesConst for PtrOfBoost {
	#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::DTrees for PtrOfBoost {
	#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ml::StatModelConst for PtrOfBoost {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModel for PtrOfBoost {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

