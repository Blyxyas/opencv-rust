#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! \defgroup gapi G-API framework
//!    # G-API Main Classes
//!    # G-API Data Types
//!       # G-API Metadata Descriptors
//!    # G-API Standard Backends
//!    # G-API Graph Compilation Arguments
//!    # G-API Serialization functionality
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::GMatPTraitConst, super::GMatPTrait, super::GArrayUTraitConst, super::GArrayUTrait, super::GOpaqueUTraitConst, super::GOpaqueUTrait, super::GFrameDescTraitConst, super::GFrameDescTrait, super::Own_ScalarTraitConst, super::Own_ScalarTrait, super::MediaFrameTraitConst, super::MediaFrameTrait, super::MediaFrame_ViewTraitConst, super::MediaFrame_ViewTrait, super::MediaFrame_IAdapterConst, super::MediaFrame_IAdapter, super::RMat_ViewTraitConst, super::RMat_ViewTrait, super::RMatTraitConst, super::RMatTrait, super::GArgTraitConst, super::GArgTrait, super::GRunArgTraitConst, super::GRunArgTrait, super::DataTraitConst, super::DataTrait, super::GCompiledTraitConst, super::GCompiledTrait, super::GCallTraitConst, super::GCallTrait, super::GTransformTraitConst, super::GTransformTrait, super::GKernelTraitConst, super::GKernelTrait, super::GKernelImplTraitConst, super::GKernelImplTrait, super::GBackendTraitConst, super::GBackendTrait, super::use_onlyTraitConst, super::use_onlyTrait };
}

// GARRAY /usr/include/opencv2/gapi/gtype_traits.hpp:56
pub const ArgKind_GARRAY: i32 = 6;
// GFRAME /usr/include/opencv2/gapi/gtype_traits.hpp:54
pub const ArgKind_GFRAME: i32 = 4;
// GMAT /usr/include/opencv2/gapi/gtype_traits.hpp:52
pub const ArgKind_GMAT: i32 = 2;
// GMATP /usr/include/opencv2/gapi/gtype_traits.hpp:53
pub const ArgKind_GMATP: i32 = 3;
// GOBJREF /usr/include/opencv2/gapi/gtype_traits.hpp:51
pub const ArgKind_GOBJREF: i32 = 1;
// GOPAQUE /usr/include/opencv2/gapi/gtype_traits.hpp:57
pub const ArgKind_GOPAQUE: i32 = 7;
// GSCALAR /usr/include/opencv2/gapi/gtype_traits.hpp:55
pub const ArgKind_GSCALAR: i32 = 5;
// OPAQUE /usr/include/opencv2/gapi/gtype_traits.hpp:49
pub const ArgKind_OPAQUE: i32 = 0;
// OPAQUE_VAL /usr/include/opencv2/gapi/gtype_traits.hpp:46
pub const ArgKind_OPAQUE_VAL: i32 = 0;
// GARRAY /usr/include/opencv2/gapi/gcommon.hpp:97
pub const GShape_GARRAY: i32 = 2;
// GFRAME /usr/include/opencv2/gapi/gcommon.hpp:99
pub const GShape_GFRAME: i32 = 4;
// GMAT /usr/include/opencv2/gapi/gcommon.hpp:95
pub const GShape_GMAT: i32 = 0;
// GOPAQUE /usr/include/opencv2/gapi/gcommon.hpp:98
pub const GShape_GOPAQUE: i32 = 3;
// GSCALAR /usr/include/opencv2/gapi/gcommon.hpp:96
pub const GShape_GSCALAR: i32 = 1;
// BGR /usr/include/opencv2/gapi/gframe.hpp:87
pub const MediaFormat_BGR: i32 = 0;
// GRAY /usr/include/opencv2/gapi/gframe.hpp:89
pub const MediaFormat_GRAY: i32 = 2;
// NV12 /usr/include/opencv2/gapi/gframe.hpp:88
pub const MediaFormat_NV12: i32 = 1;
// CV_BOOL /usr/include/opencv2/gapi/gcommon.hpp:45
pub const OpaqueKind_CV_BOOL: i32 = 1;
// CV_DOUBLE /usr/include/opencv2/gapi/gcommon.hpp:48
pub const OpaqueKind_CV_DOUBLE: i32 = 4;
// CV_DRAW_PRIM /usr/include/opencv2/gapi/gcommon.hpp:58
pub const OpaqueKind_CV_DRAW_PRIM: i32 = 14;
// CV_FLOAT /usr/include/opencv2/gapi/gcommon.hpp:49
pub const OpaqueKind_CV_FLOAT: i32 = 5;
// CV_INT /usr/include/opencv2/gapi/gcommon.hpp:46
pub const OpaqueKind_CV_INT: i32 = 2;
// CV_INT64 /usr/include/opencv2/gapi/gcommon.hpp:47
pub const OpaqueKind_CV_INT64: i32 = 3;
// CV_MAT /usr/include/opencv2/gapi/gcommon.hpp:57
pub const OpaqueKind_CV_MAT: i32 = 13;
// CV_POINT /usr/include/opencv2/gapi/gcommon.hpp:52
pub const OpaqueKind_CV_POINT: i32 = 8;
// CV_POINT2F /usr/include/opencv2/gapi/gcommon.hpp:53
pub const OpaqueKind_CV_POINT2F: i32 = 9;
// CV_RECT /usr/include/opencv2/gapi/gcommon.hpp:55
pub const OpaqueKind_CV_RECT: i32 = 11;
// CV_SCALAR /usr/include/opencv2/gapi/gcommon.hpp:56
pub const OpaqueKind_CV_SCALAR: i32 = 12;
// CV_SIZE /usr/include/opencv2/gapi/gcommon.hpp:54
pub const OpaqueKind_CV_SIZE: i32 = 10;
// CV_STRING /usr/include/opencv2/gapi/gcommon.hpp:51
pub const OpaqueKind_CV_STRING: i32 = 7;
// CV_UINT64 /usr/include/opencv2/gapi/gcommon.hpp:50
pub const OpaqueKind_CV_UINT64: i32 = 6;
// CV_UNKNOWN /usr/include/opencv2/gapi/gcommon.hpp:44
pub const OpaqueKind_CV_UNKNOWN: i32 = 0;
// ArgKind /usr/include/opencv2/gapi/gtype_traits.hpp:44
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArgKind {
	OPAQUE_VAL = 0,
	// OPAQUE = 0 as isize, // duplicate discriminant
	GOBJREF = 1,
	GMAT = 2,
	GMATP = 3,
	GFRAME = 4,
	GSCALAR = 5,
	GARRAY = 6,
	GOPAQUE = 7,
}

opencv_type_enum! { crate::gapi::ArgKind }

// GShape /usr/include/opencv2/gapi/gcommon.hpp:93
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GShape {
	GMAT = 0,
	GSCALAR = 1,
	GARRAY = 2,
	GOPAQUE = 3,
	GFRAME = 4,
}

opencv_type_enum! { crate::gapi::GShape }

// MediaFormat /usr/include/opencv2/gapi/gframe.hpp:85
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MediaFormat {
	BGR = 0,
	NV12 = 1,
	GRAY = 2,
}

opencv_type_enum! { crate::gapi::MediaFormat }

// Access /usr/include/opencv2/gapi/media.hpp:57
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MediaFrame_Access {
	R = 0,
	W = 1,
}

opencv_type_enum! { crate::gapi::MediaFrame_Access }

// OpaqueKind /usr/include/opencv2/gapi/gcommon.hpp:42
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpaqueKind {
	CV_UNKNOWN = 0,
	CV_BOOL = 1,
	CV_INT = 2,
	CV_INT64 = 3,
	CV_DOUBLE = 4,
	CV_FLOAT = 5,
	CV_UINT64 = 6,
	CV_STRING = 7,
	CV_POINT = 8,
	CV_POINT2F = 9,
	CV_SIZE = 10,
	CV_RECT = 11,
	CV_SCALAR = 12,
	CV_MAT = 13,
	CV_DRAW_PRIM = 14,
}

opencv_type_enum! { crate::gapi::OpaqueKind }

// Access /usr/include/opencv2/gapi/rmat.hpp:103
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RMat_Access {
	R = 0,
	W = 1,
}

opencv_type_enum! { crate::gapi::RMat_Access }

// Adapter /usr/include/opencv2/gapi/rmat.hpp:123
pub type RMat_Adapter = dyn crate::gapi::RMat_IAdapter;
// stepsT /usr/include/opencv2/gapi/rmat.hpp:60
pub type RMat_View_stepsT = core::Vector<size_t>;
// ContMethod /usr/include/opencv2/gapi/imgproc.hpp:163
pub type cont_method = crate::imgproc::ContourApproximationModes;
// RetrMode /usr/include/opencv2/gapi/imgproc.hpp:162
pub type retr_mode = crate::imgproc::RetrievalModes;
// descr_of(const cv::Mat &) /usr/include/opencv2/gapi/gmat.hpp:268
#[inline]
pub fn descr_of_3(mat: &core::Mat) -> Result<crate::gapi::GMatDesc> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_descr_of_const_MatR(mat.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
	Ok(ret)
}

// descr_of(const cv::MediaFrame &) /usr/include/opencv2/gapi/gframe.hpp:107
#[inline]
pub fn descr_of_5(frame: &crate::gapi::MediaFrame) -> Result<crate::gapi::GFrameDesc> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_descr_of_const_MediaFrameR(frame.as_raw_MediaFrame(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GFrameDesc::opencv_from_extern(ret) };
	Ok(ret)
}

// descr_of(const cv::RMat &) /usr/include/opencv2/gapi/gmat.hpp:265
#[inline]
pub fn descr_of_2(mat: &crate::gapi::RMat) -> Result<crate::gapi::GMatDesc> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_descr_of_const_RMatR(mat.as_raw_RMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
	Ok(ret)
}

// descr_of(const cv::Scalar &) /usr/include/opencv2/gapi/gscalar.hpp:134
#[inline]
pub fn descr_of_4(scalar: core::Scalar) -> Result<crate::gapi::GScalarDesc> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_descr_of_const_ScalarR(&scalar, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GScalarDesc::opencv_from_extern(ret) };
	Ok(ret)
}

