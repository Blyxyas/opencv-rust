pub type PtrOfKinfu_VolumeParams = core::Ptr<crate::rgbd::Kinfu_VolumeParams>;

ptr_extern! { crate::rgbd::Kinfu_VolumeParams,
	cv_PtrOfKinfu_VolumeParams_delete, cv_PtrOfKinfu_VolumeParams_get_inner_ptr, cv_PtrOfKinfu_VolumeParams_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::Kinfu_VolumeParams, cv_PtrOfKinfu_VolumeParams_new }

impl PtrOfKinfu_VolumeParams {
	#[inline] pub fn as_raw_PtrOfKinfu_VolumeParams(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKinfu_VolumeParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Kinfu_VolumeParamsTraitConst for PtrOfKinfu_VolumeParams {
	#[inline] fn as_raw_Kinfu_VolumeParams(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Kinfu_VolumeParamsTrait for PtrOfKinfu_VolumeParams {
	#[inline] fn as_raw_mut_Kinfu_VolumeParams(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

