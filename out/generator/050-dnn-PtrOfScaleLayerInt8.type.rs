pub type PtrOfScaleLayerInt8 = core::Ptr<crate::dnn::ScaleLayerInt8>;

ptr_extern! { crate::dnn::ScaleLayerInt8,
	cv_PtrOfScaleLayerInt8_delete, cv_PtrOfScaleLayerInt8_get_inner_ptr, cv_PtrOfScaleLayerInt8_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ScaleLayerInt8, cv_PtrOfScaleLayerInt8_new }

impl PtrOfScaleLayerInt8 {
	#[inline] pub fn as_raw_PtrOfScaleLayerInt8(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfScaleLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ScaleLayerInt8TraitConst for PtrOfScaleLayerInt8 {
	#[inline] fn as_raw_ScaleLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ScaleLayerInt8Trait for PtrOfScaleLayerInt8 {
	#[inline] fn as_raw_mut_ScaleLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfScaleLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfScaleLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfScaleLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfScaleLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ScaleLayerTraitConst for PtrOfScaleLayerInt8 {
	#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ScaleLayerTrait for PtrOfScaleLayerInt8 {
	#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

