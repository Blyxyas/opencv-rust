impl core::MatOpConst for types::AbstractRefMut<'static, dyn core::MatOp> {
	#[inline] fn as_raw_MatOp(&self) -> *const c_void { self.as_raw() }
}

impl core::MatOp for types::AbstractRefMut<'static, dyn core::MatOp> {
	#[inline] fn as_raw_mut_MatOp(&mut self) -> *mut c_void { self.as_raw_mut() }
}

