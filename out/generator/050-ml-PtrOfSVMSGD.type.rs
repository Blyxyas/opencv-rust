pub type PtrOfSVMSGD = core::Ptr<dyn crate::ml::SVMSGD>;

ptr_extern! { dyn crate::ml::SVMSGD,
	cv_PtrOfSVMSGD_delete, cv_PtrOfSVMSGD_get_inner_ptr, cv_PtrOfSVMSGD_get_inner_ptr_mut
}

impl PtrOfSVMSGD {
	#[inline] pub fn as_raw_PtrOfSVMSGD(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSVMSGD(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::SVMSGDConst for PtrOfSVMSGD {
	#[inline] fn as_raw_SVMSGD(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::SVMSGD for PtrOfSVMSGD {
	#[inline] fn as_raw_mut_SVMSGD(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSVMSGD {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSVMSGD {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ml::StatModelConst for PtrOfSVMSGD {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModel for PtrOfSVMSGD {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

