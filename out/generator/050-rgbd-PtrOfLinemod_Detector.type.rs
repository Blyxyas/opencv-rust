pub type PtrOfLinemod_Detector = core::Ptr<crate::rgbd::Linemod_Detector>;

ptr_extern! { crate::rgbd::Linemod_Detector,
	cv_PtrOfLinemod_Detector_delete, cv_PtrOfLinemod_Detector_get_inner_ptr, cv_PtrOfLinemod_Detector_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::Linemod_Detector, cv_PtrOfLinemod_Detector_new }

impl PtrOfLinemod_Detector {
	#[inline] pub fn as_raw_PtrOfLinemod_Detector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLinemod_Detector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Linemod_DetectorTraitConst for PtrOfLinemod_Detector {
	#[inline] fn as_raw_Linemod_Detector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Linemod_DetectorTrait for PtrOfLinemod_Detector {
	#[inline] fn as_raw_mut_Linemod_Detector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

