pub type PtrOfTanHLayer = core::Ptr<crate::dnn::TanHLayer>;

ptr_extern! { crate::dnn::TanHLayer,
	cv_PtrOfTanHLayer_delete, cv_PtrOfTanHLayer_get_inner_ptr, cv_PtrOfTanHLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::TanHLayer, cv_PtrOfTanHLayer_new }

impl PtrOfTanHLayer {
	#[inline] pub fn as_raw_PtrOfTanHLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTanHLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::TanHLayerTraitConst for PtrOfTanHLayer {
	#[inline] fn as_raw_TanHLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::TanHLayerTrait for PtrOfTanHLayer {
	#[inline] fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfTanHLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfTanHLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfTanHLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfTanHLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfTanHLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfTanHLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

