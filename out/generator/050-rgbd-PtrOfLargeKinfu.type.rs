pub type PtrOfLargeKinfu = core::Ptr<dyn crate::rgbd::LargeKinfu>;

ptr_extern! { dyn crate::rgbd::LargeKinfu,
	cv_PtrOfLargeKinfu_delete, cv_PtrOfLargeKinfu_get_inner_ptr, cv_PtrOfLargeKinfu_get_inner_ptr_mut
}

impl PtrOfLargeKinfu {
	#[inline] pub fn as_raw_PtrOfLargeKinfu(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLargeKinfu(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::LargeKinfuConst for PtrOfLargeKinfu {
	#[inline] fn as_raw_LargeKinfu(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::LargeKinfu for PtrOfLargeKinfu {
	#[inline] fn as_raw_mut_LargeKinfu(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

