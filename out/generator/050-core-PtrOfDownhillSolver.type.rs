pub type PtrOfDownhillSolver = core::Ptr<dyn core::DownhillSolver>;

ptr_extern! { dyn core::DownhillSolver,
	cv_PtrOfDownhillSolver_delete, cv_PtrOfDownhillSolver_get_inner_ptr, cv_PtrOfDownhillSolver_get_inner_ptr_mut
}

impl PtrOfDownhillSolver {
	#[inline] pub fn as_raw_PtrOfDownhillSolver(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDownhillSolver(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::DownhillSolverConst for PtrOfDownhillSolver {
	#[inline] fn as_raw_DownhillSolver(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::DownhillSolver for PtrOfDownhillSolver {
	#[inline] fn as_raw_mut_DownhillSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDownhillSolver {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDownhillSolver {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::MinProblemSolverConst for PtrOfDownhillSolver {
	#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::MinProblemSolver for PtrOfDownhillSolver {
	#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

