pub type PtrOfCorrelationLayer = core::Ptr<crate::dnn::CorrelationLayer>;

ptr_extern! { crate::dnn::CorrelationLayer,
	cv_PtrOfCorrelationLayer_delete, cv_PtrOfCorrelationLayer_get_inner_ptr, cv_PtrOfCorrelationLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::CorrelationLayer, cv_PtrOfCorrelationLayer_new }

impl PtrOfCorrelationLayer {
	#[inline] pub fn as_raw_PtrOfCorrelationLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCorrelationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CorrelationLayerTraitConst for PtrOfCorrelationLayer {
	#[inline] fn as_raw_CorrelationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::CorrelationLayerTrait for PtrOfCorrelationLayer {
	#[inline] fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfCorrelationLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfCorrelationLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfCorrelationLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfCorrelationLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

