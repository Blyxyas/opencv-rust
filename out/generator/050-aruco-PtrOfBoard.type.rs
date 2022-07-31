pub type PtrOfBoard = core::Ptr<crate::aruco::Board>;

ptr_extern! { crate::aruco::Board,
	cv_PtrOfBoard_delete, cv_PtrOfBoard_get_inner_ptr, cv_PtrOfBoard_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::aruco::Board, cv_PtrOfBoard_new }

impl PtrOfBoard {
	#[inline] pub fn as_raw_PtrOfBoard(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBoard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::aruco::BoardTraitConst for PtrOfBoard {
	#[inline] fn as_raw_Board(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::BoardTrait for PtrOfBoard {
	#[inline] fn as_raw_mut_Board(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