// descr_of(const cv::UMat &) /usr/include/opencv2/gapi/gmat.hpp:256
#[inline]
pub fn descr_of(mat: &core::UMat) -> Result<crate::gapi::GMatDesc> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_descr_of_const_UMatR(mat.as_raw_UMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
	Ok(ret)
}

// BGR2Gray(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1415
#[inline]
pub fn bgr2_gray(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_BGR2Gray_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// BGR2I420(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1448
#[inline]
pub fn bgr2_i420(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_BGR2I420_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// BGR2LUV(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1510
#[inline]
pub fn bgr2_luv(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_BGR2LUV_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// BGR2YUV(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1552
#[inline]
pub fn bgr2_yuv(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_BGR2YUV_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// BayerGR2RGB(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1639
#[inline]
pub fn bayer_gr2_rgb(src_gr: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_BayerGR2RGB_const_GMatR(src_gr.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * aperture_size: 3
/// * l2gradient: false
// Canny(const cv::GMat &, double, double, int, bool) /usr/include/opencv2/gapi/imgproc.hpp:1026
#[inline]
pub fn canny(image: &crate::gapi::GMat, threshold1: f64, threshold2: f64, aperture_size: i32, l2gradient: bool) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_Canny_const_GMatR_double_double_int_bool(image.as_raw_GMat(), threshold1, threshold2, aperture_size, l2gradient, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// I4202BGR(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1480
#[inline]
pub fn i4202_bgr(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_I4202BGR_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// I4202RGB(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1496
#[inline]
pub fn i4202_rgb(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_I4202RGB_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// LUT(const cv::GMat &, const cv::Mat &) /usr/include/opencv2/gapi/core.hpp:1699
#[inline]
pub fn lut(src: &crate::gapi::GMat, lut: &core::Mat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_LUT_const_GMatR_const_MatR(src.as_raw_GMat(), lut.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// LUV2BGR(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1524
#[inline]
pub fn luv2_bgr(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_LUV2BGR_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ksize: 1
/// * scale: 1
/// * delta: 0
/// * border_type: BORDER_DEFAULT
// Laplacian(const cv::GMat &, int, int, double, double, int) /usr/include/opencv2/gapi/imgproc.hpp:967
#[inline]
pub fn laplacian(src: &crate::gapi::GMat, ddepth: i32, ksize: i32, scale: f64, delta: f64, border_type: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_Laplacian_const_GMatR_int_int_double_double_int(src.as_raw_GMat(), ddepth, ksize, scale, delta, border_type, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// NV12toBGR(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1625
#[inline]
pub fn nv12to_bgr(src_y: &crate::gapi::GMat, src_uv: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_NV12toBGR_const_GMatR_const_GMatR(src_y.as_raw_GMat(), src_uv.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// NV12toBGRp(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1703
#[inline]
pub fn nv12to_bg_rp(src_y: &crate::gapi::GMat, src_uv: &crate::gapi::GMat) -> Result<crate::gapi::GMatP> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_NV12toBGRp_const_GMatR_const_GMatR(src_y.as_raw_GMat(), src_uv.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMatP::opencv_from_extern(ret) };
	Ok(ret)
}

// NV12toGray(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1610
#[inline]
pub fn nv12to_gray(src_y: &crate::gapi::GMat, src_uv: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_NV12toGray_const_GMatR_const_GMatR(src_y.as_raw_GMat(), src_uv.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// NV12toRGB(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1595
#[inline]
pub fn nv12to_rgb(src_y: &crate::gapi::GMat, src_uv: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_NV12toRGB_const_GMatR_const_GMatR(src_y.as_raw_GMat(), src_uv.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// NV12toRGBp(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1685
#[inline]
pub fn nv12to_rg_bp(src_y: &crate::gapi::GMat, src_uv: &crate::gapi::GMat) -> Result<crate::gapi::GMatP> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_NV12toRGBp_const_GMatR_const_GMatR(src_y.as_raw_GMat(), src_uv.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMatP::opencv_from_extern(ret) };
	Ok(ret)
}

// RGB2Gray(const cv::GMat &, float, float, float) /usr/include/opencv2/gapi/imgproc.hpp:1402
#[inline]
pub fn rgb2_gray(src: &crate::gapi::GMat, r_y: f32, g_y: f32, b_y: f32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_RGB2Gray_const_GMatR_float_float_float(src.as_raw_GMat(), r_y, g_y, b_y, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// RGB2HSV(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1653
#[inline]
pub fn rgb2_hsv(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_RGB2HSV_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// RGB2I420(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1464
#[inline]
pub fn rgb2_i420(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_RGB2I420_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// RGB2Lab(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1566
#[inline]
pub fn rgb2_lab(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_RGB2Lab_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// RGB2YUV422(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1667
#[inline]
pub fn rgb2_yuv422(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_RGB2YUV422_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// RGB2YUV(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1432
#[inline]
pub fn rgb2_yuv(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_RGB2YUV_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ksize: 3
/// * scale: 1
/// * delta: 0
/// * border_type: BORDER_DEFAULT
/// * border_value: Scalar(0)
// Sobel(const cv::GMat &, int, int, int, int, double, double, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:886
#[inline]
pub fn sobel(src: &crate::gapi::GMat, ddepth: i32, dx: i32, dy: i32, ksize: i32, scale: f64, delta: f64, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_Sobel_const_GMatR_int_int_int_int_double_double_int_const_ScalarR(src.as_raw_GMat(), ddepth, dx, dy, ksize, scale, delta, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// YUV2BGR(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1538
#[inline]
pub fn yuv2_bgr(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_YUV2BGR_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// YUV2RGB(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1580
#[inline]
pub fn yuv2_rgb(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_YUV2RGB_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// absDiffC(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1254
#[inline]
pub fn abs_diff_c(src: &crate::gapi::GMat, c: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_absDiffC_const_GMatR_const_GScalarR(src.as_raw_GMat(), c.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// absDiff(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1237
#[inline]
pub fn abs_diff(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_absDiff_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
// addC(const cv::GScalar &, const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:638
#[inline]
pub fn add_c(c: &crate::gapi::GScalar, src1: &crate::gapi::GMat, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_addC_const_GScalarR_const_GMatR_int(c.as_raw_GScalar(), src1.as_raw_GMat(), ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
// addWeighted(const cv::GMat &, double, const cv::GMat &, double, double, int) /usr/include/opencv2/gapi/core.hpp:1302
#[inline]
pub fn add_weighted(src1: &crate::gapi::GMat, alpha: f64, src2: &crate::gapi::GMat, beta: f64, gamma: f64, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_addWeighted_const_GMatR_double_const_GMatR_double_double_int(src1.as_raw_GMat(), alpha, src2.as_raw_GMat(), beta, gamma, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * border_type: BORDER_DEFAULT
// bilateralFilter(const cv::GMat &, int, double, double, int) /usr/include/opencv2/gapi/imgproc.hpp:1001
#[inline]
pub fn bilateral_filter(src: &crate::gapi::GMat, d: i32, sigma_color: f64, sigma_space: f64, border_type: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_bilateralFilter_const_GMatR_int_double_double_int(src.as_raw_GMat(), d, sigma_color, sigma_space, border_type, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// bitwise_and(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1089
#[inline]
pub fn bitwise_and(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_bitwise_and_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// bitwise_and(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1095
#[inline]
pub fn bitwise_and_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_bitwise_and_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// bitwise_not(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1165
#[inline]
pub fn bitwise_not(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_bitwise_not_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// bitwise_or(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1113
#[inline]
pub fn bitwise_or(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_bitwise_or_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// bitwise_or(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1119
#[inline]
pub fn bitwise_or_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_bitwise_or_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// bitwise_xor(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1138
#[inline]
pub fn bitwise_xor(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_bitwise_xor_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// bitwise_xor(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1144
#[inline]
pub fn bitwise_xor_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_bitwise_xor_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * anchor: Point(-1,-1)
/// * border_type: BORDER_DEFAULT
/// * border_value: Scalar(0)
// blur(const cv::GMat &, const cv::Size &, const cv::Point &, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:657
#[inline]
pub fn blur(src: &crate::gapi::GMat, ksize: core::Size, anchor: core::Point, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_blur_const_GMatR_const_SizeR_const_PointR_int_const_ScalarR(src.as_raw_GMat(), &ksize, &anchor, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * anchor: Point(-1,-1)
/// * normalize: true
/// * border_type: BORDER_DEFAULT
/// * border_value: Scalar(0)
// boxFilter(const cv::GMat &, int, const cv::Size &, const cv::Point &, bool, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:630
#[inline]
pub fn box_filter(src: &crate::gapi::GMat, dtype: i32, ksize: core::Size, anchor: core::Point, normalize: bool, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_boxFilter_const_GMatR_int_const_SizeR_const_PointR_bool_int_const_ScalarR(src.as_raw_GMat(), dtype, &ksize, &anchor, normalize, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// cmpEQ(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1041
#[inline]
pub fn cmp_eq(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_cmpEQ_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// cmpEQ(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1045
#[inline]
pub fn cmp_eq_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_cmpEQ_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// cmpGE(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:989
#[inline]
pub fn cmp_ge(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_cmpGE_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// cmpGE(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:993
#[inline]
pub fn cmp_ge_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_cmpGE_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// cmpGT(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:937
#[inline]
pub fn cmp_gt(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_cmpGT_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// cmpGT(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:941
#[inline]
pub fn cmp_gt_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_cmpGT_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// cmpLE(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1015
#[inline]
pub fn cmp_le(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_cmpLE_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// cmpLE(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1019
#[inline]
pub fn cmp_le_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_cmpLE_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// cmpLT(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:963
#[inline]
pub fn cmp_lt(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_cmpLT_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// cmpLT(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:967
#[inline]
pub fn cmp_lt_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_cmpLT_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// cmpNE(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1067
#[inline]
pub fn cmp_ne(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_cmpNE_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// cmpNE(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1071
#[inline]
pub fn cmp_ne_1(src1: &crate::gapi::GMat, src2: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_cmpNE_const_GMatR_const_GScalarR(src1.as_raw_GMat(), src2.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// combine(const cv::GKernelPackage &, const cv::GKernelPackage &) /usr/include/opencv2/gapi/gkernel.hpp:417
#[inline]
pub fn combine(lhs: &crate::gapi::GKernelPackage, rhs: &crate::gapi::GKernelPackage) -> Result<crate::gapi::GKernelPackage> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_combine_const_GKernelPackageR_const_GKernelPackageR(lhs.as_raw_GKernelPackage(), rhs.as_raw_GKernelPackage(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GKernelPackage::opencv_from_extern(ret) };
	Ok(ret)
}

// concatHor(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1632
#[inline]
pub fn concat_hor(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_concatHor_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// concatHor(const std::vector<GMat> &) /usr/include/opencv2/gapi/core.hpp:1640
#[inline]
pub fn concat_hor_1(v: &core::Vector<crate::gapi::GMat>) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_concatHor_const_vector_GMat_R(v.as_raw_VectorOfGMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// concatVert(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1672
#[inline]
pub fn concat_vert(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_concatVert_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// concatVert(const std::vector<GMat> &) /usr/include/opencv2/gapi/core.hpp:1680
#[inline]
pub fn concat_vert_1(v: &core::Vector<crate::gapi::GMat>) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_concatVert_const_vector_GMat_R(v.as_raw_VectorOfGMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * alpha: 1
/// * beta: 0
// convertTo(const cv::GMat &, int, double, double) /usr/include/opencv2/gapi/core.hpp:1716
#[inline]
pub fn convert_to(src: &crate::gapi::GMat, rdepth: i32, alpha: f64, beta: f64) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_convertTo_const_GMatR_int_double_double(src.as_raw_GMat(), rdepth, alpha, beta, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// copy(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/format.hpp:88
#[inline]
pub fn copy(in_: &crate::gapi::GFrame) -> Result<crate::gapi::GFrame> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_copy_const_GFrameR(in_.as_raw_GFrame(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GFrame::opencv_from_extern(ret) };
	Ok(ret)
}

// crop(const cv::GMat &, const cv::Rect &) /usr/include/opencv2/gapi/core.hpp:1604
#[inline]
pub fn crop(src: &crate::gapi::GMat, rect: core::Rect) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_crop_const_GMatR_const_RectR(src.as_raw_GMat(), &rect, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * iterations: 1
/// * border_type: BORDER_CONSTANT
/// * border_value: morphologyDefaultBorderValue()
// dilate3x3(const cv::GMat &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:804
#[inline]
pub fn dilate3x3(src: &crate::gapi::GMat, iterations: i32, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_dilate3x3_const_GMatR_int_int_const_ScalarR(src.as_raw_GMat(), iterations, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * anchor: Point(-1,-1)
/// * iterations: 1
/// * border_type: BORDER_CONSTANT
/// * border_value: morphologyDefaultBorderValue()
// dilate(const cv::GMat &, const cv::Mat &, const cv::Point &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:780
#[inline]
pub fn dilate(src: &crate::gapi::GMat, kernel: &core::Mat, anchor: core::Point, iterations: i32, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_dilate_const_GMatR_const_MatR_const_PointR_int_int_const_ScalarR(src.as_raw_GMat(), kernel.as_raw_Mat(), &anchor, iterations, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
// divC(const cv::GMat &, const cv::GScalar &, double, int) /usr/include/opencv2/gapi/core.hpp:788
#[inline]
pub fn div_c(src: &crate::gapi::GMat, divisor: &crate::gapi::GScalar, scale: f64, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_divC_const_GMatR_const_GScalarR_double_int(src.as_raw_GMat(), divisor.as_raw_GScalar(), scale, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
// divRC(const cv::GScalar &, const cv::GMat &, double, int) /usr/include/opencv2/gapi/core.hpp:809
#[inline]
pub fn div_rc(divident: &crate::gapi::GScalar, src: &crate::gapi::GMat, scale: f64, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_divRC_const_GScalarR_const_GMatR_double_int(divident.as_raw_GScalar(), src.as_raw_GMat(), scale, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
// div(const cv::GMat &, const cv::GMat &, double, int) /usr/include/opencv2/gapi/core.hpp:767
#[inline]
pub fn div_3(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, scale: f64, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_div_const_GMatR_const_GMatR_double_int(src1.as_raw_GMat(), src2.as_raw_GMat(), scale, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// equalizeHist(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1101
#[inline]
pub fn equalize_hist(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_equalizeHist_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * iterations: 1
/// * border_type: BORDER_CONSTANT
/// * border_value: morphologyDefaultBorderValue()
// erode3x3(const cv::GMat &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:753
#[inline]
pub fn erode3x3(src: &crate::gapi::GMat, iterations: i32, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_erode3x3_const_GMatR_int_int_const_ScalarR(src.as_raw_GMat(), iterations, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * anchor: Point(-1,-1)
/// * iterations: 1
/// * border_type: BORDER_CONSTANT
/// * border_value: morphologyDefaultBorderValue()
// erode(const cv::GMat &, const cv::Mat &, const cv::Point &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:733
#[inline]
pub fn erode(src: &crate::gapi::GMat, kernel: &core::Mat, anchor: core::Point, iterations: i32, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_erode_const_GMatR_const_MatR_const_PointR_int_int_const_ScalarR(src.as_raw_GMat(), kernel.as_raw_Mat(), &anchor, iterations, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * anchor: Point(-1,-1)
/// * delta: Scalar(0)
/// * border_type: BORDER_DEFAULT
/// * border_value: Scalar(0)
// filter2D(const cv::GMat &, int, const cv::Mat &, const cv::Point &, const cv::Scalar &, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:596
#[inline]
pub fn filter_2d(src: &crate::gapi::GMat, ddepth: i32, kernel: &core::Mat, anchor: core::Point, delta: core::Scalar, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_filter2D_const_GMatR_int_const_MatR_const_PointR_const_ScalarR_int_const_ScalarR(src.as_raw_GMat(), ddepth, kernel.as_raw_Mat(), &anchor, &delta, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// flip(const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:1590
#[inline]
pub fn flip(src: &crate::gapi::GMat, flip_code: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_flip_const_GMatR_int(src.as_raw_GMat(), flip_code, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * sigma_y: 0
/// * border_type: BORDER_DEFAULT
/// * border_value: Scalar(0)
// gaussianBlur(const cv::GMat &, const cv::Size &, double, double, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:690
#[inline]
pub fn gaussian_blur(src: &crate::gapi::GMat, ksize: core::Size, sigma_x: f64, sigma_y: f64, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_gaussianBlur_const_GMatR_const_SizeR_double_double_int_const_ScalarR(src.as_raw_GMat(), &ksize, sigma_x, sigma_y, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// inRange(const cv::GMat &, const cv::GScalar &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1441
#[inline]
pub fn in_range(src: &crate::gapi::GMat, thresh_low: &crate::gapi::GScalar, thresh_up: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_inRange_const_GMatR_const_GScalarR_const_GScalarR(src.as_raw_GMat(), thresh_low.as_raw_GScalar(), thresh_up.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// mask(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:822
#[inline]
pub fn mask(src: &crate::gapi::GMat, mask: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_mask_const_GMatR_const_GMatR(src.as_raw_GMat(), mask.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// max(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1220
#[inline]
pub fn max(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_max_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// merge3(const cv::GMat &, const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1484
#[inline]
pub fn merge3(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, src3: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_merge3_const_GMatR_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), src3.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// merge4(const cv::GMat &, const cv::GMat &, const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1465
#[inline]
pub fn merge4(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, src3: &crate::gapi::GMat, src4: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_merge4_const_GMatR_const_GMatR_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), src3.as_raw_GMat(), src4.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// min(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1203
#[inline]
pub fn min(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_min_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * anchor: Point(-1,-1)
/// * iterations: 1
/// * border_type: BORDER_CONSTANT
/// * border_value: morphologyDefaultBorderValue()
// morphologyEx(const cv::GMat &, const cv::MorphTypes, const cv::Mat &, const cv::Point &, const int, const cv::BorderTypes, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:834
#[inline]
pub fn morphology_ex(src: &crate::gapi::GMat, op: crate::imgproc::MorphTypes, kernel: &core::Mat, anchor: core::Point, iterations: i32, border_type: core::BorderTypes, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_morphologyEx_const_GMatR_const_MorphTypes_const_MatR_const_PointR_const_int_const_BorderTypes_const_ScalarR(src.as_raw_GMat(), op, kernel.as_raw_Mat(), &anchor, iterations, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
// mulC(const cv::GMat &, const cv::GScalar &, int) /usr/include/opencv2/gapi/core.hpp:742
#[inline]
pub fn mul_c_1(src: &crate::gapi::GMat, multiplier: &crate::gapi::GScalar, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_mulC_const_GMatR_const_GScalarR_int(src.as_raw_GMat(), multiplier.as_raw_GScalar(), ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
// mulC(const cv::GMat &, double, int) /usr/include/opencv2/gapi/core.hpp:740
#[inline]
pub fn mul_c(src: &crate::gapi::GMat, multiplier: f64, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_mulC_const_GMatR_double_int(src.as_raw_GMat(), multiplier, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
// mulC(const cv::GScalar &, const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:744
#[inline]
pub fn mul_c_2(multiplier: &crate::gapi::GScalar, src: &crate::gapi::GMat, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_mulC_const_GScalarR_const_GMatR_int(multiplier.as_raw_GScalar(), src.as_raw_GMat(), ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * scale: 1.0
/// * ddepth: -1
// mul(const cv::GMat &, const cv::GMat &, double, int) /usr/include/opencv2/gapi/core.hpp:722
#[inline]
pub fn mul_4(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, scale: f64, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_mul_const_GMatR_const_GMatR_double_int(src1.as_raw_GMat(), src2.as_raw_GMat(), scale, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// normInf(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1370
#[inline]
pub fn norm_inf(src: &crate::gapi::GMat) -> Result<crate::gapi::GScalar> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_normInf_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
	Ok(ret)
}

// normL1(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1325
#[inline]
pub fn norm_l1(src: &crate::gapi::GMat) -> Result<crate::gapi::GScalar> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_normL1_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
	Ok(ret)
}

// normL2(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1347
#[inline]
pub fn norm_l2(src: &crate::gapi::GMat) -> Result<crate::gapi::GScalar> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_normL2_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
// normalize(const cv::GMat &, double, double, int, int) /usr/include/opencv2/gapi/core.hpp:1738
#[inline]
pub fn normalize(src: &crate::gapi::GMat, alpha: f64, beta: f64, norm_type: i32, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_normalize_const_GMatR_double_double_int_int(src.as_raw_GMat(), alpha, beta, norm_type, ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// descr_of(const cv::gapi::own::Mat &) /usr/include/opencv2/gapi/gmat.hpp:262
#[inline]
pub fn descr_of_1(mat: &crate::gapi::Own_Mat) -> Result<crate::gapi::GMatDesc> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_own_descr_of_const_MatR(mat, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * angle_in_degrees: false
// phase(const cv::GMat &, const cv::GMat &, bool) /usr/include/opencv2/gapi/core.hpp:899
#[inline]
pub fn phase(x: &crate::gapi::GMat, y: &crate::gapi::GMat, angle_in_degrees: bool) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_phase_const_GMatR_const_GMatR_bool(x.as_raw_GMat(), y.as_raw_GMat(), angle_in_degrees, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * border_mode: BORDER_CONSTANT
/// * border_value: Scalar()
// remap(const cv::GMat &, const cv::Mat &, const cv::Mat &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/core.hpp:1551
#[inline]
pub fn remap(src: &crate::gapi::GMat, map1: &core::Mat, map2: &core::Mat, interpolation: i32, border_mode: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_remap_const_GMatR_const_MatR_const_MatR_int_int_const_ScalarR(src.as_raw_GMat(), map1.as_raw_Mat(), map2.as_raw_Mat(), interpolation, border_mode, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * interpolation: cv::INTER_LINEAR
// resizeP(const cv::GMatP &, const cv::Size &, int) /usr/include/opencv2/gapi/imgproc.hpp:1763
#[inline]
pub fn resize_p(src: &crate::gapi::GMatP, dsize: core::Size, interpolation: i32) -> Result<crate::gapi::GMatP> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_resizeP_const_GMatPR_const_SizeR_int(src.as_raw_GMatP(), &dsize, interpolation, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMatP::opencv_from_extern(ret) };
	Ok(ret)
}

// select(const cv::GMat &, const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1181
#[inline]
pub fn select(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, mask: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_select_const_GMatR_const_GMatR_const_GMatR(src1.as_raw_GMat(), src2.as_raw_GMat(), mask.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * border_type: BORDER_DEFAULT
/// * border_value: Scalar(0)
// sepFilter(const cv::GMat &, int, const cv::Mat &, const cv::Mat &, const cv::Point &, const cv::Scalar &, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:559
#[inline]
pub fn sep_filter(src: &crate::gapi::GMat, ddepth: i32, kernel_x: &core::Mat, kernel_y: &core::Mat, anchor: core::Point, delta: core::Scalar, border_type: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_sepFilter_const_GMatR_int_const_MatR_const_MatR_const_PointR_const_ScalarR_int_const_ScalarR(src.as_raw_GMat(), ddepth, kernel_x.as_raw_Mat(), kernel_y.as_raw_Mat(), &anchor, &delta, border_type, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// sqrt(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:910
#[inline]
pub fn sqrt(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_sqrt_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// BGR(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/format.hpp:43
#[inline]
pub fn bgr(in_: &crate::gapi::GFrame) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_streaming_BGR_const_GFrameR(in_.as_raw_GFrame(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// UV(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/format.hpp:63
#[inline]
pub fn uv(frame: &crate::gapi::GFrame) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_streaming_UV_const_GFrameR(frame.as_raw_GFrame(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// Y(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/format.hpp:53
#[inline]
pub fn y(frame: &crate::gapi::GFrame) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_streaming_Y_const_GFrameR(frame.as_raw_GFrame(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// desync(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/desync.hpp:79
#[inline]
pub fn desync_1(f: &crate::gapi::GFrame) -> Result<crate::gapi::GFrame> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_streaming_desync_const_GFrameR(f.as_raw_GFrame(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GFrame::opencv_from_extern(ret) };
	Ok(ret)
}

// desync(const cv::GMat &) /usr/include/opencv2/gapi/streaming/desync.hpp:78
#[inline]
pub fn desync(g: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_streaming_desync_const_GMatR(g.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// kernels() /usr/include/opencv2/gapi/streaming/format.hpp:16
#[inline]
pub fn kernels() -> Result<crate::gapi::GKernelPackage> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_streaming_kernels(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GKernelPackage::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
// subC(const cv::GMat &, const cv::GScalar &, int) /usr/include/opencv2/gapi/core.hpp:682
#[inline]
pub fn sub_c(src: &crate::gapi::GMat, c: &crate::gapi::GScalar, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_subC_const_GMatR_const_GScalarR_int(src.as_raw_GMat(), c.as_raw_GScalar(), ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
// subRC(const cv::GScalar &, const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:701
#[inline]
pub fn sub_rc(c: &crate::gapi::GScalar, src: &crate::gapi::GMat, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_subRC_const_GScalarR_const_GMatR_int(c.as_raw_GScalar(), src.as_raw_GMat(), ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * ddepth: -1
// sub(const cv::GMat &, const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:663
#[inline]
pub fn sub_3(src1: &crate::gapi::GMat, src2: &crate::gapi::GMat, ddepth: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_sub_const_GMatR_const_GMatR_int(src1.as_raw_GMat(), src2.as_raw_GMat(), ddepth, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// sum(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1266
#[inline]
pub fn sum(src: &crate::gapi::GMat) -> Result<crate::gapi::GScalar> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_sum_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
	Ok(ret)
}

// threshold(const cv::GMat &, const cv::GScalar &, const cv::GScalar &, int) /usr/include/opencv2/gapi/core.hpp:1419
#[inline]
pub fn threshold(src: &crate::gapi::GMat, thresh: &crate::gapi::GScalar, maxval: &crate::gapi::GScalar, typ: i32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_threshold_const_GMatR_const_GScalarR_const_GScalarR_int(src.as_raw_GMat(), thresh.as_raw_GScalar(), maxval.as_raw_GScalar(), typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// transpose(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1876
#[inline]
pub fn transpose(src: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_transpose_const_GMatR(src.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: cv::INTER_LINEAR
/// * border_mode: cv::BORDER_CONSTANT
/// * border_value: Scalar()
// warpAffine(const cv::GMat &, const cv::Mat &, const cv::Size &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/core.hpp:1787
#[inline]
pub fn warp_affine(src: &crate::gapi::GMat, m: &core::Mat, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_warpAffine_const_GMatR_const_MatR_const_SizeR_int_int_const_ScalarR(src.as_raw_GMat(), m.as_raw_Mat(), &dsize, flags, border_mode, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * flags: cv::INTER_LINEAR
/// * border_mode: cv::BORDER_CONSTANT
/// * border_value: Scalar()
// warpPerspective(const cv::GMat &, const cv::Mat &, const cv::Size &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/core.hpp:1762
#[inline]
pub fn warp_perspective(src: &crate::gapi::GMat, m: &core::Mat, dsize: core::Size, flags: i32, border_mode: i32, border_value: core::Scalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_gapi_warpPerspective_const_GMatR_const_MatR_const_SizeR_int_int_const_ScalarR(src.as_raw_GMat(), m.as_raw_Mat(), &dsize, flags, border_mode, &border_value, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator+(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:16
#[inline]
pub fn add(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorA_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator+(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:18
#[inline]
pub fn add_1(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorA_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator+(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:19
#[inline]
pub fn add_2(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorA_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator/(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:33
#[inline]
pub fn div_2(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorD_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator/(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:31
#[inline]
pub fn div(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorD_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator/(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:32
#[inline]
pub fn div_1(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorD_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator==(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:52
#[inline]
pub fn equals(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorEQ_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator==(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:59
#[inline]
pub fn equals_1(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorEQ_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator==(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:66
#[inline]
pub fn equals_2(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorEQ_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:21
#[inline]
pub fn sub(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_GMatR_const_GMatR(lhs.as_raw_GMat(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:23
#[inline]
pub fn sub_1(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator-(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:24
#[inline]
pub fn sub_2(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorS_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator*(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:28
#[inline]
pub fn mul_2(lhs: &crate::gapi::GMat, rhs: &crate::gapi::GScalar) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorX_const_GMatR_const_GScalarR(lhs.as_raw_GMat(), rhs.as_raw_GScalar(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator*(const cv::GMat &, float) /usr/include/opencv2/gapi/operators.hpp:26
#[inline]
pub fn mul(lhs: &crate::gapi::GMat, rhs: f32) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorX_const_GMatR_float(lhs.as_raw_GMat(), rhs, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator*(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:29
#[inline]
pub fn mul_3(lhs: &crate::gapi::GScalar, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorX_const_GScalarR_const_GMatR(lhs.as_raw_GScalar(), rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// operator*(float, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:27
#[inline]
pub fn mul_1(lhs: f32, rhs: &crate::gapi::GMat) -> Result<crate::gapi::GMat> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_operatorX_float_const_GMatR(lhs, rhs.as_raw_GMat(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
	Ok(ret)
}

// validate_input_arg(const cv::GRunArg &) /usr/include/opencv2/gapi/gproto.hpp:154
#[inline]
pub fn validate_input_arg(arg: &crate::gapi::GRunArg) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_validate_input_arg_const_GRunArgR(arg.as_raw_GRunArg(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// validate_input_args(const cv::GRunArgs &) /usr/include/opencv2/gapi/gproto.hpp:155
#[inline]
pub fn validate_input_args(args: &crate::gapi::GRunArgs) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_validate_input_args_const_GRunArgsR(args.as_raw_VectorOfGRunArg(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// GArg /usr/include/opencv2/gapi/garg.hpp:46
pub trait GArgTraitConst {
	fn as_raw_GArg(&self) -> *const c_void;

	// kind /usr/include/opencv2/gapi/garg.hpp:87
	#[inline]
	fn kind(&self) -> crate::gapi::ArgKind {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GArg_getPropKind_const(self.as_raw_GArg(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// opaque_kind /usr/include/opencv2/gapi/garg.hpp:88
	#[inline]
	fn opaque_kind(&self) -> crate::gapi::OpaqueKind {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GArg_getPropOpaque_kind_const(self.as_raw_GArg(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
}

pub trait GArgTrait: crate::gapi::GArgTraitConst {
	fn as_raw_mut_GArg(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * val: detail::ArgKind::OPAQUE_VAL
	// kind /usr/include/opencv2/gapi/garg.hpp:87
	#[inline]
	fn set_kind(&mut self, val: crate::gapi::ArgKind) {
		let ret = unsafe { sys::cv_GArg_setPropKind_ArgKind(self.as_raw_mut_GArg(), val) };
		ret
	}
	
	/// ## C++ default parameters
	/// * val: detail::OpaqueKind::CV_UNKNOWN
	// opaque_kind /usr/include/opencv2/gapi/garg.hpp:88
	#[inline]
	fn set_opaque_kind(&mut self, val: crate::gapi::OpaqueKind) {
		let ret = unsafe { sys::cv_GArg_setPropOpaque_kind_OpaqueKind(self.as_raw_mut_GArg(), val) };
		ret
	}
	
}

// GArg /usr/include/opencv2/gapi/garg.hpp:46
pub struct GArg {
	ptr: *mut c_void
}

opencv_type_boxed! { GArg }

impl Drop for GArg {
	fn drop(&mut self) {
		extern "C" { fn cv_GArg_delete(instance: *mut c_void); }
		unsafe { cv_GArg_delete(self.as_raw_mut_GArg()) };
	}
}

unsafe impl Send for GArg {}

impl crate::gapi::GArgTraitConst for GArg {
	#[inline] fn as_raw_GArg(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GArgTrait for GArg {
	#[inline] fn as_raw_mut_GArg(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GArg {
	// GArg() /usr/include/opencv2/gapi/garg.hpp:49
	#[inline]
	pub fn default() -> Result<crate::gapi::GArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GArg_GArg(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// GCall /usr/include/opencv2/gapi/gcall.hpp:26
pub trait GCallTraitConst {
	fn as_raw_GCall(&self) -> *const c_void;

}

pub trait GCallTrait: crate::gapi::GCallTraitConst {
	fn as_raw_mut_GCall(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * output: 0
	// yield(int) /usr/include/opencv2/gapi/gcall.hpp:42
	#[inline]
	fn yield_(&mut self, output: i32) -> Result<crate::gapi::GMat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GCall_yield_int(self.as_raw_mut_GCall(), output, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * output: 0
	// yieldP(int) /usr/include/opencv2/gapi/gcall.hpp:43
	#[inline]
	fn yield_p(&mut self, output: i32) -> Result<crate::gapi::GMatP> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GCall_yieldP_int(self.as_raw_mut_GCall(), output, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMatP::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * output: 0
	// yieldScalar(int) /usr/include/opencv2/gapi/gcall.hpp:44
	#[inline]
	fn yield_scalar(&mut self, output: i32) -> Result<crate::gapi::GScalar> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GCall_yieldScalar_int(self.as_raw_mut_GCall(), output, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GScalar::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * output: 0
	// yieldFrame(int) /usr/include/opencv2/gapi/gcall.hpp:45
	#[inline]
	fn yield_frame(&mut self, output: i32) -> Result<crate::gapi::GFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GCall_yieldFrame_int(self.as_raw_mut_GCall(), output, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GFrame::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// kernel() /usr/include/opencv2/gapi/gcall.hpp:63
	#[inline]
	fn kernel(&mut self) -> Result<crate::gapi::GKernel> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GCall_kernel(self.as_raw_mut_GCall(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GKernel::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// params() /usr/include/opencv2/gapi/gcall.hpp:64
	#[inline]
	fn params(&mut self) -> Result<crate::gapi::any> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GCall_params(self.as_raw_mut_GCall(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setArgs(std::vector<GArg> &&) /usr/include/opencv2/gapi/gcall.hpp:66
	#[inline]
	fn set_args(&mut self, args: &mut core::Vector<crate::gapi::GArg>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GCall_setArgs_vector_GArg_R(self.as_raw_mut_GCall(), args.as_raw_mut_VectorOfGArg(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// GCall /usr/include/opencv2/gapi/gcall.hpp:26
pub struct GCall {
	ptr: *mut c_void
}

opencv_type_boxed! { GCall }

impl Drop for GCall {
	fn drop(&mut self) {
		extern "C" { fn cv_GCall_delete(instance: *mut c_void); }
		unsafe { cv_GCall_delete(self.as_raw_mut_GCall()) };
	}
}

unsafe impl Send for GCall {}

impl crate::gapi::GCallTraitConst for GCall {
	#[inline] fn as_raw_GCall(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GCallTrait for GCall {
	#[inline] fn as_raw_mut_GCall(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GCall {
	// GCall(const cv::GKernel &) /usr/include/opencv2/gapi/gcall.hpp:31
	#[inline]
	pub fn new(k: &crate::gapi::GKernel) -> Result<crate::gapi::GCall> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GCall_GCall_const_GKernelR(k.as_raw_GKernel(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GCall::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// GCompiled /usr/include/opencv2/gapi/gcompiled.hpp:66
pub trait GCompiledTraitConst {
	fn as_raw_GCompiled(&self) -> *const c_void;

	// operator bool() /usr/include/opencv2/gapi/gcompiled.hpp:165
	#[inline]
	fn to_bool(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GCompiled_operator_bool_const(self.as_raw_GCompiled(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// canReshape() /usr/include/opencv2/gapi/gcompiled.hpp:196
	#[inline]
	fn can_reshape(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GCompiled_canReshape_const(self.as_raw_GCompiled(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GCompiledTrait: crate::gapi::GCompiledTraitConst {
	fn as_raw_mut_GCompiled(&mut self) -> *mut c_void;

	// prepareForNewStream() /usr/include/opencv2/gapi/gcompiled.hpp:222
	#[inline]
	fn prepare_for_new_stream(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GCompiled_prepareForNewStream(self.as_raw_mut_GCompiled(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// GCompiled /usr/include/opencv2/gapi/gcompiled.hpp:66
pub struct GCompiled {
	ptr: *mut c_void
}

opencv_type_boxed! { GCompiled }

impl Drop for GCompiled {
	fn drop(&mut self) {
		extern "C" { fn cv_GCompiled_delete(instance: *mut c_void); }
		unsafe { cv_GCompiled_delete(self.as_raw_mut_GCompiled()) };
	}
}

unsafe impl Send for GCompiled {}

impl crate::gapi::GCompiledTraitConst for GCompiled {
	#[inline] fn as_raw_GCompiled(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GCompiledTrait for GCompiled {
	#[inline] fn as_raw_mut_GCompiled(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GCompiled {
	// GCompiled() /usr/include/opencv2/gapi/gcompiled.hpp:75
	#[inline]
	pub fn default() -> Result<crate::gapi::GCompiled> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GCompiled_GCompiled(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GCompiled::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// GFrameDesc /usr/include/opencv2/gapi/gframe.hpp:96
pub trait GFrameDescTraitConst {
	fn as_raw_GFrameDesc(&self) -> *const c_void;

	// fmt /usr/include/opencv2/gapi/gframe.hpp:98
	#[inline]
	fn fmt(&self) -> crate::gapi::MediaFormat {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFrameDesc_getPropFmt_const(self.as_raw_GFrameDesc(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// size /usr/include/opencv2/gapi/gframe.hpp:99
	#[inline]
	fn size(&self) -> core::Size {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFrameDesc_getPropSize_const(self.as_raw_GFrameDesc(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// operator==(const cv::GFrameDesc &) /usr/include/opencv2/gapi/gframe.hpp:101
	#[inline]
	fn equals(&self, unnamed: &crate::gapi::GFrameDesc) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GFrameDesc_operatorEQ_const_const_GFrameDescR(self.as_raw_GFrameDesc(), unnamed.as_raw_GFrameDesc(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GFrameDescTrait: crate::gapi::GFrameDescTraitConst {
	fn as_raw_mut_GFrameDesc(&mut self) -> *mut c_void;

	// fmt /usr/include/opencv2/gapi/gframe.hpp:98
	#[inline]
	fn set_fmt(&mut self, val: crate::gapi::MediaFormat) {
		let ret = unsafe { sys::cv_GFrameDesc_setPropFmt_MediaFormat(self.as_raw_mut_GFrameDesc(), val) };
		ret
	}
	
	// size /usr/include/opencv2/gapi/gframe.hpp:99
	#[inline]
	fn set_size(&mut self, val: core::Size) {
		let ret = unsafe { sys::cv_GFrameDesc_setPropSize_Size(self.as_raw_mut_GFrameDesc(), val.opencv_as_extern()) };
		ret
	}
	
}

// GFrameDesc /usr/include/opencv2/gapi/gframe.hpp:96
pub struct GFrameDesc {
	ptr: *mut c_void
}

opencv_type_boxed! { GFrameDesc }

impl Drop for GFrameDesc {
	fn drop(&mut self) {
		extern "C" { fn cv_GFrameDesc_delete(instance: *mut c_void); }
		unsafe { cv_GFrameDesc_delete(self.as_raw_mut_GFrameDesc()) };
	}
}

unsafe impl Send for GFrameDesc {}

impl crate::gapi::GFrameDescTraitConst for GFrameDesc {
	#[inline] fn as_raw_GFrameDesc(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GFrameDescTrait for GFrameDesc {
	#[inline] fn as_raw_mut_GFrameDesc(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GFrameDesc {
}

// GKernel /usr/include/opencv2/gapi/gkernel.hpp:44
pub trait GKernelTraitConst {
	fn as_raw_GKernel(&self) -> *const c_void;

	// name /usr/include/opencv2/gapi/gkernel.hpp:48
	#[inline]
	fn name(&self) -> String {
		let ret = unsafe { sys::cv_GKernel_getPropName_const(self.as_raw_GKernel()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// tag /usr/include/opencv2/gapi/gkernel.hpp:49
	#[inline]
	fn tag(&self) -> String {
		let ret = unsafe { sys::cv_GKernel_getPropTag_const(self.as_raw_GKernel()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
	// outShapes /usr/include/opencv2/gapi/gkernel.hpp:51
	#[inline]
	fn out_shapes(&self) -> core::Vector<crate::gapi::GShape> {
		let ret = unsafe { sys::cv_GKernel_getPropOutShapes_const(self.as_raw_GKernel()) };
		let ret = unsafe { core::Vector::<crate::gapi::GShape>::opencv_from_extern(ret) };
		ret
	}
	
	// inKinds /usr/include/opencv2/gapi/gkernel.hpp:52
	#[inline]
	fn in_kinds(&self) -> core::Vector<crate::gapi::OpaqueKind> {
		let ret = unsafe { sys::cv_GKernel_getPropInKinds_const(self.as_raw_GKernel()) };
		let ret = unsafe { core::Vector::<crate::gapi::OpaqueKind>::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait GKernelTrait: crate::gapi::GKernelTraitConst {
	fn as_raw_mut_GKernel(&mut self) -> *mut c_void;

	// name /usr/include/opencv2/gapi/gkernel.hpp:48
	#[inline]
	fn set_name(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_GKernel_setPropName_string(self.as_raw_mut_GKernel(), val.opencv_as_extern_mut()) };
		ret
	}
	
	// tag /usr/include/opencv2/gapi/gkernel.hpp:49
	#[inline]
	fn set_tag(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_GKernel_setPropTag_string(self.as_raw_mut_GKernel(), val.opencv_as_extern_mut()) };
		ret
	}
	
	// outShapes /usr/include/opencv2/gapi/gkernel.hpp:51
	#[inline]
	fn set_out_shapes(&mut self, mut val: crate::gapi::GShapes) {
		let ret = unsafe { sys::cv_GKernel_setPropOutShapes_GShapes(self.as_raw_mut_GKernel(), val.as_raw_mut_VectorOfGShape()) };
		ret
	}
	
	// inKinds /usr/include/opencv2/gapi/gkernel.hpp:52
	#[inline]
	fn set_in_kinds(&mut self, mut val: crate::gapi::GKinds) {
		let ret = unsafe { sys::cv_GKernel_setPropInKinds_GKinds(self.as_raw_mut_GKernel(), val.as_raw_mut_VectorOfOpaqueKind()) };
		ret
	}
	
}

// GKernel /usr/include/opencv2/gapi/gkernel.hpp:44
pub struct GKernel {
	ptr: *mut c_void
}

opencv_type_boxed! { GKernel }

impl Drop for GKernel {
	fn drop(&mut self) {
		extern "C" { fn cv_GKernel_delete(instance: *mut c_void); }
		unsafe { cv_GKernel_delete(self.as_raw_mut_GKernel()) };
	}
}

unsafe impl Send for GKernel {}

impl crate::gapi::GKernelTraitConst for GKernel {
	#[inline] fn as_raw_GKernel(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GKernelTrait for GKernel {
	#[inline] fn as_raw_mut_GKernel(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GKernel {
}

// GKernelImpl /usr/include/opencv2/gapi/gkernel.hpp:59
pub trait GKernelImplTraitConst {
	fn as_raw_GKernelImpl(&self) -> *const c_void;

	// opaque /usr/include/opencv2/gapi/gkernel.hpp:61
	#[inline]
	fn opaque(&self) -> crate::gapi::any {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GKernelImpl_getPropOpaque_const(self.as_raw_GKernelImpl(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
}

pub trait GKernelImplTrait: crate::gapi::GKernelImplTraitConst {
	fn as_raw_mut_GKernelImpl(&mut self) -> *mut c_void;

	// opaque /usr/include/opencv2/gapi/gkernel.hpp:61
	#[inline]
	fn set_opaque(&mut self, val: crate::gapi::any) {
		let ret = unsafe { sys::cv_GKernelImpl_setPropOpaque_any(self.as_raw_mut_GKernelImpl(), val) };
		ret
	}
	
}

// GKernelImpl /usr/include/opencv2/gapi/gkernel.hpp:59
pub struct GKernelImpl {
	ptr: *mut c_void
}

opencv_type_boxed! { GKernelImpl }

impl Drop for GKernelImpl {
	fn drop(&mut self) {
		extern "C" { fn cv_GKernelImpl_delete(instance: *mut c_void); }
		unsafe { cv_GKernelImpl_delete(self.as_raw_mut_GKernelImpl()) };
	}
}

unsafe impl Send for GKernelImpl {}

impl crate::gapi::GKernelImplTraitConst for GKernelImpl {
	#[inline] fn as_raw_GKernelImpl(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GKernelImplTrait for GKernelImpl {
	#[inline] fn as_raw_mut_GKernelImpl(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GKernelImpl {
}

// GMatP /usr/include/opencv2/gapi/gmat.hpp:90
pub trait GMatPTraitConst: crate::gapi::GMatTraitConst {
	fn as_raw_GMatP(&self) -> *const c_void;

}

pub trait GMatPTrait: crate::gapi::GMatPTraitConst + crate::gapi::GMatTrait {
	fn as_raw_mut_GMatP(&mut self) -> *mut c_void;

}

// GMatP /usr/include/opencv2/gapi/gmat.hpp:90
pub struct GMatP {
	ptr: *mut c_void
}

opencv_type_boxed! { GMatP }

impl Drop for GMatP {
	fn drop(&mut self) {
		extern "C" { fn cv_GMatP_delete(instance: *mut c_void); }
		unsafe { cv_GMatP_delete(self.as_raw_mut_GMatP()) };
	}
}

unsafe impl Send for GMatP {}

impl crate::gapi::GMatTraitConst for GMatP {
	#[inline] fn as_raw_GMat(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GMatTrait for GMatP {
	#[inline] fn as_raw_mut_GMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::gapi::GMatPTraitConst for GMatP {
	#[inline] fn as_raw_GMatP(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GMatPTrait for GMatP {
	#[inline] fn as_raw_mut_GMatP(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GMatP {
}

boxed_cast_base! { GMatP, crate::gapi::GMat, cv_GMatP_to_GMat }

// GRunArg /usr/include/opencv2/gapi/garg.hpp:121
pub trait GRunArgTraitConst {
	fn as_raw_GRunArg(&self) -> *const c_void;

}

pub trait GRunArgTrait: crate::gapi::GRunArgTraitConst {
	fn as_raw_mut_GRunArg(&mut self) -> *mut c_void;

}

// GRunArg /usr/include/opencv2/gapi/garg.hpp:121
pub struct GRunArg {
	ptr: *mut c_void
}

opencv_type_boxed! { GRunArg }

impl Drop for GRunArg {
	fn drop(&mut self) {
		extern "C" { fn cv_GRunArg_delete(instance: *mut c_void); }
		unsafe { cv_GRunArg_delete(self.as_raw_mut_GRunArg()) };
	}
}

unsafe impl Send for GRunArg {}

impl crate::gapi::GRunArgTraitConst for GRunArg {
	#[inline] fn as_raw_GRunArg(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GRunArgTrait for GRunArg {
	#[inline] fn as_raw_mut_GRunArg(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GRunArg {
	// GRunArg() /usr/include/opencv2/gapi/garg.hpp:129
	#[inline]
	pub fn default() -> Result<crate::gapi::GRunArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GRunArg_GRunArg(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GRunArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// GRunArg(const cv::GRunArg &) /usr/include/opencv2/gapi/garg.hpp:130
	#[inline]
	pub fn copy(arg: &crate::gapi::GRunArg) -> Result<crate::gapi::GRunArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GRunArg_GRunArg_const_GRunArgR(arg.as_raw_GRunArg(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GRunArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// GRunArg(cv::GRunArg &&) /usr/include/opencv2/gapi/garg.hpp:131
	#[inline]
	pub fn copy_mut(arg: &mut crate::gapi::GRunArg) -> Result<crate::gapi::GRunArg> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GRunArg_GRunArg_GRunArgR(arg.as_raw_mut_GRunArg(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GRunArg::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// GTransform /usr/include/opencv2/gapi/gtransform.hpp:24
pub trait GTransformTraitConst {
	fn as_raw_GTransform(&self) -> *const c_void;

	// description /usr/include/opencv2/gapi/gtransform.hpp:30
	#[inline]
	fn description(&self) -> String {
		let ret = unsafe { sys::cv_GTransform_getPropDescription_const(self.as_raw_GTransform()) };
		let ret = unsafe { String::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait GTransformTrait: crate::gapi::GTransformTraitConst {
	fn as_raw_mut_GTransform(&mut self) -> *mut c_void;

	// description /usr/include/opencv2/gapi/gtransform.hpp:30
	#[inline]
	fn set_description(&mut self, val: &str) {
		extern_container_arg!(nofail mut val);
		let ret = unsafe { sys::cv_GTransform_setPropDescription_string(self.as_raw_mut_GTransform(), val.opencv_as_extern_mut()) };
		ret
	}
	
}

// GTransform /usr/include/opencv2/gapi/gtransform.hpp:24
pub struct GTransform {
	ptr: *mut c_void
}

opencv_type_boxed! { GTransform }

impl Drop for GTransform {
	fn drop(&mut self) {
		extern "C" { fn cv_GTransform_delete(instance: *mut c_void); }
		unsafe { cv_GTransform_delete(self.as_raw_mut_GTransform()) };
	}
}

unsafe impl Send for GTransform {}

impl crate::gapi::GTransformTraitConst for GTransform {
	#[inline] fn as_raw_GTransform(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GTransformTrait for GTransform {
	#[inline] fn as_raw_mut_GTransform(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GTransform {
}

// MediaFrame /usr/include/opencv2/gapi/media.hpp:52
pub trait MediaFrameTraitConst {
	fn as_raw_MediaFrame(&self) -> *const c_void;

	// access(cv::MediaFrame::Access) /usr/include/opencv2/gapi/media.hpp:103
	#[inline]
	fn access(&self, mode: crate::gapi::MediaFrame_Access) -> Result<crate::gapi::MediaFrame_View> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MediaFrame_access_const_Access(self.as_raw_MediaFrame(), mode, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::MediaFrame_View::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// desc() /usr/include/opencv2/gapi/media.hpp:110
	#[inline]
	fn desc(&self) -> Result<crate::gapi::GFrameDesc> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MediaFrame_desc_const(self.as_raw_MediaFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GFrameDesc::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// blobParams() /usr/include/opencv2/gapi/media.hpp:115
	#[inline]
	fn blob_params(&self) -> Result<crate::gapi::any> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MediaFrame_blobParams_const(self.as_raw_MediaFrame(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait MediaFrameTrait: crate::gapi::MediaFrameTraitConst {
	fn as_raw_mut_MediaFrame(&mut self) -> *mut c_void;

}

// MediaFrame /usr/include/opencv2/gapi/media.hpp:52
pub struct MediaFrame {
	ptr: *mut c_void
}

opencv_type_boxed! { MediaFrame }

impl Drop for MediaFrame {
	fn drop(&mut self) {
		extern "C" { fn cv_MediaFrame_delete(instance: *mut c_void); }
		unsafe { cv_MediaFrame_delete(self.as_raw_mut_MediaFrame()) };
	}
}

unsafe impl Send for MediaFrame {}

impl crate::gapi::MediaFrameTraitConst for MediaFrame {
	#[inline] fn as_raw_MediaFrame(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::MediaFrameTrait for MediaFrame {
	#[inline] fn as_raw_mut_MediaFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MediaFrame {
	// MediaFrame() /usr/include/opencv2/gapi/media.hpp:70
	#[inline]
	pub fn default() -> Result<crate::gapi::MediaFrame> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MediaFrame_MediaFrame(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::MediaFrame::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// IAdapter /usr/include/opencv2/gapi/media.hpp:236
pub trait MediaFrame_IAdapterConst {
	fn as_raw_MediaFrame_IAdapter(&self) -> *const c_void;

	// meta() /usr/include/opencv2/gapi/media.hpp:239
	#[inline]
	fn meta(&self) -> Result<crate::gapi::GFrameDesc> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MediaFrame_IAdapter_meta_const(self.as_raw_MediaFrame_IAdapter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GFrameDesc::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// blobParams() /usr/include/opencv2/gapi/media.hpp:243
	#[inline]
	fn blob_params(&self) -> Result<crate::gapi::any> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MediaFrame_IAdapter_blobParams_const(self.as_raw_MediaFrame_IAdapter(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait MediaFrame_IAdapter: crate::gapi::MediaFrame_IAdapterConst {
	fn as_raw_mut_MediaFrame_IAdapter(&mut self) -> *mut c_void;

	// access(MediaFrame::Access) /usr/include/opencv2/gapi/media.hpp:240
	#[inline]
	fn access(&mut self, unnamed: crate::gapi::MediaFrame_Access) -> Result<crate::gapi::MediaFrame_View> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_MediaFrame_IAdapter_access_Access(self.as_raw_mut_MediaFrame_IAdapter(), unnamed, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::MediaFrame_View::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// View /usr/include/opencv2/gapi/media.hpp:197
pub trait MediaFrame_ViewTraitConst {
	fn as_raw_MediaFrame_View(&self) -> *const c_void;

}

pub trait MediaFrame_ViewTrait: crate::gapi::MediaFrame_ViewTraitConst {
	fn as_raw_mut_MediaFrame_View(&mut self) -> *mut c_void;

}

// View /usr/include/opencv2/gapi/media.hpp:197
pub struct MediaFrame_View {
	ptr: *mut c_void
}

opencv_type_boxed! { MediaFrame_View }

impl Drop for MediaFrame_View {
	fn drop(&mut self) {
		extern "C" { fn cv_MediaFrame_View_delete(instance: *mut c_void); }
		unsafe { cv_MediaFrame_View_delete(self.as_raw_mut_MediaFrame_View()) };
	}
}

unsafe impl Send for MediaFrame_View {}

impl crate::gapi::MediaFrame_ViewTraitConst for MediaFrame_View {
	#[inline] fn as_raw_MediaFrame_View(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::MediaFrame_ViewTrait for MediaFrame_View {
	#[inline] fn as_raw_mut_MediaFrame_View(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MediaFrame_View {
	// MAX_PLANES /usr/include/opencv2/gapi/media.hpp:199
	pub const MAX_PLANES: u32 = 4;
	// View(cv::MediaFrame::View &&) /usr/include/opencv2/gapi/media.hpp:211
	#[inline]
	pub fn copy_mut(unnamed: &mut crate::gapi::MediaFrame_View) -> crate::gapi::MediaFrame_View {
		let ret = unsafe { sys::cv_MediaFrame_View_View_ViewR(unnamed.as_raw_mut_MediaFrame_View()) };
		let ret = unsafe { crate::gapi::MediaFrame_View::opencv_from_extern(ret) };
		ret
	}
	
}

// RMat /usr/include/opencv2/gapi/rmat.hpp:48
pub trait RMatTraitConst {
	fn as_raw_RMat(&self) -> *const c_void;

	// desc() /usr/include/opencv2/gapi/rmat.hpp:128
	#[inline]
	fn desc(&self) -> Result<crate::gapi::GMatDesc> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RMat_desc_const(self.as_raw_RMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GMatDesc::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// access(cv::RMat::Access) /usr/include/opencv2/gapi/rmat.hpp:135
	#[inline]
	fn access(&self, a: crate::gapi::RMat_Access) -> Result<crate::gapi::RMat_View> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RMat_access_const_Access(self.as_raw_RMat(), a, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::RMat_View::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait RMatTrait: crate::gapi::RMatTraitConst {
	fn as_raw_mut_RMat(&mut self) -> *mut c_void;

}

// RMat /usr/include/opencv2/gapi/rmat.hpp:48
pub struct RMat {
	ptr: *mut c_void
}

opencv_type_boxed! { RMat }

impl Drop for RMat {
	fn drop(&mut self) {
		extern "C" { fn cv_RMat_delete(instance: *mut c_void); }
		unsafe { cv_RMat_delete(self.as_raw_mut_RMat()) };
	}
}

unsafe impl Send for RMat {}

impl crate::gapi::RMatTraitConst for RMat {
	#[inline] fn as_raw_RMat(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::RMatTrait for RMat {
	#[inline] fn as_raw_mut_RMat(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RMat {
	// RMat() /usr/include/opencv2/gapi/rmat.hpp:126
	#[inline]
	pub fn default() -> crate::gapi::RMat {
		let ret = unsafe { sys::cv_RMat_RMat() };
		let ret = unsafe { crate::gapi::RMat::opencv_from_extern(ret) };
		ret
	}
	
}

impl Default for RMat {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// View /usr/include/opencv2/gapi/rmat.hpp:56
pub trait RMat_ViewTraitConst {
	fn as_raw_RMat_View(&self) -> *const c_void;

	// size() /usr/include/opencv2/gapi/rmat.hpp:72
	#[inline]
	fn size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RMat_View_size_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// dims() /usr/include/opencv2/gapi/rmat.hpp:73
	#[inline]
	fn dims(&self) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RMat_View_dims_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// cols() /usr/include/opencv2/gapi/rmat.hpp:74
	#[inline]
	fn cols(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RMat_View_cols_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// rows() /usr/include/opencv2/gapi/rmat.hpp:75
	#[inline]
	fn rows(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RMat_View_rows_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// type() /usr/include/opencv2/gapi/rmat.hpp:76
	#[inline]
	fn typ(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RMat_View_type_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// depth() /usr/include/opencv2/gapi/rmat.hpp:77
	#[inline]
	fn depth(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RMat_View_depth_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// chan() /usr/include/opencv2/gapi/rmat.hpp:78
	#[inline]
	fn chan(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RMat_View_chan_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// elemSize() /usr/include/opencv2/gapi/rmat.hpp:79
	#[inline]
	fn elem_size(&self) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RMat_View_elemSize_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * i: 0
	// step(size_t) /usr/include/opencv2/gapi/rmat.hpp:93
	#[inline]
	fn step(&self, i: size_t) -> Result<size_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RMat_View_step_const_size_t(self.as_raw_RMat_View(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// steps() /usr/include/opencv2/gapi/rmat.hpp:94
	#[inline]
	fn steps(&self) -> Result<core::Vector<size_t>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_RMat_View_steps_const(self.as_raw_RMat_View(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<size_t>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait RMat_ViewTrait: crate::gapi::RMat_ViewTraitConst {
	fn as_raw_mut_RMat_View(&mut self) -> *mut c_void;

}

// View /usr/include/opencv2/gapi/rmat.hpp:56
pub struct RMat_View {
	ptr: *mut c_void
}

opencv_type_boxed! { RMat_View }

impl Drop for RMat_View {
	fn drop(&mut self) {
		extern "C" { fn cv_RMat_View_delete(instance: *mut c_void); }
		unsafe { cv_RMat_View_delete(self.as_raw_mut_RMat_View()) };
	}
}

unsafe impl Send for RMat_View {}

impl crate::gapi::RMat_ViewTraitConst for RMat_View {
	#[inline] fn as_raw_RMat_View(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::RMat_ViewTrait for RMat_View {
	#[inline] fn as_raw_mut_RMat_View(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl RMat_View {
	// View() /usr/include/opencv2/gapi/rmat.hpp:62
	#[inline]
	pub fn default() -> crate::gapi::RMat_View {
		let ret = unsafe { sys::cv_RMat_View_View() };
		let ret = unsafe { crate::gapi::RMat_View::opencv_from_extern(ret) };
		ret
	}
	
	// View(cv::RMat::View &&) /usr/include/opencv2/gapi/rmat.hpp:68
	#[inline]
	pub fn copy_mut(unnamed: &mut crate::gapi::RMat_View) -> crate::gapi::RMat_View {
		let ret = unsafe { sys::cv_RMat_View_View_ViewR(unnamed.as_raw_mut_RMat_View()) };
		let ret = unsafe { crate::gapi::RMat_View::opencv_from_extern(ret) };
		ret
	}
	
}

impl Default for RMat_View {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// GArrayU /usr/include/opencv2/gapi/garray.hpp:70
pub trait GArrayUTraitConst {
	fn as_raw_GArrayU(&self) -> *const c_void;

}

pub trait GArrayUTrait: crate::gapi::GArrayUTraitConst {
	fn as_raw_mut_GArrayU(&mut self) -> *mut c_void;

}

// GArrayU /usr/include/opencv2/gapi/garray.hpp:70
pub struct GArrayU {
	ptr: *mut c_void
}

opencv_type_boxed! { GArrayU }

impl Drop for GArrayU {
	fn drop(&mut self) {
		extern "C" { fn cv_GArrayU_delete(instance: *mut c_void); }
		unsafe { cv_GArrayU_delete(self.as_raw_mut_GArrayU()) };
	}
}

unsafe impl Send for GArrayU {}

impl crate::gapi::GArrayUTraitConst for GArrayU {
	#[inline] fn as_raw_GArrayU(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GArrayUTrait for GArrayU {
	#[inline] fn as_raw_mut_GArrayU(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GArrayU {
}

// GOpaqueU /usr/include/opencv2/gapi/gopaque.hpp:67
pub trait GOpaqueUTraitConst {
	fn as_raw_GOpaqueU(&self) -> *const c_void;

}

pub trait GOpaqueUTrait: crate::gapi::GOpaqueUTraitConst {
	fn as_raw_mut_GOpaqueU(&mut self) -> *mut c_void;

}

// GOpaqueU /usr/include/opencv2/gapi/gopaque.hpp:67
pub struct GOpaqueU {
	ptr: *mut c_void
}

opencv_type_boxed! { GOpaqueU }

impl Drop for GOpaqueU {
	fn drop(&mut self) {
		extern "C" { fn cv_GOpaqueU_delete(instance: *mut c_void); }
		unsafe { cv_GOpaqueU_delete(self.as_raw_mut_GOpaqueU()) };
	}
}

unsafe impl Send for GOpaqueU {}

impl crate::gapi::GOpaqueUTraitConst for GOpaqueU {
	#[inline] fn as_raw_GOpaqueU(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GOpaqueUTrait for GOpaqueU {
	#[inline] fn as_raw_mut_GOpaqueU(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GOpaqueU {
}

// GBackend /usr/include/opencv2/gapi/gkernel.hpp:376
pub trait GBackendTraitConst {
	fn as_raw_GBackend(&self) -> *const c_void;

	// operator==(const cv::gapi::GBackend &) /usr/include/opencv2/gapi/gkernel.hpp:389
	#[inline]
	fn equals(&self, rhs: &crate::gapi::GBackend) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_GBackend_operatorEQ_const_const_GBackendR(self.as_raw_GBackend(), rhs.as_raw_GBackend(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait GBackendTrait: crate::gapi::GBackendTraitConst {
	fn as_raw_mut_GBackend(&mut self) -> *mut c_void;

}

// GBackend /usr/include/opencv2/gapi/gkernel.hpp:376
pub struct GBackend {
	ptr: *mut c_void
}

opencv_type_boxed! { GBackend }

impl Drop for GBackend {
	fn drop(&mut self) {
		extern "C" { fn cv_GBackend_delete(instance: *mut c_void); }
		unsafe { cv_GBackend_delete(self.as_raw_mut_GBackend()) };
	}
}

unsafe impl Send for GBackend {}

impl crate::gapi::GBackendTraitConst for GBackend {
	#[inline] fn as_raw_GBackend(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GBackendTrait for GBackend {
	#[inline] fn as_raw_mut_GBackend(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GBackend {
	// GBackend() /usr/include/opencv2/gapi/gkernel.hpp:382
	#[inline]
	pub fn default() -> Result<crate::gapi::GBackend> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_GBackend_GBackend(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::GBackend::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// Scalar /usr/include/opencv2/gapi/own/scalar.hpp:20
pub trait Own_ScalarTraitConst {
	fn as_raw_Own_Scalar(&self) -> *const c_void;

	// operator[](int) /usr/include/opencv2/gapi/own/scalar.hpp:30
	#[inline]
	fn get(&self, i: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_own_Scalar_operator___const_int(self.as_raw_Own_Scalar(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait Own_ScalarTrait: crate::gapi::Own_ScalarTraitConst {
	fn as_raw_mut_Own_Scalar(&mut self) -> *mut c_void;

	// val /usr/include/opencv2/gapi/own/scalar.hpp:35
	#[inline]
	fn val(&mut self) -> &mut [f64; 4] {
		let ret = unsafe { sys::cv_gapi_own_Scalar_getPropVal(self.as_raw_mut_Own_Scalar()) };
		let ret = unsafe { ret.as_mut() }.expect("Function returned null pointer");
		ret
	}
	
	// operator[](int) /usr/include/opencv2/gapi/own/scalar.hpp:31
	#[inline]
	fn get_mut(&mut self, i: i32) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_own_Scalar_operator___int(self.as_raw_mut_Own_Scalar(), i, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Scalar /usr/include/opencv2/gapi/own/scalar.hpp:20
pub struct Own_Scalar {
	ptr: *mut c_void
}

opencv_type_boxed! { Own_Scalar }

impl Drop for Own_Scalar {
	fn drop(&mut self) {
		extern "C" { fn cv_Own_Scalar_delete(instance: *mut c_void); }
		unsafe { cv_Own_Scalar_delete(self.as_raw_mut_Own_Scalar()) };
	}
}

unsafe impl Send for Own_Scalar {}

impl crate::gapi::Own_ScalarTraitConst for Own_Scalar {
	#[inline] fn as_raw_Own_Scalar(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::Own_ScalarTrait for Own_Scalar {
	#[inline] fn as_raw_mut_Own_Scalar(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Own_Scalar {
	// Scalar() /usr/include/opencv2/gapi/own/scalar.hpp:23
	#[inline]
	pub fn default() -> crate::gapi::Own_Scalar {
		let ret = unsafe { sys::cv_gapi_own_Scalar_Scalar() };
		let ret = unsafe { crate::gapi::Own_Scalar::opencv_from_extern(ret) };
		ret
	}
	
	// Scalar(double) /usr/include/opencv2/gapi/own/scalar.hpp:24
	#[inline]
	pub fn new(v0: f64) -> Result<crate::gapi::Own_Scalar> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_own_Scalar_Scalar_double(v0, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::Own_Scalar::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * v2: 0
	/// * v3: 0
	// Scalar(double, double, double, double) /usr/include/opencv2/gapi/own/scalar.hpp:25
	#[inline]
	pub fn new_1(v0: f64, v1: f64, v2: f64, v3: f64) -> Result<crate::gapi::Own_Scalar> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_own_Scalar_Scalar_double_double_double_double(v0, v1, v2, v3, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::Own_Scalar::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// all(double) /usr/include/opencv2/gapi/own/scalar.hpp:33
	#[inline]
	pub fn all(v0: f64) -> Result<crate::gapi::Own_Scalar> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_gapi_own_Scalar_all_double(v0, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::gapi::Own_Scalar::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Default for Own_Scalar {
	#[inline]
	/// Forwards to infallible Self::default()
	fn default() -> Self {
		Self::default()
	}
}

// use_only /usr/include/opencv2/gapi/gkernel.hpp:729
pub trait use_onlyTraitConst {
	fn as_raw_use_only(&self) -> *const c_void;

	// pkg /usr/include/opencv2/gapi/gkernel.hpp:731
	#[inline]
	fn pkg(&self) -> crate::gapi::GKernelPackage {
		let ret = unsafe { sys::cv_gapi_use_only_getPropPkg_const(self.as_raw_use_only()) };
		let ret = unsafe { crate::gapi::GKernelPackage::opencv_from_extern(ret) };
		ret
	}
	
}

pub trait use_onlyTrait: crate::gapi::use_onlyTraitConst {
	fn as_raw_mut_use_only(&mut self) -> *mut c_void;

	// pkg /usr/include/opencv2/gapi/gkernel.hpp:731
	#[inline]
	fn set_pkg(&mut self, mut val: crate::gapi::GKernelPackage) {
		let ret = unsafe { sys::cv_gapi_use_only_setPropPkg_GKernelPackage(self.as_raw_mut_use_only(), val.as_raw_mut_GKernelPackage()) };
		ret
	}
	
}

// use_only /usr/include/opencv2/gapi/gkernel.hpp:729
pub struct use_only {
	ptr: *mut c_void
}

opencv_type_boxed! { use_only }

impl Drop for use_only {
	fn drop(&mut self) {
		extern "C" { fn cv_use_only_delete(instance: *mut c_void); }
		unsafe { cv_use_only_delete(self.as_raw_mut_use_only()) };
	}
}

unsafe impl Send for use_only {}

impl crate::gapi::use_onlyTraitConst for use_only {
	#[inline] fn as_raw_use_only(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::use_onlyTrait for use_only {
	#[inline] fn as_raw_mut_use_only(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl use_only {
}

// Data /usr/include/opencv2/gapi/garg.hpp:195
pub trait DataTraitConst: crate::gapi::GRunArgTraitConst {
	fn as_raw_Data(&self) -> *const c_void;

}

pub trait DataTrait: crate::gapi::DataTraitConst + crate::gapi::GRunArgTrait {
	fn as_raw_mut_Data(&mut self) -> *mut c_void;

}

// Data /usr/include/opencv2/gapi/garg.hpp:195
pub struct Data {
	ptr: *mut c_void
}

opencv_type_boxed! { Data }

impl Drop for Data {
	fn drop(&mut self) {
		extern "C" { fn cv_Data_delete(instance: *mut c_void); }
		unsafe { cv_Data_delete(self.as_raw_mut_Data()) };
	}
}

unsafe impl Send for Data {}

impl crate::gapi::GRunArgTraitConst for Data {
	#[inline] fn as_raw_GRunArg(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::GRunArgTrait for Data {
	#[inline] fn as_raw_mut_GRunArg(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::gapi::DataTraitConst for Data {
	#[inline] fn as_raw_Data(&self) -> *const c_void { self.as_raw() }
}

impl crate::gapi::DataTrait for Data {
	#[inline] fn as_raw_mut_Data(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Data {
}

boxed_cast_base! { Data, crate::gapi::GRunArg, cv_Data_to_GRunArg }
