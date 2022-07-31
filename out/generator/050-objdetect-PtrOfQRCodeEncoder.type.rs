pub type PtrOfQRCodeEncoder = core::Ptr<dyn crate::objdetect::QRCodeEncoder>;

ptr_extern! { dyn crate::objdetect::QRCodeEncoder,
	cv_PtrOfQRCodeEncoder_delete, cv_PtrOfQRCodeEncoder_get_inner_ptr, cv_PtrOfQRCodeEncoder_get_inner_ptr_mut
}

impl PtrOfQRCodeEncoder {
	#[inline] pub fn as_raw_PtrOfQRCodeEncoder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQRCodeEncoder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::objdetect::QRCodeEncoderConst for PtrOfQRCodeEncoder {
	#[inline] fn as_raw_QRCodeEncoder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::QRCodeEncoder for PtrOfQRCodeEncoder {
	#[inline] fn as_raw_mut_QRCodeEncoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

