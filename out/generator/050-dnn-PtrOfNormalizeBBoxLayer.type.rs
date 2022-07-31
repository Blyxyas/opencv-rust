pub type PtrOfNormalizeBBoxLayer = core::Ptr<crate::dnn::NormalizeBBoxLayer>;

ptr_extern! { crate::dnn::NormalizeBBoxLayer,
	cv_PtrOfNormalizeBBoxLayer_delete, cv_PtrOfNormalizeBBoxLayer_get_inner_ptr, cv_PtrOfNormalizeBBoxLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::NormalizeBBoxLayer, cv_PtrOfNormalizeBBoxLayer_new }

impl PtrOfNormalizeBBoxLayer {
	#[inline] pub fn as_raw_PtrOfNormalizeBBoxLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNormalizeBBoxLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::NormalizeBBoxLayerTraitConst for PtrOfNormalizeBBoxLayer {
	#[inline] fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::NormalizeBBoxLayerTrait for PtrOfNormalizeBBoxLayer {
	#[inline] fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfNormalizeBBoxLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfNormalizeBBoxLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfNormalizeBBoxLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfNormalizeBBoxLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

