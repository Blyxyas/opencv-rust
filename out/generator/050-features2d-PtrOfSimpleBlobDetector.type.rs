pub type PtrOfSimpleBlobDetector = core::Ptr<crate::features2d::SimpleBlobDetector>;

ptr_extern! { crate::features2d::SimpleBlobDetector,
	cv_PtrOfSimpleBlobDetector_delete, cv_PtrOfSimpleBlobDetector_get_inner_ptr, cv_PtrOfSimpleBlobDetector_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::features2d::SimpleBlobDetector, cv_PtrOfSimpleBlobDetector_new }

impl PtrOfSimpleBlobDetector {
	#[inline] pub fn as_raw_PtrOfSimpleBlobDetector(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSimpleBlobDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::SimpleBlobDetectorTraitConst for PtrOfSimpleBlobDetector {
	#[inline] fn as_raw_SimpleBlobDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::SimpleBlobDetectorTrait for PtrOfSimpleBlobDetector {
	#[inline] fn as_raw_mut_SimpleBlobDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSimpleBlobDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSimpleBlobDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfSimpleBlobDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfSimpleBlobDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfSimpleBlobDetector, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfSimpleBlobDetector_to_PtrOfFeature2D,
}

