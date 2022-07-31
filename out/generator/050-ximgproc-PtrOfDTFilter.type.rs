pub type PtrOfDTFilter = core::Ptr<dyn crate::ximgproc::DTFilter>;

ptr_extern! { dyn crate::ximgproc::DTFilter,
	cv_PtrOfDTFilter_delete, cv_PtrOfDTFilter_get_inner_ptr, cv_PtrOfDTFilter_get_inner_ptr_mut
}

impl PtrOfDTFilter {
	#[inline] pub fn as_raw_PtrOfDTFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDTFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::DTFilterConst for PtrOfDTFilter {
	#[inline] fn as_raw_DTFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::DTFilter for PtrOfDTFilter {
	#[inline] fn as_raw_mut_DTFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDTFilter {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDTFilter {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

