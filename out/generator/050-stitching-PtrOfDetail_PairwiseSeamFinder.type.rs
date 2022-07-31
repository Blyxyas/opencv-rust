pub type PtrOfDetail_PairwiseSeamFinder = core::Ptr<dyn crate::stitching::Detail_PairwiseSeamFinder>;

ptr_extern! { dyn crate::stitching::Detail_PairwiseSeamFinder,
	cv_PtrOfDetail_PairwiseSeamFinder_delete, cv_PtrOfDetail_PairwiseSeamFinder_get_inner_ptr, cv_PtrOfDetail_PairwiseSeamFinder_get_inner_ptr_mut
}

impl PtrOfDetail_PairwiseSeamFinder {
	#[inline] pub fn as_raw_PtrOfDetail_PairwiseSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_PairwiseSeamFinderConst for PtrOfDetail_PairwiseSeamFinder {
	#[inline] fn as_raw_Detail_PairwiseSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_PairwiseSeamFinder for PtrOfDetail_PairwiseSeamFinder {
	#[inline] fn as_raw_mut_Detail_PairwiseSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinderConst for PtrOfDetail_PairwiseSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SeamFinder for PtrOfDetail_PairwiseSeamFinder {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_PairwiseSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>,
	cv_PtrOfDetail_PairwiseSeamFinder_to_PtrOfDetail_SeamFinder,
}

