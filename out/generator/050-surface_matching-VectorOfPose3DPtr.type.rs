pub type VectorOfPose3DPtr = core::Vector<crate::surface_matching::Pose3DPtr>;

impl VectorOfPose3DPtr {
	pub fn as_raw_VectorOfPose3DPtr(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfPose3DPtr(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::surface_matching::Pose3DPtr, *const c_void, *mut c_void,
	cv_VectorOfPose3DPtr_new, cv_VectorOfPose3DPtr_delete,
	cv_VectorOfPose3DPtr_len, cv_VectorOfPose3DPtr_is_empty,
	cv_VectorOfPose3DPtr_capacity, cv_VectorOfPose3DPtr_resize,
	cv_VectorOfPose3DPtr_shrink_to_fit,
	cv_VectorOfPose3DPtr_reserve, cv_VectorOfPose3DPtr_remove,
	cv_VectorOfPose3DPtr_swap, cv_VectorOfPose3DPtr_clear,
	cv_VectorOfPose3DPtr_get, cv_VectorOfPose3DPtr_set,
	cv_VectorOfPose3DPtr_push, cv_VectorOfPose3DPtr_insert,
}
vector_non_copy_or_bool! { crate::surface_matching::Pose3DPtr }

unsafe impl Send for core::Vector<crate::surface_matching::Pose3DPtr> {}

