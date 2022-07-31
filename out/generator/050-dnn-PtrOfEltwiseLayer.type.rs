pub type PtrOfEltwiseLayer = core::Ptr<crate::dnn::EltwiseLayer>;

ptr_extern! { crate::dnn::EltwiseLayer,
	cv_PtrOfEltwiseLayer_delete, cv_PtrOfEltwiseLayer_get_inner_ptr, cv_PtrOfEltwiseLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::EltwiseLayer, cv_PtrOfEltwiseLayer_new }

impl PtrOfEltwiseLayer {
	#[inline] pub fn as_raw_PtrOfEltwiseLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEltwiseLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::EltwiseLayerTraitConst for PtrOfEltwiseLayer {
	#[inline] fn as_raw_EltwiseLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::EltwiseLayerTrait for PtrOfEltwiseLayer {
	#[inline] fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfEltwiseLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfEltwiseLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfEltwiseLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfEltwiseLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

