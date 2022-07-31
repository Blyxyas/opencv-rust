pub type PtrOfLMSolver = core::Ptr<dyn crate::calib3d::LMSolver>;

ptr_extern! { dyn crate::calib3d::LMSolver,
	cv_PtrOfLMSolver_delete, cv_PtrOfLMSolver_get_inner_ptr, cv_PtrOfLMSolver_get_inner_ptr_mut
}

impl PtrOfLMSolver {
	#[inline] pub fn as_raw_PtrOfLMSolver(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLMSolver(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::calib3d::LMSolverConst for PtrOfLMSolver {
	#[inline] fn as_raw_LMSolver(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::LMSolver for PtrOfLMSolver {
	#[inline] fn as_raw_mut_LMSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfLMSolver {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfLMSolver {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

