pub type PtrOfStitcher = core::Ptr<crate::stitching::Stitcher>;

ptr_extern! { crate::stitching::Stitcher,
	cv_PtrOfStitcher_delete, cv_PtrOfStitcher_get_inner_ptr, cv_PtrOfStitcher_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Stitcher, cv_PtrOfStitcher_new }

impl PtrOfStitcher {
	#[inline] pub fn as_raw_PtrOfStitcher(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStitcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::StitcherTraitConst for PtrOfStitcher {
	#[inline] fn as_raw_Stitcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::StitcherTrait for PtrOfStitcher {
	#[inline] fn as_raw_mut_Stitcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

