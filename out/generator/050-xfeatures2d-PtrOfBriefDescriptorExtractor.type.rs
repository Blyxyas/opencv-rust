pub type PtrOfBriefDescriptorExtractor = core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor>;

ptr_extern! { crate::xfeatures2d::BriefDescriptorExtractor,
	cv_PtrOfBriefDescriptorExtractor_delete, cv_PtrOfBriefDescriptorExtractor_get_inner_ptr, cv_PtrOfBriefDescriptorExtractor_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::xfeatures2d::BriefDescriptorExtractor, cv_PtrOfBriefDescriptorExtractor_new }

impl PtrOfBriefDescriptorExtractor {
	#[inline] pub fn as_raw_PtrOfBriefDescriptorExtractor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBriefDescriptorExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::BriefDescriptorExtractorTraitConst for PtrOfBriefDescriptorExtractor {
	#[inline] fn as_raw_BriefDescriptorExtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::BriefDescriptorExtractorTrait for PtrOfBriefDescriptorExtractor {
	#[inline] fn as_raw_mut_BriefDescriptorExtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBriefDescriptorExtractor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBriefDescriptorExtractor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfBriefDescriptorExtractor {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfBriefDescriptorExtractor {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfBriefDescriptorExtractor, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfBriefDescriptorExtractor_to_PtrOfFeature2D,
}

