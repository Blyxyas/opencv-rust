pub type PtrOfShrinkLayer = core::Ptr<crate::dnn::ShrinkLayer>;

ptr_extern! { crate::dnn::ShrinkLayer,
	cv_PtrOfShrinkLayer_delete, cv_PtrOfShrinkLayer_get_inner_ptr, cv_PtrOfShrinkLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ShrinkLayer, cv_PtrOfShrinkLayer_new }

impl PtrOfShrinkLayer {
	#[inline] pub fn as_raw_PtrOfShrinkLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfShrinkLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ShrinkLayerTraitConst for PtrOfShrinkLayer {
	#[inline] fn as_raw_ShrinkLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ShrinkLayerTrait for PtrOfShrinkLayer {
	#[inline] fn as_raw_mut_ShrinkLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfShrinkLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfShrinkLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfShrinkLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfShrinkLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfShrinkLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfShrinkLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

