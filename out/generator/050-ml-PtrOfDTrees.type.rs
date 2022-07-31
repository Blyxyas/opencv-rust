pub type PtrOfDTrees = core::Ptr<dyn crate::ml::DTrees>;

ptr_extern! { dyn crate::ml::DTrees,
	cv_PtrOfDTrees_delete, cv_PtrOfDTrees_get_inner_ptr, cv_PtrOfDTrees_get_inner_ptr_mut
}

impl PtrOfDTrees {
	#[inline] pub fn as_raw_PtrOfDTrees(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDTrees(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::DTreesConst for PtrOfDTrees {
	#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::DTrees for PtrOfDTrees {
	#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDTrees {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDTrees {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ml::StatModelConst for PtrOfDTrees {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModel for PtrOfDTrees {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

