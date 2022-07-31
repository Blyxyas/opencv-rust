pub type PtrOfFloorLayer = core::Ptr<crate::dnn::FloorLayer>;

ptr_extern! { crate::dnn::FloorLayer,
	cv_PtrOfFloorLayer_delete, cv_PtrOfFloorLayer_get_inner_ptr, cv_PtrOfFloorLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::FloorLayer, cv_PtrOfFloorLayer_new }

impl PtrOfFloorLayer {
	#[inline] pub fn as_raw_PtrOfFloorLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFloorLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::FloorLayerTraitConst for PtrOfFloorLayer {
	#[inline] fn as_raw_FloorLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::FloorLayerTrait for PtrOfFloorLayer {
	#[inline] fn as_raw_mut_FloorLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFloorLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFloorLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfFloorLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfFloorLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfFloorLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfFloorLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

