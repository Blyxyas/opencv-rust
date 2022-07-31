pub type PtrOfMinProblemSolver_Function = core::Ptr<dyn core::MinProblemSolver_Function>;

ptr_extern! { dyn core::MinProblemSolver_Function,
	cv_PtrOfMinProblemSolver_Function_delete, cv_PtrOfMinProblemSolver_Function_get_inner_ptr, cv_PtrOfMinProblemSolver_Function_get_inner_ptr_mut
}

impl PtrOfMinProblemSolver_Function {
	#[inline] pub fn as_raw_PtrOfMinProblemSolver_Function(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMinProblemSolver_Function(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::MinProblemSolver_FunctionConst for PtrOfMinProblemSolver_Function {
	#[inline] fn as_raw_MinProblemSolver_Function(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::MinProblemSolver_Function for PtrOfMinProblemSolver_Function {
	#[inline] fn as_raw_mut_MinProblemSolver_Function(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

