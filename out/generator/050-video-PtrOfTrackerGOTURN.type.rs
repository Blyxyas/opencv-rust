pub type PtrOfTrackerGOTURN = core::Ptr<dyn crate::video::TrackerGOTURN>;

ptr_extern! { dyn crate::video::TrackerGOTURN,
	cv_PtrOfTrackerGOTURN_delete, cv_PtrOfTrackerGOTURN_get_inner_ptr, cv_PtrOfTrackerGOTURN_get_inner_ptr_mut
}

impl PtrOfTrackerGOTURN {
	#[inline] pub fn as_raw_PtrOfTrackerGOTURN(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfTrackerGOTURN(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::video::TrackerGOTURNConst for PtrOfTrackerGOTURN {
	#[inline] fn as_raw_TrackerGOTURN(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::TrackerGOTURN for PtrOfTrackerGOTURN {
	#[inline] fn as_raw_mut_TrackerGOTURN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::video::TrackerConst for PtrOfTrackerGOTURN {
	#[inline] fn as_raw_Tracker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::Tracker for PtrOfTrackerGOTURN {
	#[inline] fn as_raw_mut_Tracker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

