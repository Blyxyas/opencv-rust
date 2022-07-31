pub type PtrOfEltwiseLayerInt8 = core::Ptr<crate::dnn::EltwiseLayerInt8>;

ptr_extern! { crate::dnn::EltwiseLayerInt8,
	cv_PtrOfEltwiseLayerInt8_delete, cv_PtrOfEltwiseLayerInt8_get_inner_ptr, cv_PtrOfEltwiseLayerInt8_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::EltwiseLayerInt8, cv_PtrOfEltwiseLayerInt8_new }

impl PtrOfEltwiseLayerInt8 {
	#[inline] pub fn as_raw_PtrOfEltwiseLayerInt8(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEltwiseLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::EltwiseLayerInt8TraitConst for PtrOfEltwiseLayerInt8 {
	#[inline] fn as_raw_EltwiseLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::EltwiseLayerInt8Trait for PtrOfEltwiseLayerInt8 {
	#[inline] fn as_raw_mut_EltwiseLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfEltwiseLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfEltwiseLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfEltwiseLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfEltwiseLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

