pub type PtrOfSVM = core::Ptr<dyn crate::ml::SVM>;

ptr_extern! { dyn crate::ml::SVM,
	cv_PtrOfSVM_delete, cv_PtrOfSVM_get_inner_ptr, cv_PtrOfSVM_get_inner_ptr_mut
}

impl PtrOfSVM {
	#[inline] pub fn as_raw_PtrOfSVM(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSVM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::SVMConst for PtrOfSVM {
	#[inline] fn as_raw_SVM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::SVM for PtrOfSVM {
	#[inline] fn as_raw_mut_SVM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSVM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSVM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ml::StatModelConst for PtrOfSVM {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModel for PtrOfSVM {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

