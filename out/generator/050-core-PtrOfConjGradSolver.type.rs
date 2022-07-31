pub type PtrOfConjGradSolver = core::Ptr<dyn core::ConjGradSolver>;

ptr_extern! { dyn core::ConjGradSolver,
	cv_PtrOfConjGradSolver_delete, cv_PtrOfConjGradSolver_get_inner_ptr, cv_PtrOfConjGradSolver_get_inner_ptr_mut
}

impl PtrOfConjGradSolver {
	#[inline] pub fn as_raw_PtrOfConjGradSolver(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfConjGradSolver(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::ConjGradSolverConst for PtrOfConjGradSolver {
	#[inline] fn as_raw_ConjGradSolver(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::ConjGradSolver for PtrOfConjGradSolver {
	#[inline] fn as_raw_mut_ConjGradSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfConjGradSolver {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfConjGradSolver {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::MinProblemSolverConst for PtrOfConjGradSolver {
	#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::MinProblemSolver for PtrOfConjGradSolver {
	#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

