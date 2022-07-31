pub type PtrOfRTrees = core::Ptr<dyn crate::ml::RTrees>;

ptr_extern! { dyn crate::ml::RTrees,
	cv_PtrOfRTrees_delete, cv_PtrOfRTrees_get_inner_ptr, cv_PtrOfRTrees_get_inner_ptr_mut
}

impl PtrOfRTrees {
	#[inline] pub fn as_raw_PtrOfRTrees(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRTrees(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::RTreesConst for PtrOfRTrees {
	#[inline] fn as_raw_RTrees(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::RTrees for PtrOfRTrees {
	#[inline] fn as_raw_mut_RTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRTrees {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRTrees {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ml::DTreesConst for PtrOfRTrees {
	#[inline] fn as_raw_DTrees(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::DTrees for PtrOfRTrees {
	#[inline] fn as_raw_mut_DTrees(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ml::StatModelConst for PtrOfRTrees {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModel for PtrOfRTrees {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

