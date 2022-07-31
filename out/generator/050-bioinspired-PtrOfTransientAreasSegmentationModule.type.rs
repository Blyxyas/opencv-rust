pub type PtrOfTransientAreasSegmentationModule = core::Ptr<dyn crate::bioinspired::TransientAreasSegmentationModule>;

ptr_extern! { dyn crate::bioinspired::TransientAreasSegmentationModule,
	cv_PtrOfTransientAreasSegmentationModule_delete, cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr, cv_PtrOfTransientAreasSegmentationModule_get_inner_ptr_mut
}

impl PtrOfTransientAreasSegmentationModule {
	#[inline] pub fn as_raw_PtrOfTransientAreasSegmentationModule(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTransientAreasSegmentationModule(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::bioinspired::TransientAreasSegmentationModuleConst for PtrOfTransientAreasSegmentationModule {
	#[inline] fn as_raw_TransientAreasSegmentationModule(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bioinspired::TransientAreasSegmentationModule for PtrOfTransientAreasSegmentationModule {
	#[inline] fn as_raw_mut_TransientAreasSegmentationModule(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfTransientAreasSegmentationModule {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfTransientAreasSegmentationModule {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

