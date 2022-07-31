pub type PtrOfScaleLayer = core::Ptr<crate::dnn::ScaleLayer>;

ptr_extern! { crate::dnn::ScaleLayer,
	cv_PtrOfScaleLayer_delete, cv_PtrOfScaleLayer_get_inner_ptr, cv_PtrOfScaleLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ScaleLayer, cv_PtrOfScaleLayer_new }

impl PtrOfScaleLayer {
	#[inline] pub fn as_raw_PtrOfScaleLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfScaleLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ScaleLayerTraitConst for PtrOfScaleLayer {
	#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ScaleLayerTrait for PtrOfScaleLayer {
	#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfScaleLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfScaleLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfScaleLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfScaleLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

