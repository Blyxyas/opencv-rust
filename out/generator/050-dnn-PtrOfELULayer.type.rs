pub type PtrOfELULayer = core::Ptr<crate::dnn::ELULayer>;

ptr_extern! { crate::dnn::ELULayer,
	cv_PtrOfELULayer_delete, cv_PtrOfELULayer_get_inner_ptr, cv_PtrOfELULayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ELULayer, cv_PtrOfELULayer_new }

impl PtrOfELULayer {
	#[inline] pub fn as_raw_PtrOfELULayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfELULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ELULayerTraitConst for PtrOfELULayer {
	#[inline] fn as_raw_ELULayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ELULayerTrait for PtrOfELULayer {
	#[inline] fn as_raw_mut_ELULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfELULayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfELULayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfELULayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfELULayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfELULayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfELULayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

