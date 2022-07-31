pub type PtrOfFastBilateralSolverFilter = core::Ptr<dyn crate::ximgproc::FastBilateralSolverFilter>;

ptr_extern! { dyn crate::ximgproc::FastBilateralSolverFilter,
	cv_PtrOfFastBilateralSolverFilter_delete, cv_PtrOfFastBilateralSolverFilter_get_inner_ptr, cv_PtrOfFastBilateralSolverFilter_get_inner_ptr_mut
}

impl PtrOfFastBilateralSolverFilter {
	#[inline] pub fn as_raw_PtrOfFastBilateralSolverFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFastBilateralSolverFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::FastBilateralSolverFilterConst for PtrOfFastBilateralSolverFilter {
	#[inline] fn as_raw_FastBilateralSolverFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::FastBilateralSolverFilter for PtrOfFastBilateralSolverFilter {
	#[inline] fn as_raw_mut_FastBilateralSolverFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFastBilateralSolverFilter {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFastBilateralSolverFilter {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

