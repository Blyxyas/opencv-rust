pub type PtrOfTrackerKCF = core::Ptr<dyn crate::tracking::TrackerKCF>;

ptr_extern! { dyn crate::tracking::TrackerKCF,
	cv_PtrOfTrackerKCF_delete, cv_PtrOfTrackerKCF_get_inner_ptr, cv_PtrOfTrackerKCF_get_inner_ptr_mut
}

impl PtrOfTrackerKCF {
	#[inline] pub fn as_raw_PtrOfTrackerKCF(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerKCF(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::tracking::TrackerKCFConst for PtrOfTrackerKCF {
	#[inline] fn as_raw_TrackerKCF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::tracking::TrackerKCF for PtrOfTrackerKCF {
	#[inline] fn as_raw_mut_TrackerKCF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::TrackerConst for PtrOfTrackerKCF {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::Tracker for PtrOfTrackerKCF {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

