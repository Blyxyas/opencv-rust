pub type PtrOfDetail_GraphCutSeamFinder = core::Ptr<crate::stitching::Detail_GraphCutSeamFinder>;

ptr_extern! { crate::stitching::Detail_GraphCutSeamFinder,
	cv_PtrOfDetail_GraphCutSeamFinder_delete, cv_PtrOfDetail_GraphCutSeamFinder_get_inner_ptr, cv_PtrOfDetail_GraphCutSeamFinder_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_GraphCutSeamFinder, cv_PtrOfDetail_GraphCutSeamFinder_new }

impl PtrOfDetail_GraphCutSeamFinder {
	#[inline] pub fn as_raw_PtrOfDetail_GraphCutSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_GraphCutSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_GraphCutSeamFinderTraitConst for PtrOfDetail_GraphCutSeamFinder {
	#[inline] fn as_raw_Detail_GraphCutSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_GraphCutSeamFinderTrait for PtrOfDetail_GraphCutSeamFinder {
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTraitConst for PtrOfDetail_GraphCutSeamFinder {
	#[inline] fn as_raw_Detail_GraphCutSeamFinderBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_GraphCutSeamFinderBaseTrait for PtrOfDetail_GraphCutSeamFinder {
	#[inline] fn as_raw_mut_Detail_GraphCutSeamFinderBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinderConst for PtrOfDetail_GraphCutSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SeamFinder for PtrOfDetail_GraphCutSeamFinder {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_GraphCutSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>,
	cv_PtrOfDetail_GraphCutSeamFinder_to_PtrOfDetail_SeamFinder,
}

