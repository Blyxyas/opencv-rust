#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Extra 2D Features Framework
//!    # Experimental 2D Features Algorithms
//! 
//! This section describes experimental algorithms for 2d feature detection.
//! 
//!    # Non-free 2D Features Algorithms
//! 
//! This section describes two popular algorithms for 2d feature detection, SIFT and SURF, that are
//! known to be patented. You need to set the OPENCV_ENABLE_NONFREE option in cmake to use those. Use them at your own risk.
//! 
//!    # Experimental 2D Features Matching Algorithm
//! 
//! This section describes the following matching strategies:
//!    - GMS: Grid-based Motion Statistics, [Bian2017gms](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Bian2017gms)
//!    - LOGOS: Local geometric support for high-outlier spatial verification, [Lowry2018LOGOSLG](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Lowry2018LOGOSLG)
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::SURFConst, super::SURF, super::FREAKTraitConst, super::FREAKTrait, super::StarDetectorTraitConst, super::StarDetectorTrait, super::BriefDescriptorExtractorTraitConst, super::BriefDescriptorExtractorTrait, super::LUCIDTraitConst, super::LUCIDTrait, super::LATCHTraitConst, super::LATCHTrait, super::BEBLIDTraitConst, super::BEBLIDTrait, super::DAISYConst, super::DAISY, super::MSDDetectorTraitConst, super::MSDDetectorTrait, super::VGGConst, super::VGG, super::BoostDescConst, super::BoostDesc, super::PCTSignaturesConst, super::PCTSignatures, super::PCTSignaturesSQFDConst, super::PCTSignaturesSQFD, super::Elliptic_KeyPointTraitConst, super::Elliptic_KeyPointTrait, super::HarrisLaplaceFeatureDetectorTraitConst, super::HarrisLaplaceFeatureDetectorTrait, super::AffineFeature2DConst, super::AffineFeature2D, super::TBMRConst, super::TBMR, super::SURF_CUDATraitConst, super::SURF_CUDATrait };
}

// BGM /usr/include/opencv2/xfeatures2d.hpp:415
pub const BoostDesc_BGM: i32 = 100;
// BGM_BILINEAR /usr/include/opencv2/xfeatures2d.hpp:415
pub const BoostDesc_BGM_BILINEAR: i32 = 102;
// BGM_HARD /usr/include/opencv2/xfeatures2d.hpp:415
pub const BoostDesc_BGM_HARD: i32 = 101;
// BINBOOST_128 /usr/include/opencv2/xfeatures2d.hpp:416
pub const BoostDesc_BINBOOST_128: i32 = 301;
// BINBOOST_256 /usr/include/opencv2/xfeatures2d.hpp:416
pub const BoostDesc_BINBOOST_256: i32 = 302;
// BINBOOST_64 /usr/include/opencv2/xfeatures2d.hpp:416
pub const BoostDesc_BINBOOST_64: i32 = 300;
// LBGM /usr/include/opencv2/xfeatures2d.hpp:415
pub const BoostDesc_LBGM: i32 = 200;
// VGG_120 /usr/include/opencv2/xfeatures2d.hpp:359
pub const VGG_VGG_120: i32 = 100;
// VGG_48 /usr/include/opencv2/xfeatures2d.hpp:359
pub const VGG_VGG_48: i32 = 103;
// VGG_64 /usr/include/opencv2/xfeatures2d.hpp:359
pub const VGG_VGG_64: i32 = 102;
// VGG_80 /usr/include/opencv2/xfeatures2d.hpp:359
pub const VGG_VGG_80: i32 = 101;
// BeblidSize /usr/include/opencv2/xfeatures2d.hpp:211
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BEBLID_BeblidSize {
	SIZE_512_BITS = 100,
	SIZE_256_BITS = 101,
}

opencv_type_enum! { crate::xfeatures2d::BEBLID_BeblidSize }

// NormalizationType /usr/include/opencv2/xfeatures2d.hpp:246
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DAISY_NormalizationType {
	NRM_NONE = 100,
	NRM_PARTIAL = 101,
	NRM_FULL = 102,
	NRM_SIFT = 103,
}

opencv_type_enum! { crate::xfeatures2d::DAISY_NormalizationType }

// DistanceFunction /usr/include/opencv2/xfeatures2d.hpp:454
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PCTSignatures_DistanceFunction {
	L0_25 = 0,
	L0_5 = 1,
	L1 = 2,
	L2 = 3,
	L2SQUARED = 4,
	L5 = 5,
	L_INFINITY = 6,
}

opencv_type_enum! { crate::xfeatures2d::PCTSignatures_DistanceFunction }

// PointDistribution /usr/include/opencv2/xfeatures2d.hpp:462
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PCTSignatures_PointDistribution {
	UNIFORM = 0,
	REGULAR = 1,
	NORMAL = 2,
}

opencv_type_enum! { crate::xfeatures2d::PCTSignatures_PointDistribution }

// SimilarityFunction /usr/include/opencv2/xfeatures2d.hpp:479
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PCTSignatures_SimilarityFunction {
	MINUS = 0,
	GAUSSIAN = 1,
	HEURISTIC = 2,
}

opencv_type_enum! { crate::xfeatures2d::PCTSignatures_SimilarityFunction }

// KeypointLayout /usr/include/opencv2/xfeatures2d/cuda.hpp:89
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SURF_CUDA_KeypointLayout {
	X_ROW = 0,
	Y_ROW = 1,
	LAPLACIAN_ROW = 2,
	OCTAVE_ROW = 3,
	SIZE_ROW = 4,
	ANGLE_ROW = 5,
	HESSIAN_ROW = 6,
	ROWS_COUNT = 7,
}

opencv_type_enum! { crate::xfeatures2d::SURF_CUDA_KeypointLayout }

