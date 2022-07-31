pub type PtrOfColoredKinfu_ColoredKinFu = core::Ptr<dyn crate::rgbd::ColoredKinfu_ColoredKinFu>;

ptr_extern! { dyn crate::rgbd::ColoredKinfu_ColoredKinFu,
	cv_PtrOfColoredKinfu_ColoredKinFu_delete, cv_PtrOfColoredKinfu_ColoredKinFu_get_inner_ptr, cv_PtrOfColoredKinfu_ColoredKinFu_get_inner_ptr_mut
}

impl PtrOfColoredKinfu_ColoredKinFu {
	#[inline] pub fn as_raw_PtrOfColoredKinfu_ColoredKinFu(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfColoredKinfu_ColoredKinFu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::ColoredKinfu_ColoredKinFuConst for PtrOfColoredKinfu_ColoredKinFu {
	#[inline] fn as_raw_ColoredKinfu_ColoredKinFu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::ColoredKinfu_ColoredKinFu for PtrOfColoredKinfu_ColoredKinFu {
	#[inline] fn as_raw_mut_ColoredKinfu_ColoredKinFu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

