pub type PtrOfSURF_CUDA = core::Ptr<crate::xfeatures2d::SURF_CUDA>;

ptr_extern! { crate::xfeatures2d::SURF_CUDA,
	cv_PtrOfSURF_CUDA_delete, cv_PtrOfSURF_CUDA_get_inner_ptr, cv_PtrOfSURF_CUDA_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::xfeatures2d::SURF_CUDA, cv_PtrOfSURF_CUDA_new }

impl PtrOfSURF_CUDA {
	#[inline] pub fn as_raw_PtrOfSURF_CUDA(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSURF_CUDA(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::SURF_CUDATraitConst for PtrOfSURF_CUDA {
	#[inline] fn as_raw_SURF_CUDA(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::SURF_CUDATrait for PtrOfSURF_CUDA {
	#[inline] fn as_raw_mut_SURF_CUDA(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

