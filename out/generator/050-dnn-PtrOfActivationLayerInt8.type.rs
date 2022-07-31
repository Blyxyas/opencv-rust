pub type PtrOfActivationLayerInt8 = core::Ptr<crate::dnn::ActivationLayerInt8>;

ptr_extern! { crate::dnn::ActivationLayerInt8,
	cv_PtrOfActivationLayerInt8_delete, cv_PtrOfActivationLayerInt8_get_inner_ptr, cv_PtrOfActivationLayerInt8_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ActivationLayerInt8, cv_PtrOfActivationLayerInt8_new }

impl PtrOfActivationLayerInt8 {
	#[inline] pub fn as_raw_PtrOfActivationLayerInt8(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfActivationLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ActivationLayerInt8TraitConst for PtrOfActivationLayerInt8 {
	#[inline] fn as_raw_ActivationLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerInt8Trait for PtrOfActivationLayerInt8 {
	#[inline] fn as_raw_mut_ActivationLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfActivationLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfActivationLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfActivationLayerInt8 {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfActivationLayerInt8 {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfActivationLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfActivationLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

