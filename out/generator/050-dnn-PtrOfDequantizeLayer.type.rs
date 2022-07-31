pub type PtrOfDequantizeLayer = core::Ptr<crate::dnn::DequantizeLayer>;

ptr_extern! { crate::dnn::DequantizeLayer,
	cv_PtrOfDequantizeLayer_delete, cv_PtrOfDequantizeLayer_get_inner_ptr, cv_PtrOfDequantizeLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::DequantizeLayer, cv_PtrOfDequantizeLayer_new }

impl PtrOfDequantizeLayer {
	#[inline] pub fn as_raw_PtrOfDequantizeLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDequantizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::DequantizeLayerTraitConst for PtrOfDequantizeLayer {
	#[inline] fn as_raw_DequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::DequantizeLayerTrait for PtrOfDequantizeLayer {
	#[inline] fn as_raw_mut_DequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDequantizeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDequantizeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfDequantizeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfDequantizeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

