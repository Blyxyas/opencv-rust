pub type PtrOfPaddingLayer = core::Ptr<crate::dnn::PaddingLayer>;

ptr_extern! { crate::dnn::PaddingLayer,
	cv_PtrOfPaddingLayer_delete, cv_PtrOfPaddingLayer_get_inner_ptr, cv_PtrOfPaddingLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::PaddingLayer, cv_PtrOfPaddingLayer_new }

impl PtrOfPaddingLayer {
	#[inline] pub fn as_raw_PtrOfPaddingLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPaddingLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PaddingLayerTraitConst for PtrOfPaddingLayer {
	#[inline] fn as_raw_PaddingLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::PaddingLayerTrait for PtrOfPaddingLayer {
	#[inline] fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfPaddingLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfPaddingLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfPaddingLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfPaddingLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

