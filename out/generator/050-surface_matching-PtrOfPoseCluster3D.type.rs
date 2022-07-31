pub type PtrOfPoseCluster3D = core::Ptr<crate::surface_matching::PoseCluster3D>;

ptr_extern! { crate::surface_matching::PoseCluster3D,
	cv_PtrOfPoseCluster3D_delete, cv_PtrOfPoseCluster3D_get_inner_ptr, cv_PtrOfPoseCluster3D_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::surface_matching::PoseCluster3D, cv_PtrOfPoseCluster3D_new }

impl PtrOfPoseCluster3D {
	#[inline] pub fn as_raw_PtrOfPoseCluster3D(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPoseCluster3D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::surface_matching::PoseCluster3DTraitConst for PtrOfPoseCluster3D {
	#[inline] fn as_raw_PoseCluster3D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::surface_matching::PoseCluster3DTrait for PtrOfPoseCluster3D {
	#[inline] fn as_raw_mut_PoseCluster3D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

