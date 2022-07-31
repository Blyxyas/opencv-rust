pub type PtrOfVGG = core::Ptr<dyn crate::xfeatures2d::VGG>;

ptr_extern! { dyn crate::xfeatures2d::VGG,
	cv_PtrOfVGG_delete, cv_PtrOfVGG_get_inner_ptr, cv_PtrOfVGG_get_inner_ptr_mut
}

impl PtrOfVGG {
	#[inline] pub fn as_raw_PtrOfVGG(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfVGG(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::VGGConst for PtrOfVGG {
	#[inline] fn as_raw_VGG(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::VGG for PtrOfVGG {
	#[inline] fn as_raw_mut_VGG(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfVGG {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfVGG {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for PtrOfVGG {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for PtrOfVGG {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfVGG, core::Ptr<crate::features2d::Feature2D>,
	cv_PtrOfVGG_to_PtrOfFeature2D,
}

