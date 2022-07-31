pub type PtrOfRgbdNormals = core::Ptr<crate::rgbd::RgbdNormals>;

ptr_extern! { crate::rgbd::RgbdNormals,
	cv_PtrOfRgbdNormals_delete, cv_PtrOfRgbdNormals_get_inner_ptr, cv_PtrOfRgbdNormals_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::RgbdNormals, cv_PtrOfRgbdNormals_new }

impl PtrOfRgbdNormals {
	#[inline] pub fn as_raw_PtrOfRgbdNormals(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRgbdNormals(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdNormalsTraitConst for PtrOfRgbdNormals {
	#[inline] fn as_raw_RgbdNormals(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::RgbdNormalsTrait for PtrOfRgbdNormals {
	#[inline] fn as_raw_mut_RgbdNormals(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRgbdNormals {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRgbdNormals {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

