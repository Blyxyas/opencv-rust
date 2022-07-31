pub type PtrOfDataAugmentationLayer = core::Ptr<crate::dnn::DataAugmentationLayer>;

ptr_extern! { crate::dnn::DataAugmentationLayer,
	cv_PtrOfDataAugmentationLayer_delete, cv_PtrOfDataAugmentationLayer_get_inner_ptr, cv_PtrOfDataAugmentationLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::DataAugmentationLayer, cv_PtrOfDataAugmentationLayer_new }

impl PtrOfDataAugmentationLayer {
	#[inline] pub fn as_raw_PtrOfDataAugmentationLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDataAugmentationLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::DataAugmentationLayerTraitConst for PtrOfDataAugmentationLayer {
	#[inline] fn as_raw_DataAugmentationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::DataAugmentationLayerTrait for PtrOfDataAugmentationLayer {
	#[inline] fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDataAugmentationLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDataAugmentationLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfDataAugmentationLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfDataAugmentationLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

