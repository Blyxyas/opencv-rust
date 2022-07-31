pub type PtrOfLMSolver_Callback = core::Ptr<dyn crate::calib3d::LMSolver_Callback>;

ptr_extern! { dyn crate::calib3d::LMSolver_Callback,
	cv_PtrOfLMSolver_Callback_delete, cv_PtrOfLMSolver_Callback_get_inner_ptr, cv_PtrOfLMSolver_Callback_get_inner_ptr_mut
}

impl PtrOfLMSolver_Callback {
	#[inline] pub fn as_raw_PtrOfLMSolver_Callback(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLMSolver_Callback(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::calib3d::LMSolver_CallbackConst for PtrOfLMSolver_Callback {
	#[inline] fn as_raw_LMSolver_Callback(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::LMSolver_Callback for PtrOfLMSolver_Callback {
	#[inline] fn as_raw_mut_LMSolver_Callback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

