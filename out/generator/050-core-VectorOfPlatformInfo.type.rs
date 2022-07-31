pub type VectorOfPlatformInfo = core::Vector<core::PlatformInfo>;

impl VectorOfPlatformInfo {
	pub fn as_raw_VectorOfPlatformInfo(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfPlatformInfo(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::PlatformInfo, *const c_void, *mut c_void,
	cv_VectorOfPlatformInfo_new, cv_VectorOfPlatformInfo_delete,
	cv_VectorOfPlatformInfo_len, cv_VectorOfPlatformInfo_is_empty,
	cv_VectorOfPlatformInfo_capacity, cv_VectorOfPlatformInfo_resize,
	cv_VectorOfPlatformInfo_shrink_to_fit,
	cv_VectorOfPlatformInfo_reserve, cv_VectorOfPlatformInfo_remove,
	cv_VectorOfPlatformInfo_swap, cv_VectorOfPlatformInfo_clear,
	cv_VectorOfPlatformInfo_get, cv_VectorOfPlatformInfo_set,
	cv_VectorOfPlatformInfo_push, cv_VectorOfPlatformInfo_insert,
}
vector_non_copy_or_bool! { core::PlatformInfo }

unsafe impl Send for core::Vector<core::PlatformInfo> {}

