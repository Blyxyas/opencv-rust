pub type PtrOfSplitLayer = core::Ptr<crate::dnn::SplitLayer>;

ptr_extern! { crate::dnn::SplitLayer,
	cv_PtrOfSplitLayer_delete, cv_PtrOfSplitLayer_get_inner_ptr, cv_PtrOfSplitLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SplitLayer, cv_PtrOfSplitLayer_new }

impl PtrOfSplitLayer {
	#[inline] pub fn as_raw_PtrOfSplitLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSplitLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SplitLayerTraitConst for PtrOfSplitLayer {
	#[inline] fn as_raw_SplitLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SplitLayerTrait for PtrOfSplitLayer {
	#[inline] fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSplitLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSplitLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSplitLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSplitLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

