pub type PtrOfCLAHE = core::Ptr<dyn crate::imgproc::CLAHE>;

ptr_extern! { dyn crate::imgproc::CLAHE,
	cv_PtrOfCLAHE_delete, cv_PtrOfCLAHE_get_inner_ptr, cv_PtrOfCLAHE_get_inner_ptr_mut
}

impl PtrOfCLAHE {
	#[inline] pub fn as_raw_PtrOfCLAHE(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCLAHE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::imgproc::CLAHEConst for PtrOfCLAHE {
	#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::imgproc::CLAHE for PtrOfCLAHE {
	#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfCLAHE {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfCLAHE {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

