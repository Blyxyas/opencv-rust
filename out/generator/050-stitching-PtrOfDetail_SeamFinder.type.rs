pub type PtrOfDetail_SeamFinder = core::Ptr<dyn crate::stitching::Detail_SeamFinder>;

ptr_extern! { dyn crate::stitching::Detail_SeamFinder,
	cv_PtrOfDetail_SeamFinder_delete, cv_PtrOfDetail_SeamFinder_get_inner_ptr, cv_PtrOfDetail_SeamFinder_get_inner_ptr_mut
}

impl PtrOfDetail_SeamFinder {
	#[inline] pub fn as_raw_PtrOfDetail_SeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_SeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinderConst for PtrOfDetail_SeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SeamFinder for PtrOfDetail_SeamFinder {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