// SurfDescriptorExtractor /usr/include/opencv2/xfeatures2d/nonfree.hpp:115
pub type SurfDescriptorExtractor = dyn crate::xfeatures2d::SURF;
// SurfFeatureDetector /usr/include/opencv2/xfeatures2d/nonfree.hpp:114
pub type SurfFeatureDetector = dyn crate::xfeatures2d::SURF;
/// ## C++ default parameters
/// * nonmax_suppression: true
/// * typ: FastFeatureDetector::TYPE_9_16
// FASTForPointSet(cv::InputArray, std::vector<KeyPoint> &, int, bool, cv::FastFeatureDetector::DetectorType) /usr/include/opencv2/xfeatures2d.hpp:1043
#[inline]
pub fn fast_for_point_set(image: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, threshold: i32, nonmax_suppression: bool, typ: crate::features2d::FastFeatureDetector_DetectorType) -> Result<()> {
	input_array_arg!(image);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_xfeatures2d_FASTForPointSet_const__InputArrayR_vector_KeyPoint_R_int_bool_DetectorType(image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), threshold, nonmax_suppression, typ, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * with_rotation: false
/// * with_scale: false
/// * threshold_factor: 6.0
// matchGMS(const cv::Size &, const cv::Size &, const std::vector<KeyPoint> &, const std::vector<KeyPoint> &, const std::vector<DMatch> &, std::vector<DMatch> &, const bool, const bool, const double) /usr/include/opencv2/xfeatures2d.hpp:1068
#[inline]
pub fn match_gms(size1: core::Size, size2: core::Size, keypoints1: &core::Vector<core::KeyPoint>, keypoints2: &core::Vector<core::KeyPoint>, matches1to2: &core::Vector<core::DMatch>, matches_gms: &mut core::Vector<core::DMatch>, with_rotation: bool, with_scale: bool, threshold_factor: f64) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_xfeatures2d_matchGMS_const_SizeR_const_SizeR_const_vector_KeyPoint_R_const_vector_KeyPoint_R_const_vector_DMatch_R_vector_DMatch_R_const_bool_const_bool_const_double(&size1, &size2, keypoints1.as_raw_VectorOfKeyPoint(), keypoints2.as_raw_VectorOfKeyPoint(), matches1to2.as_raw_VectorOfDMatch(), matches_gms.as_raw_mut_VectorOfDMatch(), with_rotation, with_scale, threshold_factor, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// matchLOGOS(const std::vector<KeyPoint> &, const std::vector<KeyPoint> &, const std::vector<int> &, const std::vector<int> &, std::vector<DMatch> &) /usr/include/opencv2/xfeatures2d.hpp:1083
#[inline]
pub fn match_logos(keypoints1: &core::Vector<core::KeyPoint>, keypoints2: &core::Vector<core::KeyPoint>, nn1: &core::Vector<i32>, nn2: &core::Vector<i32>, matches1to2: &mut core::Vector<core::DMatch>) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_xfeatures2d_matchLOGOS_const_vector_KeyPoint_R_const_vector_KeyPoint_R_const_vector_int_R_const_vector_int_R_vector_DMatch_R(keypoints1.as_raw_VectorOfKeyPoint(), keypoints2.as_raw_VectorOfKeyPoint(), nn1.as_raw_VectorOfi32(), nn2.as_raw_VectorOfi32(), matches1to2.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// SURF_CUDA /usr/include/opencv2/xfeatures2d/cuda.hpp:86
pub trait SURF_CUDATraitConst {
	fn as_raw_SURF_CUDA(&self) -> *const c_void;

	// hessianThreshold /usr/include/opencv2/xfeatures2d/cuda.hpp:182
	#[inline]
	fn hessian_threshold(&self) -> f64 {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_getPropHessianThreshold_const(self.as_raw_SURF_CUDA()) };
		ret
	}
	
	// nOctaves /usr/include/opencv2/xfeatures2d/cuda.hpp:183
	#[inline]
	fn n_octaves(&self) -> i32 {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_getPropNOctaves_const(self.as_raw_SURF_CUDA()) };
		ret
	}
	
	// nOctaveLayers /usr/include/opencv2/xfeatures2d/cuda.hpp:184
	#[inline]
	fn n_octave_layers(&self) -> i32 {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_getPropNOctaveLayers_const(self.as_raw_SURF_CUDA()) };
		ret
	}
	
	// extended /usr/include/opencv2/xfeatures2d/cuda.hpp:185
	#[inline]
	fn extended(&self) -> bool {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_getPropExtended_const(self.as_raw_SURF_CUDA()) };
		ret
	}
	
	// upright /usr/include/opencv2/xfeatures2d/cuda.hpp:186
	#[inline]
	fn upright(&self) -> bool {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_getPropUpright_const(self.as_raw_SURF_CUDA()) };
		ret
	}
	
	// keypointsRatio /usr/include/opencv2/xfeatures2d/cuda.hpp:189
	#[inline]
	fn keypoints_ratio(&self) -> f32 {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_getPropKeypointsRatio_const(self.as_raw_SURF_CUDA()) };
		ret
	}
	
	// sum /usr/include/opencv2/xfeatures2d/cuda.hpp:191
	#[inline]
	fn sum(&self) -> core::GpuMat {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_getPropSum_const(self.as_raw_SURF_CUDA()) };
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		ret
	}
	
	// mask1 /usr/include/opencv2/xfeatures2d/cuda.hpp:191
	#[inline]
	fn mask1(&self) -> core::GpuMat {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_getPropMask1_const(self.as_raw_SURF_CUDA()) };
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		ret
	}
	
	// maskSum /usr/include/opencv2/xfeatures2d/cuda.hpp:191
	#[inline]
	fn mask_sum(&self) -> core::GpuMat {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_getPropMaskSum_const(self.as_raw_SURF_CUDA()) };
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		ret
	}
	
	// det /usr/include/opencv2/xfeatures2d/cuda.hpp:193
	#[inline]
	fn det(&self) -> core::GpuMat {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_getPropDet_const(self.as_raw_SURF_CUDA()) };
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		ret
	}
	
	// trace /usr/include/opencv2/xfeatures2d/cuda.hpp:193
	#[inline]
	fn trace(&self) -> core::GpuMat {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_getPropTrace_const(self.as_raw_SURF_CUDA()) };
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		ret
	}
	
	// maxPosBuffer /usr/include/opencv2/xfeatures2d/cuda.hpp:195
	#[inline]
	fn max_pos_buffer(&self) -> core::GpuMat {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_getPropMaxPosBuffer_const(self.as_raw_SURF_CUDA()) };
		let ret = unsafe { core::GpuMat::opencv_from_extern(ret) };
		ret
	}
	
	// descriptorSize() /usr/include/opencv2/xfeatures2d/cuda.hpp:121
	#[inline]
	fn descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_descriptorSize_const(self.as_raw_SURF_CUDA(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// defaultNorm() /usr/include/opencv2/xfeatures2d/cuda.hpp:123
	#[inline]
	fn default_norm(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_defaultNorm_const(self.as_raw_SURF_CUDA(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait SURF_CUDATrait: crate::xfeatures2d::SURF_CUDATraitConst {
	fn as_raw_mut_SURF_CUDA(&mut self) -> *mut c_void;

	// hessianThreshold /usr/include/opencv2/xfeatures2d/cuda.hpp:182
	#[inline]
	fn set_hessian_threshold(&mut self, val: f64) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_setPropHessianThreshold_double(self.as_raw_mut_SURF_CUDA(), val) };
		ret
	}
	
	// nOctaves /usr/include/opencv2/xfeatures2d/cuda.hpp:183
	#[inline]
	fn set_n_octaves(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_setPropNOctaves_int(self.as_raw_mut_SURF_CUDA(), val) };
		ret
	}
	
	// nOctaveLayers /usr/include/opencv2/xfeatures2d/cuda.hpp:184
	#[inline]
	fn set_n_octave_layers(&mut self, val: i32) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_setPropNOctaveLayers_int(self.as_raw_mut_SURF_CUDA(), val) };
		ret
	}
	
	// extended /usr/include/opencv2/xfeatures2d/cuda.hpp:185
	#[inline]
	fn set_extended(&mut self, val: bool) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_setPropExtended_bool(self.as_raw_mut_SURF_CUDA(), val) };
		ret
	}
	
	// upright /usr/include/opencv2/xfeatures2d/cuda.hpp:186
	#[inline]
	fn set_upright(&mut self, val: bool) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_setPropUpright_bool(self.as_raw_mut_SURF_CUDA(), val) };
		ret
	}
	
	// keypointsRatio /usr/include/opencv2/xfeatures2d/cuda.hpp:189
	#[inline]
	fn set_keypoints_ratio(&mut self, val: f32) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_setPropKeypointsRatio_float(self.as_raw_mut_SURF_CUDA(), val) };
		ret
	}
	
	// sum /usr/include/opencv2/xfeatures2d/cuda.hpp:191
	#[inline]
	fn set_sum(&mut self, mut val: core::GpuMat) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_setPropSum_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_mut_GpuMat()) };
		ret
	}
	
	// mask1 /usr/include/opencv2/xfeatures2d/cuda.hpp:191
	#[inline]
	fn set_mask1(&mut self, mut val: core::GpuMat) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_setPropMask1_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_mut_GpuMat()) };
		ret
	}
	
	// maskSum /usr/include/opencv2/xfeatures2d/cuda.hpp:191
	#[inline]
	fn set_mask_sum(&mut self, mut val: core::GpuMat) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_setPropMaskSum_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_mut_GpuMat()) };
		ret
	}
	
	// det /usr/include/opencv2/xfeatures2d/cuda.hpp:193
	#[inline]
	fn set_det(&mut self, mut val: core::GpuMat) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_setPropDet_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_mut_GpuMat()) };
		ret
	}
	
	// trace /usr/include/opencv2/xfeatures2d/cuda.hpp:193
	#[inline]
	fn set_trace(&mut self, mut val: core::GpuMat) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_setPropTrace_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_mut_GpuMat()) };
		ret
	}
	
	// maxPosBuffer /usr/include/opencv2/xfeatures2d/cuda.hpp:195
	#[inline]
	fn set_max_pos_buffer(&mut self, mut val: core::GpuMat) {
		let ret = unsafe { sys::cv_cuda_SURF_CUDA_setPropMaxPosBuffer_GpuMat(self.as_raw_mut_SURF_CUDA(), val.as_raw_mut_GpuMat()) };
		ret
	}
	
	// uploadKeypoints(const std::vector<KeyPoint> &, cv::cuda::GpuMat &) /usr/include/opencv2/xfeatures2d/cuda.hpp:126
	#[inline]
	fn upload_keypoints(&mut self, keypoints: &core::Vector<core::KeyPoint>, keypoints_gpu: &mut core::GpuMat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_uploadKeypoints_const_vector_KeyPoint_R_GpuMatR(self.as_raw_mut_SURF_CUDA(), keypoints.as_raw_VectorOfKeyPoint(), keypoints_gpu.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// downloadKeypoints(const cv::cuda::GpuMat &, std::vector<KeyPoint> &) /usr/include/opencv2/xfeatures2d/cuda.hpp:128
	#[inline]
	fn download_keypoints(&mut self, keypoints_gpu: &core::GpuMat, keypoints: &mut core::Vector<core::KeyPoint>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_downloadKeypoints_const_GpuMatR_vector_KeyPoint_R(self.as_raw_mut_SURF_CUDA(), keypoints_gpu.as_raw_GpuMat(), keypoints.as_raw_mut_VectorOfKeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// downloadDescriptors(const cv::cuda::GpuMat &, std::vector<float> &) /usr/include/opencv2/xfeatures2d/cuda.hpp:131
	#[inline]
	fn download_descriptors(&mut self, descriptors_gpu: &core::GpuMat, descriptors: &mut core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_downloadDescriptors_const_GpuMatR_vector_float_R(self.as_raw_mut_SURF_CUDA(), descriptors_gpu.as_raw_GpuMat(), descriptors.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// detect(const cv::cuda::GpuMat &, const cv::cuda::GpuMat &, cv::cuda::GpuMat &) /usr/include/opencv2/xfeatures2d/cuda.hpp:155
	#[inline]
	fn detect(&mut self, img: &core::GpuMat, mask: &core::GpuMat, keypoints: &mut core::GpuMat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_detect_const_GpuMatR_const_GpuMatR_GpuMatR(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_GpuMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * use_provided_keypoints: false
	// detectWithDescriptors(const cv::cuda::GpuMat &, const cv::cuda::GpuMat &, cv::cuda::GpuMat &, cv::cuda::GpuMat &, bool) /usr/include/opencv2/xfeatures2d/cuda.hpp:171
	#[inline]
	fn detect_with_descriptors(&mut self, img: &core::GpuMat, mask: &core::GpuMat, keypoints: &mut core::GpuMat, descriptors: &mut core::GpuMat, use_provided_keypoints: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_detectWithDescriptors_const_GpuMatR_const_GpuMatR_GpuMatR_GpuMatR_bool(self.as_raw_mut_SURF_CUDA(), img.as_raw_GpuMat(), mask.as_raw_GpuMat(), keypoints.as_raw_mut_GpuMat(), descriptors.as_raw_mut_GpuMat(), use_provided_keypoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// releaseMemory() /usr/include/opencv2/xfeatures2d/cuda.hpp:179
	#[inline]
	fn release_memory(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_releaseMemory(self.as_raw_mut_SURF_CUDA(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// SURF_CUDA /usr/include/opencv2/xfeatures2d/cuda.hpp:86
pub struct SURF_CUDA {
	ptr: *mut c_void
}

opencv_type_boxed! { SURF_CUDA }

impl Drop for SURF_CUDA {
	fn drop(&mut self) {
		extern "C" { fn cv_SURF_CUDA_delete(instance: *mut c_void); }
		unsafe { cv_SURF_CUDA_delete(self.as_raw_mut_SURF_CUDA()) };
	}
}

unsafe impl Send for SURF_CUDA {}

impl crate::xfeatures2d::SURF_CUDATraitConst for SURF_CUDA {
	#[inline] fn as_raw_SURF_CUDA(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::SURF_CUDATrait for SURF_CUDA {
	#[inline] fn as_raw_mut_SURF_CUDA(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SURF_CUDA {
	// SURF_CUDA() /usr/include/opencv2/xfeatures2d/cuda.hpp:102
	#[inline]
	pub fn default() -> Result<crate::xfeatures2d::SURF_CUDA> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_SURF_CUDA(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::SURF_CUDA::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * _n_octaves: 4
	/// * _n_octave_layers: 2
	/// * _extended: false
	/// * _keypoints_ratio: 0.01f
	/// * _upright: false
	// SURF_CUDA(double, int, int, bool, float, bool) /usr/include/opencv2/xfeatures2d/cuda.hpp:104
	#[inline]
	pub fn new(_hessian_threshold: f64, _n_octaves: i32, _n_octave_layers: i32, _extended: bool, _keypoints_ratio: f32, _upright: bool) -> Result<crate::xfeatures2d::SURF_CUDA> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_SURF_CUDA_double_int_int_bool_float_bool(_hessian_threshold, _n_octaves, _n_octave_layers, _extended, _keypoints_ratio, _upright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::SURF_CUDA::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * _n_octaves: 4
	/// * _n_octave_layers: 2
	/// * _extended: false
	/// * _keypoints_ratio: 0.01f
	/// * _upright: false
	// create(double, int, int, bool, float, bool) /usr/include/opencv2/xfeatures2d/cuda.hpp:117
	#[inline]
	pub fn create(_hessian_threshold: f64, _n_octaves: i32, _n_octave_layers: i32, _extended: bool, _keypoints_ratio: f32, _upright: bool) -> Result<core::Ptr<crate::xfeatures2d::SURF_CUDA>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_cuda_SURF_CUDA_create_double_int_int_bool_float_bool(_hessian_threshold, _n_octaves, _n_octave_layers, _extended, _keypoints_ratio, _upright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::SURF_CUDA>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// AffineFeature2D /usr/include/opencv2/xfeatures2d.hpp:949
pub trait AffineFeature2DConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_AffineFeature2D(&self) -> *const c_void;

}

pub trait AffineFeature2D: crate::features2d::Feature2DTrait + crate::xfeatures2d::AffineFeature2DConst {
	fn as_raw_mut_AffineFeature2D(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * mask: noArray()
	// detect(cv::InputArray, std::vector<Elliptic_KeyPoint> &, cv::InputArray) /usr/include/opencv2/xfeatures2d.hpp:975
	#[inline]
	fn detect(&mut self, image: &dyn core::ToInputArray, keypoints: &mut core::Vector<crate::xfeatures2d::Elliptic_KeyPoint>, mask: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_detect_const__InputArrayR_vector_Elliptic_KeyPoint_R_const__InputArrayR(self.as_raw_mut_AffineFeature2D(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfElliptic_KeyPoint(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * use_provided_keypoints: false
	// detectAndCompute(cv::InputArray, cv::InputArray, std::vector<Elliptic_KeyPoint> &, cv::OutputArray, bool) /usr/include/opencv2/xfeatures2d.hpp:985
	#[inline]
	fn detect_and_compute(&mut self, image: &dyn core::ToInputArray, mask: &dyn core::ToInputArray, keypoints: &mut core::Vector<crate::xfeatures2d::Elliptic_KeyPoint>, descriptors: &mut dyn core::ToOutputArray, use_provided_keypoints: bool) -> Result<()> {
		input_array_arg!(image);
		input_array_arg!(mask);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_detectAndCompute_const__InputArrayR_const__InputArrayR_vector_Elliptic_KeyPoint_R_const__OutputArrayR_bool(self.as_raw_mut_AffineFeature2D(), image.as_raw__InputArray(), mask.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfElliptic_KeyPoint(), descriptors.as_raw__OutputArray(), use_provided_keypoints, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn AffineFeature2D + '_ {
	// create(Ptr<cv::FeatureDetector>, Ptr<cv::DescriptorExtractor>) /usr/include/opencv2/xfeatures2d.hpp:956
	#[inline]
	pub fn create(mut keypoint_detector: core::Ptr<crate::features2d::Feature2D>, mut descriptor_extractor: core::Ptr<crate::features2d::Feature2D>) -> Result<core::Ptr<dyn crate::xfeatures2d::AffineFeature2D>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_create_Ptr_Feature2D__Ptr_Feature2D_(keypoint_detector.as_raw_mut_PtrOfFeature2D(), descriptor_extractor.as_raw_mut_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::xfeatures2d::AffineFeature2D>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// create(Ptr<cv::FeatureDetector>) /usr/include/opencv2/xfeatures2d.hpp:964
	#[inline]
	pub fn create_1(mut keypoint_detector: core::Ptr<crate::features2d::Feature2D>) -> Result<core::Ptr<dyn crate::xfeatures2d::AffineFeature2D>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_AffineFeature2D_create_Ptr_Feature2D_(keypoint_detector.as_raw_mut_PtrOfFeature2D(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::xfeatures2d::AffineFeature2D>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// BEBLID /usr/include/opencv2/xfeatures2d.hpp:204
pub trait BEBLIDTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_BEBLID(&self) -> *const c_void;

}

pub trait BEBLIDTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::BEBLIDTraitConst {
	fn as_raw_mut_BEBLID(&mut self) -> *mut c_void;

}

// BEBLID /usr/include/opencv2/xfeatures2d.hpp:204
pub struct BEBLID {
	ptr: *mut c_void
}

opencv_type_boxed! { BEBLID }

impl Drop for BEBLID {
	fn drop(&mut self) {
		extern "C" { fn cv_BEBLID_delete(instance: *mut c_void); }
		unsafe { cv_BEBLID_delete(self.as_raw_mut_BEBLID()) };
	}
}

unsafe impl Send for BEBLID {}

impl core::AlgorithmTraitConst for BEBLID {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BEBLID {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for BEBLID {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for BEBLID {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::BEBLIDTraitConst for BEBLID {
	#[inline] fn as_raw_BEBLID(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::BEBLIDTrait for BEBLID {
	#[inline] fn as_raw_mut_BEBLID(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BEBLID {
	/// ## C++ default parameters
	/// * n_bits: BEBLID::SIZE_512_BITS
	// create(float, int) /usr/include/opencv2/xfeatures2d.hpp:224
	#[inline]
	pub fn create(scale_factor: f32, n_bits: i32) -> Result<core::Ptr<crate::xfeatures2d::BEBLID>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BEBLID_create_float_int(scale_factor, n_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BEBLID>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { BEBLID, core::Algorithm, cv_BEBLID_to_Algorithm }

boxed_cast_base! { BEBLID, crate::features2d::Feature2D, cv_BEBLID_to_Feature2D }

// BoostDesc /usr/include/opencv2/xfeatures2d.hpp:409
pub trait BoostDescConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_BoostDesc(&self) -> *const c_void;

	// getUseScaleOrientation() /usr/include/opencv2/xfeatures2d.hpp:423
	#[inline]
	fn get_use_scale_orientation(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BoostDesc_getUseScaleOrientation_const(self.as_raw_BoostDesc(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getScaleFactor() /usr/include/opencv2/xfeatures2d.hpp:426
	#[inline]
	fn get_scale_factor(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BoostDesc_getScaleFactor_const(self.as_raw_BoostDesc(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BoostDesc: crate::features2d::Feature2DTrait + crate::xfeatures2d::BoostDescConst {
	fn as_raw_mut_BoostDesc(&mut self) -> *mut c_void;

	// setUseScaleOrientation(const bool) /usr/include/opencv2/xfeatures2d.hpp:422
	#[inline]
	fn set_use_scale_orientation(&mut self, use_scale_orientation: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BoostDesc_setUseScaleOrientation_const_bool(self.as_raw_mut_BoostDesc(), use_scale_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setScaleFactor(const float) /usr/include/opencv2/xfeatures2d.hpp:425
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BoostDesc_setScaleFactor_const_float(self.as_raw_mut_BoostDesc(), scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn BoostDesc + '_ {
	/// ## C++ default parameters
	/// * desc: BoostDesc::BINBOOST_256
	/// * use_scale_orientation: true
	/// * scale_factor: 6.25f
	// create(int, bool, float) /usr/include/opencv2/xfeatures2d.hpp:419
	#[inline]
	pub fn create(desc: i32, use_scale_orientation: bool, scale_factor: f32) -> Result<core::Ptr<dyn crate::xfeatures2d::BoostDesc>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BoostDesc_create_int_bool_float(desc, use_scale_orientation, scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::xfeatures2d::BoostDesc>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// BriefDescriptorExtractor /usr/include/opencv2/xfeatures2d.hpp:130
pub trait BriefDescriptorExtractorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_BriefDescriptorExtractor(&self) -> *const c_void;

}

pub trait BriefDescriptorExtractorTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::BriefDescriptorExtractorTraitConst {
	fn as_raw_mut_BriefDescriptorExtractor(&mut self) -> *mut c_void;

}

// BriefDescriptorExtractor /usr/include/opencv2/xfeatures2d.hpp:130
pub struct BriefDescriptorExtractor {
	ptr: *mut c_void
}

opencv_type_boxed! { BriefDescriptorExtractor }

impl Drop for BriefDescriptorExtractor {
	fn drop(&mut self) {
		extern "C" { fn cv_BriefDescriptorExtractor_delete(instance: *mut c_void); }
		unsafe { cv_BriefDescriptorExtractor_delete(self.as_raw_mut_BriefDescriptorExtractor()) };
	}
}

unsafe impl Send for BriefDescriptorExtractor {}

impl core::AlgorithmTraitConst for BriefDescriptorExtractor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BriefDescriptorExtractor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for BriefDescriptorExtractor {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for BriefDescriptorExtractor {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::BriefDescriptorExtractorTraitConst for BriefDescriptorExtractor {
	#[inline] fn as_raw_BriefDescriptorExtractor(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::BriefDescriptorExtractorTrait for BriefDescriptorExtractor {
	#[inline] fn as_raw_mut_BriefDescriptorExtractor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BriefDescriptorExtractor {
	/// ## C++ default parameters
	/// * bytes: 32
	/// * use_orientation: false
	// create(int, bool) /usr/include/opencv2/xfeatures2d.hpp:133
	#[inline]
	pub fn create(bytes: i32, use_orientation: bool) -> Result<core::Ptr<crate::xfeatures2d::BriefDescriptorExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_BriefDescriptorExtractor_create_int_bool(bytes, use_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::BriefDescriptorExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { BriefDescriptorExtractor, core::Algorithm, cv_BriefDescriptorExtractor_to_Algorithm }

boxed_cast_base! { BriefDescriptorExtractor, crate::features2d::Feature2D, cv_BriefDescriptorExtractor_to_Feature2D }

// DAISY /usr/include/opencv2/xfeatures2d.hpp:243
pub trait DAISYConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_DAISY(&self) -> *const c_void;

	// GetDescriptor(double, double, int, float *) /usr/include/opencv2/xfeatures2d.hpp:284
	#[inline]
	fn get_descriptor(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX(self.as_raw_DAISY(), y, x, orientation, descriptor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// GetDescriptor(double, double, int, float *, double *) /usr/include/opencv2/xfeatures2d.hpp:293
	#[inline]
	fn get_descriptor_1(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32, h: &mut f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_GetDescriptor_const_double_double_int_floatX_doubleX(self.as_raw_DAISY(), y, x, orientation, descriptor, h, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// GetUnnormalizedDescriptor(double, double, int, float *) /usr/include/opencv2/xfeatures2d.hpp:301
	#[inline]
	fn get_unnormalized_descriptor(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX(self.as_raw_DAISY(), y, x, orientation, descriptor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// GetUnnormalizedDescriptor(double, double, int, float *, double *) /usr/include/opencv2/xfeatures2d.hpp:310
	#[inline]
	fn get_unnormalized_descriptor_1(&self, y: f64, x: f64, orientation: i32, descriptor: &mut f32, h: &mut f64) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_GetUnnormalizedDescriptor_const_double_double_int_floatX_doubleX(self.as_raw_DAISY(), y, x, orientation, descriptor, h, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait DAISY: crate::features2d::Feature2DTrait + crate::xfeatures2d::DAISYConst {
	fn as_raw_mut_DAISY(&mut self) -> *mut c_void;

	// compute(cv::InputArray, std::vector<KeyPoint> &, cv::OutputArray) /usr/include/opencv2/xfeatures2d.hpp:259
	#[inline]
	fn compute(&mut self, image: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::KeyPoint>, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_compute_const__InputArrayR_vector_KeyPoint_R_const__OutputArrayR(self.as_raw_mut_DAISY(), image.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// compute(cv::InputArrayOfArrays, std::vector<std::vector<KeyPoint>> &, cv::OutputArrayOfArrays) /usr/include/opencv2/xfeatures2d.hpp:261
	#[inline]
	fn compute_1(&mut self, images: &dyn core::ToInputArray, keypoints: &mut core::Vector<core::Vector<core::KeyPoint>>, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(images);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_compute_const__InputArrayR_vector_vector_KeyPoint__R_const__OutputArrayR(self.as_raw_mut_DAISY(), images.as_raw__InputArray(), keypoints.as_raw_mut_VectorOfVectorOfKeyPoint(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// compute(cv::InputArray, cv::Rect, cv::OutputArray) /usr/include/opencv2/xfeatures2d.hpp:270
	#[inline]
	fn compute_2(&mut self, image: &dyn core::ToInputArray, roi: core::Rect, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_compute_const__InputArrayR_Rect_const__OutputArrayR(self.as_raw_mut_DAISY(), image.as_raw__InputArray(), roi.opencv_as_extern(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// compute(cv::InputArray, cv::OutputArray) /usr/include/opencv2/xfeatures2d.hpp:276
	#[inline]
	fn compute_3(&mut self, image: &dyn core::ToInputArray, descriptors: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(descriptors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_compute_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_DAISY(), image.as_raw__InputArray(), descriptors.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn DAISY + '_ {
	/// ## C++ default parameters
	/// * radius: 15
	/// * q_radius: 3
	/// * q_theta: 8
	/// * q_hist: 8
	/// * norm: DAISY::NRM_NONE
	/// * h: noArray()
	/// * interpolation: true
	/// * use_orientation: false
	// create(float, int, int, int, DAISY::NormalizationType, cv::InputArray, bool, bool) /usr/include/opencv2/xfeatures2d.hpp:250
	#[inline]
	pub fn create(radius: f32, q_radius: i32, q_theta: i32, q_hist: i32, norm: crate::xfeatures2d::DAISY_NormalizationType, h: &dyn core::ToInputArray, interpolation: bool, use_orientation: bool) -> Result<core::Ptr<dyn crate::xfeatures2d::DAISY>> {
		input_array_arg!(h);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_DAISY_create_float_int_int_int_NormalizationType_const__InputArrayR_bool_bool(radius, q_radius, q_theta, q_hist, norm, h.as_raw__InputArray(), interpolation, use_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::xfeatures2d::DAISY>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// Elliptic_KeyPoint /usr/include/opencv2/xfeatures2d.hpp:905
pub trait Elliptic_KeyPointTraitConst {
	fn as_raw_Elliptic_KeyPoint(&self) -> *const c_void;

	// axes /usr/include/opencv2/xfeatures2d.hpp:908
	#[inline]
	fn axes(&self) -> core::Size_<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_getPropAxes_const(self.as_raw_Elliptic_KeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	// si /usr/include/opencv2/xfeatures2d.hpp:909
	#[inline]
	fn si(&self) -> f32 {
		let ret = unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_getPropSi_const(self.as_raw_Elliptic_KeyPoint()) };
		ret
	}
	
	// transf /usr/include/opencv2/xfeatures2d.hpp:910
	#[inline]
	fn transf(&self) -> core::Matx23f {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_getPropTransf_const(self.as_raw_Elliptic_KeyPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
}

pub trait Elliptic_KeyPointTrait: crate::xfeatures2d::Elliptic_KeyPointTraitConst {
	fn as_raw_mut_Elliptic_KeyPoint(&mut self) -> *mut c_void;

	// axes /usr/include/opencv2/xfeatures2d.hpp:908
	#[inline]
	fn set_axes(&mut self, val: core::Size_<f32>) {
		let ret = unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_setPropAxes_Size__float_(self.as_raw_mut_Elliptic_KeyPoint(), val.opencv_as_extern()) };
		ret
	}
	
	// si /usr/include/opencv2/xfeatures2d.hpp:909
	#[inline]
	fn set_si(&mut self, val: f32) {
		let ret = unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_setPropSi_float(self.as_raw_mut_Elliptic_KeyPoint(), val) };
		ret
	}
	
	// transf /usr/include/opencv2/xfeatures2d.hpp:910
	#[inline]
	fn set_transf(&mut self, val: core::Matx23f) {
		let ret = unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_setPropTransf_Matx23f(self.as_raw_mut_Elliptic_KeyPoint(), val.opencv_as_extern()) };
		ret
	}
	
}

// Elliptic_KeyPoint /usr/include/opencv2/xfeatures2d.hpp:905
pub struct Elliptic_KeyPoint {
	ptr: *mut c_void
}

opencv_type_boxed! { Elliptic_KeyPoint }

impl Drop for Elliptic_KeyPoint {
	fn drop(&mut self) {
		extern "C" { fn cv_Elliptic_KeyPoint_delete(instance: *mut c_void); }
		unsafe { cv_Elliptic_KeyPoint_delete(self.as_raw_mut_Elliptic_KeyPoint()) };
	}
}

unsafe impl Send for Elliptic_KeyPoint {}

impl crate::xfeatures2d::Elliptic_KeyPointTraitConst for Elliptic_KeyPoint {
	#[inline] fn as_raw_Elliptic_KeyPoint(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::Elliptic_KeyPointTrait for Elliptic_KeyPoint {
	#[inline] fn as_raw_mut_Elliptic_KeyPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Elliptic_KeyPoint {
	// Elliptic_KeyPoint() /usr/include/opencv2/xfeatures2d.hpp:911
	#[inline]
	pub fn default() -> Result<crate::xfeatures2d::Elliptic_KeyPoint> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::Elliptic_KeyPoint::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// Elliptic_KeyPoint(cv::Point2f, float, cv::Size, float, float) /usr/include/opencv2/xfeatures2d.hpp:912
	#[inline]
	pub fn new(pt: core::Point2f, angle: f32, axes: core::Size, size: f32, si: f32) -> Result<crate::xfeatures2d::Elliptic_KeyPoint> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_Elliptic_KeyPoint_Elliptic_KeyPoint_Point2f_float_Size_float_float(pt.opencv_as_extern(), angle, axes.opencv_as_extern(), size, si, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::xfeatures2d::Elliptic_KeyPoint::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// FREAK /usr/include/opencv2/xfeatures2d.hpp:85
pub trait FREAKTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_FREAK(&self) -> *const c_void;

}

pub trait FREAKTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::FREAKTraitConst {
	fn as_raw_mut_FREAK(&mut self) -> *mut c_void;

}

// FREAK /usr/include/opencv2/xfeatures2d.hpp:85
pub struct FREAK {
	ptr: *mut c_void
}

opencv_type_boxed! { FREAK }

impl Drop for FREAK {
	fn drop(&mut self) {
		extern "C" { fn cv_FREAK_delete(instance: *mut c_void); }
		unsafe { cv_FREAK_delete(self.as_raw_mut_FREAK()) };
	}
}

unsafe impl Send for FREAK {}

impl core::AlgorithmTraitConst for FREAK {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for FREAK {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for FREAK {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for FREAK {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::FREAKTraitConst for FREAK {
	#[inline] fn as_raw_FREAK(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::FREAKTrait for FREAK {
	#[inline] fn as_raw_mut_FREAK(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl FREAK {
	// NB_SCALES /usr/include/opencv2/xfeatures2d.hpp:89
	pub const NB_SCALES: i32 = 64;
	// NB_PAIRS /usr/include/opencv2/xfeatures2d.hpp:90
	pub const NB_PAIRS: i32 = 512;
	// NB_ORIENPAIRS /usr/include/opencv2/xfeatures2d.hpp:91
	pub const NB_ORIENPAIRS: i32 = 45;
	/// ## C++ default parameters
	/// * orientation_normalized: true
	/// * scale_normalized: true
	/// * pattern_scale: 22.0f
	/// * n_octaves: 4
	/// * selected_pairs: std::vector<int>()
	// create(bool, bool, float, int, const std::vector<int> &) /usr/include/opencv2/xfeatures2d.hpp:100
	#[inline]
	pub fn create(orientation_normalized: bool, scale_normalized: bool, pattern_scale: f32, n_octaves: i32, selected_pairs: &core::Vector<i32>) -> Result<core::Ptr<crate::xfeatures2d::FREAK>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_FREAK_create_bool_bool_float_int_const_vector_int_R(orientation_normalized, scale_normalized, pattern_scale, n_octaves, selected_pairs.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::FREAK>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { FREAK, core::Algorithm, cv_FREAK_to_Algorithm }

boxed_cast_base! { FREAK, crate::features2d::Feature2D, cv_FREAK_to_Feature2D }

// HarrisLaplaceFeatureDetector /usr/include/opencv2/xfeatures2d.hpp:919
pub trait HarrisLaplaceFeatureDetectorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_HarrisLaplaceFeatureDetector(&self) -> *const c_void;

}

pub trait HarrisLaplaceFeatureDetectorTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::HarrisLaplaceFeatureDetectorTraitConst {
	fn as_raw_mut_HarrisLaplaceFeatureDetector(&mut self) -> *mut c_void;

}

// HarrisLaplaceFeatureDetector /usr/include/opencv2/xfeatures2d.hpp:919
pub struct HarrisLaplaceFeatureDetector {
	ptr: *mut c_void
}

opencv_type_boxed! { HarrisLaplaceFeatureDetector }

impl Drop for HarrisLaplaceFeatureDetector {
	fn drop(&mut self) {
		extern "C" { fn cv_HarrisLaplaceFeatureDetector_delete(instance: *mut c_void); }
		unsafe { cv_HarrisLaplaceFeatureDetector_delete(self.as_raw_mut_HarrisLaplaceFeatureDetector()) };
	}
}

unsafe impl Send for HarrisLaplaceFeatureDetector {}

impl core::AlgorithmTraitConst for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTraitConst for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_HarrisLaplaceFeatureDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::HarrisLaplaceFeatureDetectorTrait for HarrisLaplaceFeatureDetector {
	#[inline] fn as_raw_mut_HarrisLaplaceFeatureDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl HarrisLaplaceFeatureDetector {
	/// ## C++ default parameters
	/// * num_octaves: 6
	/// * corn_thresh: 0.01f
	/// * dog_thresh: 0.01f
	/// * max_corners: 5000
	/// * num_layers: 4
	// create(int, float, float, int, int) /usr/include/opencv2/xfeatures2d.hpp:931
	#[inline]
	pub fn create(num_octaves: i32, corn_thresh: f32, dog_thresh: f32, max_corners: i32, num_layers: i32) -> Result<core::Ptr<crate::xfeatures2d::HarrisLaplaceFeatureDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_HarrisLaplaceFeatureDetector_create_int_float_float_int_int(num_octaves, corn_thresh, dog_thresh, max_corners, num_layers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::HarrisLaplaceFeatureDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { HarrisLaplaceFeatureDetector, core::Algorithm, cv_HarrisLaplaceFeatureDetector_to_Algorithm }

boxed_cast_base! { HarrisLaplaceFeatureDetector, crate::features2d::Feature2D, cv_HarrisLaplaceFeatureDetector_to_Feature2D }

// LATCH /usr/include/opencv2/xfeatures2d.hpp:176
pub trait LATCHTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_LATCH(&self) -> *const c_void;

}

pub trait LATCHTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::LATCHTraitConst {
	fn as_raw_mut_LATCH(&mut self) -> *mut c_void;

}

// LATCH /usr/include/opencv2/xfeatures2d.hpp:176
pub struct LATCH {
	ptr: *mut c_void
}

opencv_type_boxed! { LATCH }

impl Drop for LATCH {
	fn drop(&mut self) {
		extern "C" { fn cv_LATCH_delete(instance: *mut c_void); }
		unsafe { cv_LATCH_delete(self.as_raw_mut_LATCH()) };
	}
}

unsafe impl Send for LATCH {}

impl core::AlgorithmTraitConst for LATCH {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LATCH {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for LATCH {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for LATCH {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::LATCHTraitConst for LATCH {
	#[inline] fn as_raw_LATCH(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::LATCHTrait for LATCH {
	#[inline] fn as_raw_mut_LATCH(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LATCH {
	/// ## C++ default parameters
	/// * bytes: 32
	/// * rotation_invariance: true
	/// * half_ssd_size: 3
	/// * sigma: 2.0
	// create(int, bool, int, double) /usr/include/opencv2/xfeatures2d.hpp:179
	#[inline]
	pub fn create(bytes: i32, rotation_invariance: bool, half_ssd_size: i32, sigma: f64) -> Result<core::Ptr<crate::xfeatures2d::LATCH>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LATCH_create_int_bool_int_double(bytes, rotation_invariance, half_ssd_size, sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::LATCH>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { LATCH, core::Algorithm, cv_LATCH_to_Algorithm }

boxed_cast_base! { LATCH, crate::features2d::Feature2D, cv_LATCH_to_Feature2D }

// LUCID /usr/include/opencv2/xfeatures2d.hpp:143
pub trait LUCIDTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_LUCID(&self) -> *const c_void;

}

pub trait LUCIDTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::LUCIDTraitConst {
	fn as_raw_mut_LUCID(&mut self) -> *mut c_void;

}

// LUCID /usr/include/opencv2/xfeatures2d.hpp:143
pub struct LUCID {
	ptr: *mut c_void
}

opencv_type_boxed! { LUCID }

impl Drop for LUCID {
	fn drop(&mut self) {
		extern "C" { fn cv_LUCID_delete(instance: *mut c_void); }
		unsafe { cv_LUCID_delete(self.as_raw_mut_LUCID()) };
	}
}

unsafe impl Send for LUCID {}

impl core::AlgorithmTraitConst for LUCID {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LUCID {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for LUCID {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for LUCID {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::LUCIDTraitConst for LUCID {
	#[inline] fn as_raw_LUCID(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::LUCIDTrait for LUCID {
	#[inline] fn as_raw_mut_LUCID(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LUCID {
	/// ## C++ default parameters
	/// * lucid_kernel: 1
	/// * blur_kernel: 2
	// create(const int, const int) /usr/include/opencv2/xfeatures2d.hpp:150
	#[inline]
	pub fn create(lucid_kernel: i32, blur_kernel: i32) -> Result<core::Ptr<crate::xfeatures2d::LUCID>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_LUCID_create_const_int_const_int(lucid_kernel, blur_kernel, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::LUCID>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { LUCID, core::Algorithm, cv_LUCID_to_Algorithm }

boxed_cast_base! { LUCID, crate::features2d::Feature2D, cv_LUCID_to_Feature2D }

// MSDDetector /usr/include/opencv2/xfeatures2d.hpp:327
pub trait MSDDetectorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_MSDDetector(&self) -> *const c_void;

}

pub trait MSDDetectorTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::MSDDetectorTraitConst {
	fn as_raw_mut_MSDDetector(&mut self) -> *mut c_void;

}

// MSDDetector /usr/include/opencv2/xfeatures2d.hpp:327
pub struct MSDDetector {
	ptr: *mut c_void
}

opencv_type_boxed! { MSDDetector }

impl Drop for MSDDetector {
	fn drop(&mut self) {
		extern "C" { fn cv_MSDDetector_delete(instance: *mut c_void); }
		unsafe { cv_MSDDetector_delete(self.as_raw_mut_MSDDetector()) };
	}
}

unsafe impl Send for MSDDetector {}

impl core::AlgorithmTraitConst for MSDDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MSDDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for MSDDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for MSDDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::MSDDetectorTraitConst for MSDDetector {
	#[inline] fn as_raw_MSDDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::MSDDetectorTrait for MSDDetector {
	#[inline] fn as_raw_mut_MSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MSDDetector {
	/// ## C++ default parameters
	/// * m_patch_radius: 3
	/// * m_search_area_radius: 5
	/// * m_nms_radius: 5
	/// * m_nms_scale_radius: 0
	/// * m_th_saliency: 250.0f
	/// * m_k_nn: 4
	/// * m_scale_factor: 1.25f
	/// * m_n_scales: -1
	/// * m_compute_orientation: false
	// create(int, int, int, int, float, int, float, int, bool) /usr/include/opencv2/xfeatures2d.hpp:331
	#[inline]
	pub fn create(m_patch_radius: i32, m_search_area_radius: i32, m_nms_radius: i32, m_nms_scale_radius: i32, m_th_saliency: f32, m_k_nn: i32, m_scale_factor: f32, m_n_scales: i32, m_compute_orientation: bool) -> Result<core::Ptr<crate::xfeatures2d::MSDDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_MSDDetector_create_int_int_int_int_float_int_float_int_bool(m_patch_radius, m_search_area_radius, m_nms_radius, m_nms_scale_radius, m_th_saliency, m_k_nn, m_scale_factor, m_n_scales, m_compute_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::MSDDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { MSDDetector, core::Algorithm, cv_MSDDetector_to_Algorithm }

boxed_cast_base! { MSDDetector, crate::features2d::Feature2D, cv_MSDDetector_to_Feature2D }

// PCTSignatures /usr/include/opencv2/xfeatures2d.hpp:448
pub trait PCTSignaturesConst: core::AlgorithmTraitConst {
	fn as_raw_PCTSignatures(&self) -> *const c_void;

	// computeSignature(cv::InputArray, cv::OutputArray) /usr/include/opencv2/xfeatures2d.hpp:534
	#[inline]
	fn compute_signature(&self, image: &dyn core::ToInputArray, signature: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(signature);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_computeSignature_const_const__InputArrayR_const__OutputArrayR(self.as_raw_PCTSignatures(), image.as_raw__InputArray(), signature.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// computeSignatures(const std::vector<Mat> &, std::vector<Mat> &) /usr/include/opencv2/xfeatures2d.hpp:543
	#[inline]
	fn compute_signatures(&self, images: &core::Vector<core::Mat>, signatures: &mut core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_computeSignatures_const_const_vector_Mat_R_vector_Mat_R(self.as_raw_PCTSignatures(), images.as_raw_VectorOfMat(), signatures.as_raw_mut_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSampleCount() /usr/include/opencv2/xfeatures2d.hpp:585
	#[inline]
	fn get_sample_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getSampleCount_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getGrayscaleBits() /usr/include/opencv2/xfeatures2d.hpp:592
	#[inline]
	fn get_grayscale_bits(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getGrayscaleBits_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWindowRadius() /usr/include/opencv2/xfeatures2d.hpp:605
	#[inline]
	fn get_window_radius(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWindowRadius_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWeightX() /usr/include/opencv2/xfeatures2d.hpp:618
	#[inline]
	fn get_weight_x(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightX_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWeightY() /usr/include/opencv2/xfeatures2d.hpp:629
	#[inline]
	fn get_weight_y(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightY_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWeightL() /usr/include/opencv2/xfeatures2d.hpp:640
	#[inline]
	fn get_weight_l(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightL_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWeightA() /usr/include/opencv2/xfeatures2d.hpp:651
	#[inline]
	fn get_weight_a(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightA_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWeightB() /usr/include/opencv2/xfeatures2d.hpp:662
	#[inline]
	fn get_weight_b(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightB_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWeightContrast() /usr/include/opencv2/xfeatures2d.hpp:673
	#[inline]
	fn get_weight_contrast(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightContrast_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getWeightEntropy() /usr/include/opencv2/xfeatures2d.hpp:684
	#[inline]
	fn get_weight_entropy(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getWeightEntropy_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getSamplingPoints() /usr/include/opencv2/xfeatures2d.hpp:695
	#[inline]
	fn get_sampling_points(&self) -> Result<core::Vector<core::Point2f>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getSamplingPoints_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<core::Point2f>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getInitSeedIndexes() /usr/include/opencv2/xfeatures2d.hpp:772
	#[inline]
	fn get_init_seed_indexes(&self) -> Result<core::Vector<i32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getInitSeedIndexes_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<i32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getInitSeedCount() /usr/include/opencv2/xfeatures2d.hpp:780
	#[inline]
	fn get_init_seed_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getInitSeedCount_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getIterationCount() /usr/include/opencv2/xfeatures2d.hpp:787
	#[inline]
	fn get_iteration_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getIterationCount_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMaxClustersCount() /usr/include/opencv2/xfeatures2d.hpp:799
	#[inline]
	fn get_max_clusters_count(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getMaxClustersCount_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getClusterMinSize() /usr/include/opencv2/xfeatures2d.hpp:811
	#[inline]
	fn get_cluster_min_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getClusterMinSize_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getJoiningDistance() /usr/include/opencv2/xfeatures2d.hpp:824
	#[inline]
	fn get_joining_distance(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getJoiningDistance_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getDropThreshold() /usr/include/opencv2/xfeatures2d.hpp:835
	#[inline]
	fn get_drop_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getDropThreshold_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getDistanceFunction() /usr/include/opencv2/xfeatures2d.hpp:844
	#[inline]
	fn get_distance_function(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_getDistanceFunction_const(self.as_raw_PCTSignatures(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait PCTSignatures: core::AlgorithmTrait + crate::xfeatures2d::PCTSignaturesConst {
	fn as_raw_mut_PCTSignatures(&mut self) -> *mut c_void;

	// setGrayscaleBits(int) /usr/include/opencv2/xfeatures2d.hpp:598
	#[inline]
	fn set_grayscale_bits(&mut self, grayscale_bits: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setGrayscaleBits_int(self.as_raw_mut_PCTSignatures(), grayscale_bits, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWindowRadius(int) /usr/include/opencv2/xfeatures2d.hpp:611
	#[inline]
	fn set_window_radius(&mut self, radius: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWindowRadius_int(self.as_raw_mut_PCTSignatures(), radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWeightX(float) /usr/include/opencv2/xfeatures2d.hpp:623
	#[inline]
	fn set_weight_x(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightX_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWeightY(float) /usr/include/opencv2/xfeatures2d.hpp:634
	#[inline]
	fn set_weight_y(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightY_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWeightL(float) /usr/include/opencv2/xfeatures2d.hpp:645
	#[inline]
	fn set_weight_l(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightL_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWeightA(float) /usr/include/opencv2/xfeatures2d.hpp:656
	#[inline]
	fn set_weight_a(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightA_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWeightB(float) /usr/include/opencv2/xfeatures2d.hpp:667
	#[inline]
	fn set_weight_b(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightB_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWeightContrast(float) /usr/include/opencv2/xfeatures2d.hpp:678
	#[inline]
	fn set_weight_contrast(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightContrast_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWeightEntropy(float) /usr/include/opencv2/xfeatures2d.hpp:689
	#[inline]
	fn set_weight_entropy(&mut self, weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeightEntropy_float(self.as_raw_mut_PCTSignatures(), weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWeight(int, float) /usr/include/opencv2/xfeatures2d.hpp:713
	#[inline]
	fn set_weight(&mut self, idx: i32, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeight_int_float(self.as_raw_mut_PCTSignatures(), idx, value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setWeights(const std::vector<float> &) /usr/include/opencv2/xfeatures2d.hpp:727
	#[inline]
	fn set_weights(&mut self, weights: &core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setWeights_const_vector_float_R(self.as_raw_mut_PCTSignatures(), weights.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setTranslation(int, float) /usr/include/opencv2/xfeatures2d.hpp:743
	#[inline]
	fn set_translation(&mut self, idx: i32, value: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setTranslation_int_float(self.as_raw_mut_PCTSignatures(), idx, value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setTranslations(const std::vector<float> &) /usr/include/opencv2/xfeatures2d.hpp:757
	#[inline]
	fn set_translations(&mut self, translations: &core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setTranslations_const_vector_float_R(self.as_raw_mut_PCTSignatures(), translations.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setSamplingPoints(std::vector<Point2f>) /usr/include/opencv2/xfeatures2d.hpp:764
	#[inline]
	fn set_sampling_points(&mut self, mut sampling_points: core::Vector<core::Point2f>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setSamplingPoints_vector_Point2f_(self.as_raw_mut_PCTSignatures(), sampling_points.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setInitSeedIndexes(std::vector<int>) /usr/include/opencv2/xfeatures2d.hpp:776
	#[inline]
	fn set_init_seed_indexes(&mut self, mut init_seed_indexes: core::Vector<i32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setInitSeedIndexes_vector_int_(self.as_raw_mut_PCTSignatures(), init_seed_indexes.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setIterationCount(int) /usr/include/opencv2/xfeatures2d.hpp:793
	#[inline]
	fn set_iteration_count(&mut self, iteration_count: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setIterationCount_int(self.as_raw_mut_PCTSignatures(), iteration_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setMaxClustersCount(int) /usr/include/opencv2/xfeatures2d.hpp:804
	#[inline]
	fn set_max_clusters_count(&mut self, max_clusters_count: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setMaxClustersCount_int(self.as_raw_mut_PCTSignatures(), max_clusters_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setClusterMinSize(int) /usr/include/opencv2/xfeatures2d.hpp:817
	#[inline]
	fn set_cluster_min_size(&mut self, cluster_min_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setClusterMinSize_int(self.as_raw_mut_PCTSignatures(), cluster_min_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setJoiningDistance(float) /usr/include/opencv2/xfeatures2d.hpp:830
	#[inline]
	fn set_joining_distance(&mut self, joining_distance: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setJoiningDistance_float(self.as_raw_mut_PCTSignatures(), joining_distance, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setDropThreshold(float) /usr/include/opencv2/xfeatures2d.hpp:839
	#[inline]
	fn set_drop_threshold(&mut self, drop_threshold: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setDropThreshold_float(self.as_raw_mut_PCTSignatures(), drop_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setDistanceFunction(int) /usr/include/opencv2/xfeatures2d.hpp:849
	#[inline]
	fn set_distance_function(&mut self, distance_function: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_setDistanceFunction_int(self.as_raw_mut_PCTSignatures(), distance_function, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn PCTSignatures + '_ {
	/// ## C++ default parameters
	/// * init_sample_count: 2000
	/// * init_seed_count: 400
	/// * point_distribution: 0
	// create(const int, const int, const int) /usr/include/opencv2/xfeatures2d.hpp:497
	#[inline]
	pub fn create(init_sample_count: i32, init_seed_count: i32, point_distribution: i32) -> Result<core::Ptr<dyn crate::xfeatures2d::PCTSignatures>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_create_const_int_const_int_const_int(init_sample_count, init_seed_count, point_distribution, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::xfeatures2d::PCTSignatures>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// create(const std::vector<Point2f> &, const int) /usr/include/opencv2/xfeatures2d.hpp:511
	#[inline]
	pub fn create_1(init_sampling_points: &core::Vector<core::Point2f>, init_seed_count: i32) -> Result<core::Ptr<dyn crate::xfeatures2d::PCTSignatures>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_create_const_vector_Point2f_R_const_int(init_sampling_points.as_raw_VectorOfPoint2f(), init_seed_count, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::xfeatures2d::PCTSignatures>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// create(const std::vector<Point2f> &, const std::vector<int> &) /usr/include/opencv2/xfeatures2d.hpp:523
	#[inline]
	pub fn create_2(init_sampling_points: &core::Vector<core::Point2f>, init_cluster_seed_indexes: &core::Vector<i32>) -> Result<core::Ptr<dyn crate::xfeatures2d::PCTSignatures>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_create_const_vector_Point2f_R_const_vector_int_R(init_sampling_points.as_raw_VectorOfPoint2f(), init_cluster_seed_indexes.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::xfeatures2d::PCTSignatures>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * radius_to_shorter_side_ratio: 1.0/8
	/// * border_thickness: 1
	// drawSignature(cv::InputArray, cv::InputArray, cv::OutputArray, float, int) /usr/include/opencv2/xfeatures2d.hpp:559
	#[inline]
	pub fn draw_signature(source: &dyn core::ToInputArray, signature: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray, radius_to_shorter_side_ratio: f32, border_thickness: i32) -> Result<()> {
		input_array_arg!(source);
		input_array_arg!(signature);
		output_array_arg!(result);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_drawSignature_const__InputArrayR_const__InputArrayR_const__OutputArrayR_float_int(source.as_raw__InputArray(), signature.as_raw__InputArray(), result.as_raw__OutputArray(), radius_to_shorter_side_ratio, border_thickness, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// generateInitPoints(std::vector<Point2f> &, const int, int) /usr/include/opencv2/xfeatures2d.hpp:574
	#[inline]
	pub fn generate_init_points(init_points: &mut core::Vector<core::Point2f>, count: i32, point_distribution: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignatures_generateInitPoints_vector_Point2f_R_const_int_int(init_points.as_raw_mut_VectorOfPoint2f(), count, point_distribution, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
// PCTSignaturesSQFD /usr/include/opencv2/xfeatures2d.hpp:861
pub trait PCTSignaturesSQFDConst: core::AlgorithmTraitConst {
	fn as_raw_PCTSignaturesSQFD(&self) -> *const c_void;

	// computeQuadraticFormDistance(cv::InputArray, cv::InputArray) /usr/include/opencv2/xfeatures2d.hpp:884
	#[inline]
	fn compute_quadratic_form_distance(&self, _signature0: &dyn core::ToInputArray, _signature1: &dyn core::ToInputArray) -> Result<f32> {
		input_array_arg!(_signature0);
		input_array_arg!(_signature1);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistance_const_const__InputArrayR_const__InputArrayR(self.as_raw_PCTSignaturesSQFD(), _signature0.as_raw__InputArray(), _signature1.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// computeQuadraticFormDistances(const cv::Mat &, const std::vector<Mat> &, std::vector<float> &) /usr/include/opencv2/xfeatures2d.hpp:895
	#[inline]
	fn compute_quadratic_form_distances(&self, source_signature: &core::Mat, image_signatures: &core::Vector<core::Mat>, distances: &mut core::Vector<f32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_computeQuadraticFormDistances_const_const_MatR_const_vector_Mat_R_vector_float_R(self.as_raw_PCTSignaturesSQFD(), source_signature.as_raw_Mat(), image_signatures.as_raw_VectorOfMat(), distances.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait PCTSignaturesSQFD: core::AlgorithmTrait + crate::xfeatures2d::PCTSignaturesSQFDConst {
	fn as_raw_mut_PCTSignaturesSQFD(&mut self) -> *mut c_void;

}

impl dyn PCTSignaturesSQFD + '_ {
	/// ## C++ default parameters
	/// * distance_function: 3
	/// * similarity_function: 2
	/// * similarity_parameter: 1.0f
	// create(const int, const int, const float) /usr/include/opencv2/xfeatures2d.hpp:874
	#[inline]
	pub fn create(distance_function: i32, similarity_function: i32, similarity_parameter: f32) -> Result<core::Ptr<dyn crate::xfeatures2d::PCTSignaturesSQFD>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_PCTSignaturesSQFD_create_const_int_const_int_const_float(distance_function, similarity_function, similarity_parameter, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::xfeatures2d::PCTSignaturesSQFD>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// SURF /usr/include/opencv2/xfeatures2d/nonfree.hpp:82
pub trait SURFConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_SURF(&self) -> *const c_void;

	// getHessianThreshold() /usr/include/opencv2/xfeatures2d/nonfree.hpp:99
	#[inline]
	fn get_hessian_threshold(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_getHessianThreshold_const(self.as_raw_SURF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getNOctaves() /usr/include/opencv2/xfeatures2d/nonfree.hpp:102
	#[inline]
	fn get_n_octaves(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_getNOctaves_const(self.as_raw_SURF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getNOctaveLayers() /usr/include/opencv2/xfeatures2d/nonfree.hpp:105
	#[inline]
	fn get_n_octave_layers(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_getNOctaveLayers_const(self.as_raw_SURF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getExtended() /usr/include/opencv2/xfeatures2d/nonfree.hpp:108
	#[inline]
	fn get_extended(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_getExtended_const(self.as_raw_SURF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getUpright() /usr/include/opencv2/xfeatures2d/nonfree.hpp:111
	#[inline]
	fn get_upright(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_getUpright_const(self.as_raw_SURF(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait SURF: crate::features2d::Feature2DTrait + crate::xfeatures2d::SURFConst {
	fn as_raw_mut_SURF(&mut self) -> *mut c_void;

	// setHessianThreshold(double) /usr/include/opencv2/xfeatures2d/nonfree.hpp:98
	#[inline]
	fn set_hessian_threshold(&mut self, hessian_threshold: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_setHessianThreshold_double(self.as_raw_mut_SURF(), hessian_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setNOctaves(int) /usr/include/opencv2/xfeatures2d/nonfree.hpp:101
	#[inline]
	fn set_n_octaves(&mut self, n_octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_setNOctaves_int(self.as_raw_mut_SURF(), n_octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setNOctaveLayers(int) /usr/include/opencv2/xfeatures2d/nonfree.hpp:104
	#[inline]
	fn set_n_octave_layers(&mut self, n_octave_layers: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_setNOctaveLayers_int(self.as_raw_mut_SURF(), n_octave_layers, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setExtended(bool) /usr/include/opencv2/xfeatures2d/nonfree.hpp:107
	#[inline]
	fn set_extended(&mut self, extended: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_setExtended_bool(self.as_raw_mut_SURF(), extended, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setUpright(bool) /usr/include/opencv2/xfeatures2d/nonfree.hpp:110
	#[inline]
	fn set_upright(&mut self, upright: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_setUpright_bool(self.as_raw_mut_SURF(), upright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn SURF + '_ {
	/// ## C++ default parameters
	/// * hessian_threshold: 100
	/// * n_octaves: 4
	/// * n_octave_layers: 3
	/// * extended: false
	/// * upright: false
	// create(double, int, int, bool, bool) /usr/include/opencv2/xfeatures2d/nonfree.hpp:94
	#[inline]
	pub fn create(hessian_threshold: f64, n_octaves: i32, n_octave_layers: i32, extended: bool, upright: bool) -> Result<core::Ptr<dyn crate::xfeatures2d::SURF>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_SURF_create_double_int_int_bool_bool(hessian_threshold, n_octaves, n_octave_layers, extended, upright, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::xfeatures2d::SURF>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// StarDetector /usr/include/opencv2/xfeatures2d.hpp:110
pub trait StarDetectorTraitConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_StarDetector(&self) -> *const c_void;

}

pub trait StarDetectorTrait: crate::features2d::Feature2DTrait + crate::xfeatures2d::StarDetectorTraitConst {
	fn as_raw_mut_StarDetector(&mut self) -> *mut c_void;

}

// StarDetector /usr/include/opencv2/xfeatures2d.hpp:110
pub struct StarDetector {
	ptr: *mut c_void
}

opencv_type_boxed! { StarDetector }

impl Drop for StarDetector {
	fn drop(&mut self) {
		extern "C" { fn cv_StarDetector_delete(instance: *mut c_void); }
		unsafe { cv_StarDetector_delete(self.as_raw_mut_StarDetector()) };
	}
}

unsafe impl Send for StarDetector {}

impl core::AlgorithmTraitConst for StarDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StarDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::features2d::Feature2DTraitConst for StarDetector {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::features2d::Feature2DTrait for StarDetector {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xfeatures2d::StarDetectorTraitConst for StarDetector {
	#[inline] fn as_raw_StarDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::xfeatures2d::StarDetectorTrait for StarDetector {
	#[inline] fn as_raw_mut_StarDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl StarDetector {
	/// ## C++ default parameters
	/// * max_size: 45
	/// * response_threshold: 30
	/// * line_threshold_projected: 10
	/// * line_threshold_binarized: 8
	/// * suppress_nonmax_size: 5
	// create(int, int, int, int, int) /usr/include/opencv2/xfeatures2d.hpp:114
	#[inline]
	pub fn create(max_size: i32, response_threshold: i32, line_threshold_projected: i32, line_threshold_binarized: i32, suppress_nonmax_size: i32) -> Result<core::Ptr<crate::xfeatures2d::StarDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_StarDetector_create_int_int_int_int_int(max_size, response_threshold, line_threshold_projected, line_threshold_binarized, suppress_nonmax_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::xfeatures2d::StarDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { StarDetector, core::Algorithm, cv_StarDetector_to_Algorithm }

boxed_cast_base! { StarDetector, crate::features2d::Feature2D, cv_StarDetector_to_Feature2D }

// TBMR /usr/include/opencv2/xfeatures2d.hpp:1010
pub trait TBMRConst: crate::xfeatures2d::AffineFeature2DConst {
	fn as_raw_TBMR(&self) -> *const c_void;

	// getMinArea() /usr/include/opencv2/xfeatures2d.hpp:1019
	#[inline]
	fn get_min_area(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_getMinArea_const(self.as_raw_TBMR(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getMaxAreaRelative() /usr/include/opencv2/xfeatures2d.hpp:1021
	#[inline]
	fn get_max_area_relative(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_getMaxAreaRelative_const(self.as_raw_TBMR(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getScaleFactor() /usr/include/opencv2/xfeatures2d.hpp:1023
	#[inline]
	fn get_scale_factor(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_getScaleFactor_const(self.as_raw_TBMR(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getNScales() /usr/include/opencv2/xfeatures2d.hpp:1025
	#[inline]
	fn get_n_scales(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_getNScales_const(self.as_raw_TBMR(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait TBMR: crate::xfeatures2d::AffineFeature2D + crate::xfeatures2d::TBMRConst {
	fn as_raw_mut_TBMR(&mut self) -> *mut c_void;

	// setMinArea(int) /usr/include/opencv2/xfeatures2d.hpp:1018
	#[inline]
	fn set_min_area(&mut self, min_area: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_setMinArea_int(self.as_raw_mut_TBMR(), min_area, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setMaxAreaRelative(float) /usr/include/opencv2/xfeatures2d.hpp:1020
	#[inline]
	fn set_max_area_relative(&mut self, max_area: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_setMaxAreaRelative_float(self.as_raw_mut_TBMR(), max_area, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setScaleFactor(float) /usr/include/opencv2/xfeatures2d.hpp:1022
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_setScaleFactor_float(self.as_raw_mut_TBMR(), scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setNScales(int) /usr/include/opencv2/xfeatures2d.hpp:1024
	#[inline]
	fn set_n_scales(&mut self, n_scales: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_setNScales_int(self.as_raw_mut_TBMR(), n_scales, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn TBMR + '_ {
	/// ## C++ default parameters
	/// * min_area: 60
	/// * max_area_relative: 0.01f
	/// * scale_factor: 1.25f
	/// * n_scales: -1
	// create(int, float, float, int) /usr/include/opencv2/xfeatures2d.hpp:1013
	#[inline]
	pub fn create(min_area: i32, max_area_relative: f32, scale_factor: f32, n_scales: i32) -> Result<core::Ptr<dyn crate::xfeatures2d::TBMR>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_TBMR_create_int_float_float_int(min_area, max_area_relative, scale_factor, n_scales, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::xfeatures2d::TBMR>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}
// VGG /usr/include/opencv2/xfeatures2d.hpp:353
pub trait VGGConst: crate::features2d::Feature2DTraitConst {
	fn as_raw_VGG(&self) -> *const c_void;

	// getSigma() /usr/include/opencv2/xfeatures2d.hpp:367
	#[inline]
	fn get_sigma(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_getSigma_const(self.as_raw_VGG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getUseNormalizeImage() /usr/include/opencv2/xfeatures2d.hpp:370
	#[inline]
	fn get_use_normalize_image(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_getUseNormalizeImage_const(self.as_raw_VGG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getUseScaleOrientation() /usr/include/opencv2/xfeatures2d.hpp:373
	#[inline]
	fn get_use_scale_orientation(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_getUseScaleOrientation_const(self.as_raw_VGG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getScaleFactor() /usr/include/opencv2/xfeatures2d.hpp:376
	#[inline]
	fn get_scale_factor(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_getScaleFactor_const(self.as_raw_VGG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getUseNormalizeDescriptor() /usr/include/opencv2/xfeatures2d.hpp:379
	#[inline]
	fn get_use_normalize_descriptor(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_getUseNormalizeDescriptor_const(self.as_raw_VGG(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait VGG: crate::features2d::Feature2DTrait + crate::xfeatures2d::VGGConst {
	fn as_raw_mut_VGG(&mut self) -> *mut c_void;

	// setSigma(const float) /usr/include/opencv2/xfeatures2d.hpp:366
	#[inline]
	fn set_sigma(&mut self, isigma: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_setSigma_const_float(self.as_raw_mut_VGG(), isigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setUseNormalizeImage(const bool) /usr/include/opencv2/xfeatures2d.hpp:369
	#[inline]
	fn set_use_normalize_image(&mut self, img_normalize: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_setUseNormalizeImage_const_bool(self.as_raw_mut_VGG(), img_normalize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setUseScaleOrientation(const bool) /usr/include/opencv2/xfeatures2d.hpp:372
	#[inline]
	fn set_use_scale_orientation(&mut self, use_scale_orientation: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_setUseScaleOrientation_const_bool(self.as_raw_mut_VGG(), use_scale_orientation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setScaleFactor(const float) /usr/include/opencv2/xfeatures2d.hpp:375
	#[inline]
	fn set_scale_factor(&mut self, scale_factor: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_setScaleFactor_const_float(self.as_raw_mut_VGG(), scale_factor, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setUseNormalizeDescriptor(const bool) /usr/include/opencv2/xfeatures2d.hpp:378
	#[inline]
	fn set_use_normalize_descriptor(&mut self, dsc_normalize: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_setUseNormalizeDescriptor_const_bool(self.as_raw_mut_VGG(), dsc_normalize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

impl dyn VGG + '_ {
	/// ## C++ default parameters
	/// * desc: VGG::VGG_120
	/// * isigma: 1.4f
	/// * img_normalize: true
	/// * use_scale_orientation: true
	/// * scale_factor: 6.25f
	/// * dsc_normalize: false
	// create(int, float, bool, bool, float, bool) /usr/include/opencv2/xfeatures2d.hpp:362
	#[inline]
	pub fn create(desc: i32, isigma: f32, img_normalize: bool, use_scale_orientation: bool, scale_factor: f32, dsc_normalize: bool) -> Result<core::Ptr<dyn crate::xfeatures2d::VGG>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xfeatures2d_VGG_create_int_float_bool_bool_float_bool(desc, isigma, img_normalize, use_scale_orientation, scale_factor, dsc_normalize, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::xfeatures2d::VGG>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}