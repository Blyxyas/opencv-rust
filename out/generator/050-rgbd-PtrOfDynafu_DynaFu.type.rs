pub type PtrOfDynafu_DynaFu = core::Ptr<dyn crate::rgbd::Dynafu_DynaFu>;

ptr_extern! { dyn crate::rgbd::Dynafu_DynaFu,
	cv_PtrOfDynafu_DynaFu_delete, cv_PtrOfDynafu_DynaFu_get_inner_ptr, cv_PtrOfDynafu_DynaFu_get_inner_ptr_mut
}

impl PtrOfDynafu_DynaFu {
	#[inline] pub fn as_raw_PtrOfDynafu_DynaFu(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDynafu_DynaFu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Dynafu_DynaFuConst for PtrOfDynafu_DynaFu {
	#[inline] fn as_raw_Dynafu_DynaFu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Dynafu_DynaFu for PtrOfDynafu_DynaFu {
	#[inline] fn as_raw_mut_Dynafu_DynaFu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

