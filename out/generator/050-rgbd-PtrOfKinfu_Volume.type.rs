pub type PtrOfKinfu_Volume = core::Ptr<dyn crate::rgbd::Kinfu_Volume>;

ptr_extern! { dyn crate::rgbd::Kinfu_Volume,
	cv_PtrOfKinfu_Volume_delete, cv_PtrOfKinfu_Volume_get_inner_ptr, cv_PtrOfKinfu_Volume_get_inner_ptr_mut
}

impl PtrOfKinfu_Volume {
	#[inline] pub fn as_raw_PtrOfKinfu_Volume(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKinfu_Volume(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Kinfu_VolumeConst for PtrOfKinfu_Volume {
	#[inline] fn as_raw_Kinfu_Volume(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Kinfu_Volume for PtrOfKinfu_Volume {
	#[inline] fn as_raw_mut_Kinfu_Volume(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

