pub type PtrOfKinfu_Detail_PoseGraph = core::Ptr<dyn crate::rgbd::Kinfu_Detail_PoseGraph>;

ptr_extern! { dyn crate::rgbd::Kinfu_Detail_PoseGraph,
	cv_PtrOfKinfu_Detail_PoseGraph_delete, cv_PtrOfKinfu_Detail_PoseGraph_get_inner_ptr, cv_PtrOfKinfu_Detail_PoseGraph_get_inner_ptr_mut
}

impl PtrOfKinfu_Detail_PoseGraph {
	#[inline] pub fn as_raw_PtrOfKinfu_Detail_PoseGraph(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKinfu_Detail_PoseGraph(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Kinfu_Detail_PoseGraphConst for PtrOfKinfu_Detail_PoseGraph {
	#[inline] fn as_raw_Kinfu_Detail_PoseGraph(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Kinfu_Detail_PoseGraph for PtrOfKinfu_Detail_PoseGraph {
	#[inline] fn as_raw_mut_Kinfu_Detail_PoseGraph(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

