pub type PtrOfFromFileMotionReader = core::Ptr<crate::videostab::FromFileMotionReader>;

ptr_extern! { crate::videostab::FromFileMotionReader,
	cv_PtrOfFromFileMotionReader_delete, cv_PtrOfFromFileMotionReader_get_inner_ptr, cv_PtrOfFromFileMotionReader_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::FromFileMotionReader, cv_PtrOfFromFileMotionReader_new }

impl PtrOfFromFileMotionReader {
	#[inline] pub fn as_raw_PtrOfFromFileMotionReader(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFromFileMotionReader(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::FromFileMotionReaderTraitConst for PtrOfFromFileMotionReader {
	#[inline] fn as_raw_FromFileMotionReader(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::FromFileMotionReaderTrait for PtrOfFromFileMotionReader {
	#[inline] fn as_raw_mut_FromFileMotionReader(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ImageMotionEstimatorBaseConst for PtrOfFromFileMotionReader {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBase for PtrOfFromFileMotionReader {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfFromFileMotionReader, core::Ptr<dyn crate::videostab::ImageMotionEstimatorBase>,
	cv_PtrOfFromFileMotionReader_to_PtrOfImageMotionEstimatorBase,
}

