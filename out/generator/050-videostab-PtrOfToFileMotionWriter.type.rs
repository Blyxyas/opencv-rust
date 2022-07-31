pub type PtrOfToFileMotionWriter = core::Ptr<crate::videostab::ToFileMotionWriter>;

ptr_extern! { crate::videostab::ToFileMotionWriter,
	cv_PtrOfToFileMotionWriter_delete, cv_PtrOfToFileMotionWriter_get_inner_ptr, cv_PtrOfToFileMotionWriter_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::ToFileMotionWriter, cv_PtrOfToFileMotionWriter_new }

impl PtrOfToFileMotionWriter {
	#[inline] pub fn as_raw_PtrOfToFileMotionWriter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfToFileMotionWriter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::ToFileMotionWriterTraitConst for PtrOfToFileMotionWriter {
	#[inline] fn as_raw_ToFileMotionWriter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ToFileMotionWriterTrait for PtrOfToFileMotionWriter {
	#[inline] fn as_raw_mut_ToFileMotionWriter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ImageMotionEstimatorBaseConst for PtrOfToFileMotionWriter {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBase for PtrOfToFileMotionWriter {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfToFileMotionWriter, core::Ptr<dyn crate::videostab::ImageMotionEstimatorBase>,
	cv_PtrOfToFileMotionWriter_to_PtrOfImageMotionEstimatorBase,
}

