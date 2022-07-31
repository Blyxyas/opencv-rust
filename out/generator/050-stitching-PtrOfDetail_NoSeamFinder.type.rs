pub type PtrOfDetail_NoSeamFinder = core::Ptr<crate::stitching::Detail_NoSeamFinder>;

ptr_extern! { crate::stitching::Detail_NoSeamFinder,
	cv_PtrOfDetail_NoSeamFinder_delete, cv_PtrOfDetail_NoSeamFinder_get_inner_ptr, cv_PtrOfDetail_NoSeamFinder_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_NoSeamFinder, cv_PtrOfDetail_NoSeamFinder_new }

impl PtrOfDetail_NoSeamFinder {
	#[inline] pub fn as_raw_PtrOfDetail_NoSeamFinder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_NoSeamFinder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_NoSeamFinderTraitConst for PtrOfDetail_NoSeamFinder {
	#[inline] fn as_raw_Detail_NoSeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_NoSeamFinderTrait for PtrOfDetail_NoSeamFinder {
	#[inline] fn as_raw_mut_Detail_NoSeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_SeamFinderConst for PtrOfDetail_NoSeamFinder {
	#[inline] fn as_raw_Detail_SeamFinder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_SeamFinder for PtrOfDetail_NoSeamFinder {
	#[inline] fn as_raw_mut_Detail_SeamFinder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_NoSeamFinder, core::Ptr<dyn crate::stitching::Detail_SeamFinder>,
	cv_PtrOfDetail_NoSeamFinder_to_PtrOfDetail_SeamFinder,
}

