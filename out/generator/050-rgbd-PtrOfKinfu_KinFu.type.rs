pub type PtrOfKinfu_KinFu = core::Ptr<dyn crate::rgbd::Kinfu_KinFu>;

ptr_extern! { dyn crate::rgbd::Kinfu_KinFu,
	cv_PtrOfKinfu_KinFu_delete, cv_PtrOfKinfu_KinFu_get_inner_ptr, cv_PtrOfKinfu_KinFu_get_inner_ptr_mut
}

impl PtrOfKinfu_KinFu {
	#[inline] pub fn as_raw_PtrOfKinfu_KinFu(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKinfu_KinFu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Kinfu_KinFuConst for PtrOfKinfu_KinFu {
	#[inline] fn as_raw_Kinfu_KinFu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Kinfu_KinFu for PtrOfKinfu_KinFu {
	#[inline] fn as_raw_mut_Kinfu_KinFu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

