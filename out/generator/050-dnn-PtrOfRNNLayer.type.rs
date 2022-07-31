pub type PtrOfRNNLayer = core::Ptr<dyn crate::dnn::RNNLayer>;

ptr_extern! { dyn crate::dnn::RNNLayer,
	cv_PtrOfRNNLayer_delete, cv_PtrOfRNNLayer_get_inner_ptr, cv_PtrOfRNNLayer_get_inner_ptr_mut
}

impl PtrOfRNNLayer {
	#[inline] pub fn as_raw_PtrOfRNNLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRNNLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::RNNLayerConst for PtrOfRNNLayer {
	#[inline] fn as_raw_RNNLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::RNNLayer for PtrOfRNNLayer {
	#[inline] fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRNNLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRNNLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfRNNLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfRNNLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

