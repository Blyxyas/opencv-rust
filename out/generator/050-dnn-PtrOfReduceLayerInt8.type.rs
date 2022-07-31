pub type PtrOfReduceLayerInt8 = core::Ptr<crate::dnn::ReduceLayerInt8>;

ptr_extern! { crate::dnn::ReduceLayerInt8,
	cv_PtrOfReduceLayerInt8_delete, cv_PtrOfReduceLayerInt8_get_inner_ptr, cv_PtrOfReduceLayerInt8_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ReduceLayerInt8, cv_PtrOfReduceLayerInt8_new }

impl PtrOfReduceLayerInt8 {
	#[inline] pub fn as_raw_PtrOfReduceLayerInt8(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReduceLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReduceLayerInt8TraitConst for PtrOfReduceLayerInt8 {
	#[inline] fn as_raw_ReduceLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReduceLayerInt8Trait for PtrOfReduceLayerInt8 {
	#[inline] fn as_raw_mut_ReduceLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfReduceLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfReduceLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfReduceLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfReduceLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ReduceLayerTraitConst for PtrOfReduceLayerInt8 {
	#[inline] fn as_raw_ReduceLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReduceLayerTrait for PtrOfReduceLayerInt8 {
	#[inline] fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

