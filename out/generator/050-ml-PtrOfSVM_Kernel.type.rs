pub type PtrOfSVM_Kernel = core::Ptr<dyn crate::ml::SVM_Kernel>;

ptr_extern! { dyn crate::ml::SVM_Kernel,
	cv_PtrOfSVM_Kernel_delete, cv_PtrOfSVM_Kernel_get_inner_ptr, cv_PtrOfSVM_Kernel_get_inner_ptr_mut
}

impl PtrOfSVM_Kernel {
	#[inline] pub fn as_raw_PtrOfSVM_Kernel(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSVM_Kernel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::SVM_KernelConst for PtrOfSVM_Kernel {
	#[inline] fn as_raw_SVM_Kernel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::SVM_Kernel for PtrOfSVM_Kernel {
	#[inline] fn as_raw_mut_SVM_Kernel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSVM_Kernel {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSVM_Kernel {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

