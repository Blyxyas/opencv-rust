pub type PtrOfPCAPrior = core::Ptr<crate::optflow::PCAPrior>;

ptr_extern! { crate::optflow::PCAPrior,
	cv_PtrOfPCAPrior_delete, cv_PtrOfPCAPrior_get_inner_ptr, cv_PtrOfPCAPrior_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::optflow::PCAPrior, cv_PtrOfPCAPrior_new }

impl PtrOfPCAPrior {
	#[inline] pub fn as_raw_PtrOfPCAPrior(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPCAPrior(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::optflow::PCAPriorTraitConst for PtrOfPCAPrior {
	#[inline] fn as_raw_PCAPrior(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::PCAPriorTrait for PtrOfPCAPrior {
	#[inline] fn as_raw_mut_PCAPrior(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

