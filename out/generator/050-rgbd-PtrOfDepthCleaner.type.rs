pub type PtrOfDepthCleaner = core::Ptr<crate::rgbd::DepthCleaner>;

ptr_extern! { crate::rgbd::DepthCleaner,
	cv_PtrOfDepthCleaner_delete, cv_PtrOfDepthCleaner_get_inner_ptr, cv_PtrOfDepthCleaner_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::DepthCleaner, cv_PtrOfDepthCleaner_new }

impl PtrOfDepthCleaner {
	#[inline] pub fn as_raw_PtrOfDepthCleaner(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDepthCleaner(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::DepthCleanerTraitConst for PtrOfDepthCleaner {
	#[inline] fn as_raw_DepthCleaner(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::DepthCleanerTrait for PtrOfDepthCleaner {
	#[inline] fn as_raw_mut_DepthCleaner(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDepthCleaner {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDepthCleaner {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

