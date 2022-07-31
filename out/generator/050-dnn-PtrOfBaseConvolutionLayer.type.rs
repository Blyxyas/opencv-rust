pub type PtrOfBaseConvolutionLayer = core::Ptr<crate::dnn::BaseConvolutionLayer>;

ptr_extern! { crate::dnn::BaseConvolutionLayer,
	cv_PtrOfBaseConvolutionLayer_delete, cv_PtrOfBaseConvolutionLayer_get_inner_ptr, cv_PtrOfBaseConvolutionLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::BaseConvolutionLayer, cv_PtrOfBaseConvolutionLayer_new }

impl PtrOfBaseConvolutionLayer {
	#[inline] pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBaseConvolutionLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BaseConvolutionLayerTraitConst for PtrOfBaseConvolutionLayer {
	#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BaseConvolutionLayerTrait for PtrOfBaseConvolutionLayer {
	#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBaseConvolutionLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBaseConvolutionLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfBaseConvolutionLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfBaseConvolutionLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

