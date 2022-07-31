pub type PtrOfLSTMLayer = core::Ptr<dyn crate::dnn::LSTMLayer>;

ptr_extern! { dyn crate::dnn::LSTMLayer,
	cv_PtrOfLSTMLayer_delete, cv_PtrOfLSTMLayer_get_inner_ptr, cv_PtrOfLSTMLayer_get_inner_ptr_mut
}

impl PtrOfLSTMLayer {
	#[inline] pub fn as_raw_PtrOfLSTMLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLSTMLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::LSTMLayerConst for PtrOfLSTMLayer {
	#[inline] fn as_raw_LSTMLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LSTMLayer for PtrOfLSTMLayer {
	#[inline] fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfLSTMLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfLSTMLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfLSTMLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfLSTMLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

