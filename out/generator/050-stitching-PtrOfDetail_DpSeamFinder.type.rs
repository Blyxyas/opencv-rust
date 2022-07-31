pub type PtrOfDetail_DpSeamFinder = core::Ptr<crate::stitching::Detail_DpSeamFinder>;

ptr_extern! { crate::stitching::Detail_DpSeamFinder,
	cv_PtrOfDetail_DpSeamFinder_delete, cv_PtrOfDetail_DpSeamFinder_get_inner_ptr, cv_PtrOfDetail_DpSeamFinder_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_DpSeamFinder, cv_PtrOfDetail_DpSeamFinder_new }

impl PtrOfDetail_DpSeamFinder {
	#[inline] pub fn as_raw_PtrOfDetail_DpSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_DpSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_DpSeamFinderTraitConst for PtrOfDetail_DpSeamFinder {
	#[inline] fn as_raw_Detail_DpSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_DpSeamFinderTrait for PtrOfDetail_DpSeamFinder {
	#[inline] fn as_raw_mut_Detail_DpSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinderConst for PtrOfDetail_DpSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SeamFinder for PtrOfDetail_DpSeamFinder {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_DpSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>,
	cv_PtrOfDetail_DpSeamFinder_to_PtrOfDetail_SeamFinder,
}

