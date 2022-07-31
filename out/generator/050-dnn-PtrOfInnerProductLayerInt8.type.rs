pub type PtrOfInnerProductLayerInt8 = core::Ptr<crate::dnn::InnerProductLayerInt8>;

ptr_extern! { crate::dnn::InnerProductLayerInt8,
	cv_PtrOfInnerProductLayerInt8_delete, cv_PtrOfInnerProductLayerInt8_get_inner_ptr, cv_PtrOfInnerProductLayerInt8_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::InnerProductLayerInt8, cv_PtrOfInnerProductLayerInt8_new }

impl PtrOfInnerProductLayerInt8 {
	#[inline] pub fn as_raw_PtrOfInnerProductLayerInt8(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfInnerProductLayerInt8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::InnerProductLayerInt8TraitConst for PtrOfInnerProductLayerInt8 {
	#[inline] fn as_raw_InnerProductLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::InnerProductLayerInt8Trait for PtrOfInnerProductLayerInt8 {
	#[inline] fn as_raw_mut_InnerProductLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfInnerProductLayerInt8 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfInnerProductLayerInt8 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::InnerProductLayerTraitConst for PtrOfInnerProductLayerInt8 {
	#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::InnerProductLayerTrait for PtrOfInnerProductLayerInt8 {
	#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfInnerProductLayerInt8 {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfInnerProductLayerInt8 {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

