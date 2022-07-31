pub type PtrOfPose3D = core::Ptr<crate::surface_matching::Pose3D>;

ptr_extern! { crate::surface_matching::Pose3D,
	cv_PtrOfPose3D_delete, cv_PtrOfPose3D_get_inner_ptr, cv_PtrOfPose3D_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::surface_matching::Pose3D, cv_PtrOfPose3D_new }

impl PtrOfPose3D {
	#[inline] pub fn as_raw_PtrOfPose3D(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPose3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::surface_matching::Pose3DTraitConst for PtrOfPose3D {
	#[inline] fn as_raw_Pose3D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::surface_matching::Pose3DTrait for PtrOfPose3D {
	#[inline] fn as_raw_mut_Pose3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

