#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Clustering and Search in Multi-Dimensional Spaces
//! 
//! This section documents OpenCV's interface to the FLANN library. FLANN (Fast Library for Approximate
//! Nearest Neighbors) is a library that contains a collection of algorithms optimized for fast nearest
//! neighbor search in large datasets and for high dimensional features. More information about FLANN
//! can be found in [Muja2009](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_Muja2009) .
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::IndexParamsTraitConst, super::IndexParamsTrait, super::KDTreeIndexParamsTraitConst, super::KDTreeIndexParamsTrait, super::LinearIndexParamsTraitConst, super::LinearIndexParamsTrait, super::CompositeIndexParamsTraitConst, super::CompositeIndexParamsTrait, super::AutotunedIndexParamsTraitConst, super::AutotunedIndexParamsTrait, super::HierarchicalClusteringIndexParamsTraitConst, super::HierarchicalClusteringIndexParamsTrait, super::KMeansIndexParamsTraitConst, super::KMeansIndexParamsTrait, super::LshIndexParamsTraitConst, super::LshIndexParamsTrait, super::SavedIndexParamsTraitConst, super::SavedIndexParamsTrait, super::SearchParamsTraitConst, super::SearchParamsTrait, super::IndexTraitConst, super::IndexTrait };
}

// AUTOTUNED /usr/include/opencv2/flann/defines.h:89
pub const AUTOTUNED: i32 = 255;
// BITS_PER_BASE /usr/include/opencv2/flann/kmeans_index.h:53
pub const BITS_PER_BASE: i32 = 2;
// BITS_PER_CHAR /usr/include/opencv2/flann/kmeans_index.h:52
pub const BITS_PER_CHAR: i32 = 8;
// BLOCKSIZE /usr/include/opencv2/flann/allocator.h:74
pub const BLOCKSIZE: u32 = 8192;
// CENTERS_GONZALES /usr/include/opencv2/flann/defines.h:103
pub const CENTERS_GONZALES: i32 = 1;
// CENTERS_KMEANSPP /usr/include/opencv2/flann/defines.h:104
pub const CENTERS_KMEANSPP: i32 = 2;
// CENTERS_RANDOM /usr/include/opencv2/flann/defines.h:102
pub const CENTERS_RANDOM: i32 = 0;
// COMPOSITE /usr/include/opencv2/flann/defines.h:86
pub const COMPOSITE: i32 = 3;
// CS /usr/include/opencv2/flann/defines.h:140
pub const CS: i32 = 7;
// EUCLIDEAN /usr/include/opencv2/flann/defines.h:134
pub const EUCLIDEAN: i32 = 1;
// FLANN_CENTERS_GONZALES /usr/include/opencv2/flann/defines.h:97
pub const FLANN_CENTERS_GONZALES: i32 = 1;
// FLANN_CENTERS_GROUPWISE /usr/include/opencv2/flann/defines.h:99
pub const FLANN_CENTERS_GROUPWISE: i32 = 3;
// FLANN_CENTERS_KMEANSPP /usr/include/opencv2/flann/defines.h:98
pub const FLANN_CENTERS_KMEANSPP: i32 = 2;
// FLANN_CENTERS_RANDOM /usr/include/opencv2/flann/defines.h:96
pub const FLANN_CENTERS_RANDOM: i32 = 0;
// FLANN_CHECKS_AUTOTUNED /usr/include/opencv2/flann/defines.h:162
pub const FLANN_CHECKS_AUTOTUNED: i32 = -2;
// FLANN_CHECKS_UNLIMITED /usr/include/opencv2/flann/defines.h:161
pub const FLANN_CHECKS_UNLIMITED: i32 = -1;
// FLANN_DIST_CHI_SQUARE /usr/include/opencv2/flann/defines.h:126
pub const FLANN_DIST_CHI_SQUARE: i32 = 7;
// FLANN_DIST_CS /usr/include/opencv2/flann/defines.h:127
pub const FLANN_DIST_CS: i32 = 7;
// FLANN_DIST_DNAMMING /usr/include/opencv2/flann/defines.h:131
pub const FLANN_DIST_DNAMMING: i32 = 10;
// FLANN_DIST_EUCLIDEAN /usr/include/opencv2/flann/defines.h:118
pub const FLANN_DIST_EUCLIDEAN: i32 = 1;
// FLANN_DIST_HAMMING /usr/include/opencv2/flann/defines.h:130
pub const FLANN_DIST_HAMMING: i32 = 9;
// FLANN_DIST_HELLINGER /usr/include/opencv2/flann/defines.h:125
pub const FLANN_DIST_HELLINGER: i32 = 6;
// FLANN_DIST_HIST_INTERSECT /usr/include/opencv2/flann/defines.h:124
pub const FLANN_DIST_HIST_INTERSECT: i32 = 5;
// FLANN_DIST_KL /usr/include/opencv2/flann/defines.h:129
pub const FLANN_DIST_KL: i32 = 8;
// FLANN_DIST_KULLBACK_LEIBLER /usr/include/opencv2/flann/defines.h:128
pub const FLANN_DIST_KULLBACK_LEIBLER: i32 = 8;
// FLANN_DIST_L1 /usr/include/opencv2/flann/defines.h:121
pub const FLANN_DIST_L1: i32 = 2;
// FLANN_DIST_L2 /usr/include/opencv2/flann/defines.h:119
pub const FLANN_DIST_L2: i32 = 1;
// FLANN_DIST_MANHATTAN /usr/include/opencv2/flann/defines.h:120
pub const FLANN_DIST_MANHATTAN: i32 = 2;
// FLANN_DIST_MAX /usr/include/opencv2/flann/defines.h:123
pub const FLANN_DIST_MAX: i32 = 4;
// FLANN_DIST_MINKOWSKI /usr/include/opencv2/flann/defines.h:122
pub const FLANN_DIST_MINKOWSKI: i32 = 3;
// FLANN_FLOAT32 /usr/include/opencv2/flann/defines.h:155
pub const FLANN_FLOAT32: i32 = 8;
// FLANN_FLOAT64 /usr/include/opencv2/flann/defines.h:156
pub const FLANN_FLOAT64: i32 = 9;
// FLANN_INDEX_AUTOTUNED /usr/include/opencv2/flann/defines.h:80
pub const FLANN_INDEX_AUTOTUNED: i32 = 255;
// FLANN_INDEX_COMPOSITE /usr/include/opencv2/flann/defines.h:75
pub const FLANN_INDEX_COMPOSITE: i32 = 3;
// FLANN_INDEX_HIERARCHICAL /usr/include/opencv2/flann/defines.h:77
pub const FLANN_INDEX_HIERARCHICAL: i32 = 5;
// FLANN_INDEX_KDTREE /usr/include/opencv2/flann/defines.h:73
pub const FLANN_INDEX_KDTREE: i32 = 1;
// FLANN_INDEX_KDTREE_SINGLE /usr/include/opencv2/flann/defines.h:76
pub const FLANN_INDEX_KDTREE_SINGLE: i32 = 4;
// FLANN_INDEX_KMEANS /usr/include/opencv2/flann/defines.h:74
pub const FLANN_INDEX_KMEANS: i32 = 2;
// FLANN_INDEX_LINEAR /usr/include/opencv2/flann/defines.h:72
pub const FLANN_INDEX_LINEAR: i32 = 0;
// FLANN_INDEX_LSH /usr/include/opencv2/flann/defines.h:78
pub const FLANN_INDEX_LSH: i32 = 6;
// FLANN_INDEX_SAVED /usr/include/opencv2/flann/defines.h:79
pub const FLANN_INDEX_SAVED: i32 = 254;
// FLANN_INDEX_TYPE_16S /usr/include/opencv2/flann/miniflann.hpp:61
pub const FLANN_INDEX_TYPE_16S: i32 = 3;
// FLANN_INDEX_TYPE_16U /usr/include/opencv2/flann/miniflann.hpp:60
pub const FLANN_INDEX_TYPE_16U: i32 = 2;
// FLANN_INDEX_TYPE_32F /usr/include/opencv2/flann/miniflann.hpp:63
pub const FLANN_INDEX_TYPE_32F: i32 = 5;
// FLANN_INDEX_TYPE_32S /usr/include/opencv2/flann/miniflann.hpp:62
pub const FLANN_INDEX_TYPE_32S: i32 = 4;
// FLANN_INDEX_TYPE_64F /usr/include/opencv2/flann/miniflann.hpp:64
pub const FLANN_INDEX_TYPE_64F: i32 = 6;
// FLANN_INDEX_TYPE_8S /usr/include/opencv2/flann/miniflann.hpp:59
pub const FLANN_INDEX_TYPE_8S: i32 = 1;
// FLANN_INDEX_TYPE_8U /usr/include/opencv2/flann/miniflann.hpp:58
pub const FLANN_INDEX_TYPE_8U: i32 = 0;
// FLANN_INDEX_TYPE_ALGORITHM /usr/include/opencv2/flann/miniflann.hpp:67
pub const FLANN_INDEX_TYPE_ALGORITHM: i32 = 9;
// FLANN_INDEX_TYPE_BOOL /usr/include/opencv2/flann/miniflann.hpp:66
pub const FLANN_INDEX_TYPE_BOOL: i32 = 8;
// FLANN_INDEX_TYPE_STRING /usr/include/opencv2/flann/miniflann.hpp:65
pub const FLANN_INDEX_TYPE_STRING: i32 = 7;
// FLANN_INT16 /usr/include/opencv2/flann/defines.h:148
pub const FLANN_INT16: i32 = 1;
// FLANN_INT32 /usr/include/opencv2/flann/defines.h:149
pub const FLANN_INT32: i32 = 2;
// FLANN_INT64 /usr/include/opencv2/flann/defines.h:150
pub const FLANN_INT64: i32 = 3;
// FLANN_INT8 /usr/include/opencv2/flann/defines.h:147
pub const FLANN_INT8: i32 = 0;
// FLANN_LOG_ERROR /usr/include/opencv2/flann/defines.h:111
pub const FLANN_LOG_ERROR: i32 = 2;
// FLANN_LOG_FATAL /usr/include/opencv2/flann/defines.h:110
pub const FLANN_LOG_FATAL: i32 = 1;
// FLANN_LOG_INFO /usr/include/opencv2/flann/defines.h:113
pub const FLANN_LOG_INFO: i32 = 4;
// FLANN_LOG_NONE /usr/include/opencv2/flann/defines.h:109
pub const FLANN_LOG_NONE: i32 = 0;
// FLANN_LOG_WARN /usr/include/opencv2/flann/defines.h:112
pub const FLANN_LOG_WARN: i32 = 3;
// FLANN_SIGNATURE_ /usr/include/opencv2/flann/saving.h:43
pub const FLANN_SIGNATURE_: &str = "FLANN_INDEX";
// FLANN_UINT16 /usr/include/opencv2/flann/defines.h:152
pub const FLANN_UINT16: i32 = 5;
// FLANN_UINT32 /usr/include/opencv2/flann/defines.h:153
pub const FLANN_UINT32: i32 = 6;
// FLANN_UINT64 /usr/include/opencv2/flann/defines.h:154
pub const FLANN_UINT64: i32 = 7;
// FLANN_UINT8 /usr/include/opencv2/flann/defines.h:151
pub const FLANN_UINT8: i32 = 4;
// FLANN_USE_BOOST /usr/include/opencv2/flann/dynamic_bitset.h:41
pub const FLANN_USE_BOOST: i32 = 0;
// FLANN_VERSION_ /usr/include/opencv2/flann/config.h:38
pub const FLANN_VERSION_: &str = "1.6.10";
// HELLINGER /usr/include/opencv2/flann/defines.h:139
pub const HELLINGER: i32 = 6;
// HIST_INTERSECT /usr/include/opencv2/flann/defines.h:138
pub const HIST_INTERSECT: i32 = 5;
// KDTREE /usr/include/opencv2/flann/defines.h:84
pub const KDTREE: i32 = 1;
// KDTREE_SINGLE /usr/include/opencv2/flann/defines.h:87
pub const KDTREE_SINGLE: i32 = 4;
// KL /usr/include/opencv2/flann/defines.h:141
pub const KL: i32 = 8;
// KMEANS /usr/include/opencv2/flann/defines.h:85
pub const KMEANS: i32 = 2;
// KULLBACK_LEIBLER /usr/include/opencv2/flann/defines.h:142
pub const KULLBACK_LEIBLER: i32 = 8;
// LAST_VALUE_FLANN_INDEX_TYPE /usr/include/opencv2/flann/miniflann.hpp:68
pub const LAST_VALUE_FLANN_INDEX_TYPE: i32 = 9;
// LINEAR /usr/include/opencv2/flann/defines.h:83
pub const LINEAR: i32 = 0;
// MANHATTAN /usr/include/opencv2/flann/defines.h:135
pub const MANHATTAN: i32 = 2;
// MAX_DIST /usr/include/opencv2/flann/defines.h:137
pub const MAX_DIST: i32 = 4;
// MINKOWSKI /usr/include/opencv2/flann/defines.h:136
pub const MINKOWSKI: i32 = 3;
// SAVED /usr/include/opencv2/flann/defines.h:88
pub const SAVED: i32 = 254;
// USE_UNORDERED_MAP /usr/include/opencv2/flann/lsh_table.h:46
pub const USE_UNORDERED_MAP: i32 = 1;
// WORDSIZE /usr/include/opencv2/flann/allocator.h:73
pub const WORDSIZE: u32 = 16;
// FlannIndexType /usr/include/opencv2/flann/miniflann.hpp:57
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlannIndexType {
	FLANN_INDEX_TYPE_8U = 0,
	FLANN_INDEX_TYPE_8S = 1,
	FLANN_INDEX_TYPE_16U = 2,
	FLANN_INDEX_TYPE_16S = 3,
	FLANN_INDEX_TYPE_32S = 4,
	FLANN_INDEX_TYPE_32F = 5,
	FLANN_INDEX_TYPE_64F = 6,
	FLANN_INDEX_TYPE_STRING = 7,
	FLANN_INDEX_TYPE_BOOL = 8,
	FLANN_INDEX_TYPE_ALGORITHM = 9,
	// LAST_VALUE_FLANN_INDEX_TYPE = 9 as isize, // duplicate discriminant
}

opencv_type_enum! { crate::flann::FlannIndexType }

// flann_algorithm_t /usr/include/opencv2/flann/defines.h:70
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum flann_algorithm_t {
	FLANN_INDEX_LINEAR = 0,
	FLANN_INDEX_KDTREE = 1,
	FLANN_INDEX_KMEANS = 2,
	FLANN_INDEX_COMPOSITE = 3,
	FLANN_INDEX_KDTREE_SINGLE = 4,
	FLANN_INDEX_HIERARCHICAL = 5,
	FLANN_INDEX_LSH = 6,
	FLANN_INDEX_SAVED = 254,
	FLANN_INDEX_AUTOTUNED = 255,
	// LINEAR = 0 as isize, // duplicate discriminant
	// KDTREE = 1 as isize, // duplicate discriminant
	// KMEANS = 2 as isize, // duplicate discriminant
	// COMPOSITE = 3 as isize, // duplicate discriminant
	// KDTREE_SINGLE = 4 as isize, // duplicate discriminant
	// SAVED = 254 as isize, // duplicate discriminant
	// AUTOTUNED = 255 as isize, // duplicate discriminant
}

opencv_type_enum! { crate::flann::flann_algorithm_t }

// flann_centers_init_t /usr/include/opencv2/flann/defines.h:94
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum flann_centers_init_t {
	FLANN_CENTERS_RANDOM = 0,
	FLANN_CENTERS_GONZALES = 1,
	FLANN_CENTERS_KMEANSPP = 2,
	FLANN_CENTERS_GROUPWISE = 3,
	// CENTERS_RANDOM = 0 as isize, // duplicate discriminant
	// CENTERS_GONZALES = 1 as isize, // duplicate discriminant
	// CENTERS_KMEANSPP = 2 as isize, // duplicate discriminant
}

opencv_type_enum! { crate::flann::flann_centers_init_t }

// flann_datatype_t /usr/include/opencv2/flann/defines.h:145
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum flann_datatype_t {
	FLANN_INT8 = 0,
	FLANN_INT16 = 1,
	FLANN_INT32 = 2,
	FLANN_INT64 = 3,
	FLANN_UINT8 = 4,
	FLANN_UINT16 = 5,
	FLANN_UINT32 = 6,
	FLANN_UINT64 = 7,
	FLANN_FLOAT32 = 8,
	FLANN_FLOAT64 = 9,
}

opencv_type_enum! { crate::flann::flann_datatype_t }

// flann_distance_t /usr/include/opencv2/flann/defines.h:116
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum flann_distance_t {
	FLANN_DIST_EUCLIDEAN = 1,
	// FLANN_DIST_L2 = 1 as isize, // duplicate discriminant
	FLANN_DIST_MANHATTAN = 2,
	// FLANN_DIST_L1 = 2 as isize, // duplicate discriminant
	FLANN_DIST_MINKOWSKI = 3,
	FLANN_DIST_MAX = 4,
	FLANN_DIST_HIST_INTERSECT = 5,
	FLANN_DIST_HELLINGER = 6,
	FLANN_DIST_CHI_SQUARE = 7,
	// FLANN_DIST_CS = 7 as isize, // duplicate discriminant
	FLANN_DIST_KULLBACK_LEIBLER = 8,
	// FLANN_DIST_KL = 8 as isize, // duplicate discriminant
	FLANN_DIST_HAMMING = 9,
	FLANN_DIST_DNAMMING = 10,
	// EUCLIDEAN = 1 as isize, // duplicate discriminant
	// MANHATTAN = 2 as isize, // duplicate discriminant
	// MINKOWSKI = 3 as isize, // duplicate discriminant
	// MAX_DIST = 4 as isize, // duplicate discriminant
	// HIST_INTERSECT = 5 as isize, // duplicate discriminant
	// HELLINGER = 6 as isize, // duplicate discriminant
	// CS = 7 as isize, // duplicate discriminant
	// KL = 8 as isize, // duplicate discriminant
	// KULLBACK_LEIBLER = 8 as isize, // duplicate discriminant
}

opencv_type_enum! { crate::flann::flann_distance_t }

// flann_log_level_t /usr/include/opencv2/flann/defines.h:107
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum flann_log_level_t {
	FLANN_LOG_NONE = 0,
	FLANN_LOG_FATAL = 1,
	FLANN_LOG_ERROR = 2,
	FLANN_LOG_WARN = 3,
	FLANN_LOG_INFO = 4,
}

opencv_type_enum! { crate::flann::flann_log_level_t }

// BucketKey /usr/include/opencv2/flann/lsh_table.h:80
pub type bucket_key = u32;
// FeatureIndex /usr/include/opencv2/flann/lsh_table.h:77
pub type feature_index = u32;
// flann_distance_type() /usr/include/opencv2/flann.hpp:61
#[inline]
pub fn flann_distance_type() -> Result<crate::flann::flann_distance_t> {
	return_send!(via ocvrs_return);
	unsafe { sys::cvflann_flann_distance_type(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// set_distance_type(cvflann::flann_distance_t, int) /usr/include/opencv2/flann.hpp:62
#[inline]
pub fn set_distance_type(distance_type: crate::flann::flann_distance_t, order: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cvflann_set_distance_type_flann_distance_t_int(distance_type, order, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// AutotunedIndexParams /usr/include/opencv2/flann/miniflann.hpp:116
pub trait AutotunedIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_AutotunedIndexParams(&self) -> *const c_void;

}

pub trait AutotunedIndexParamsTrait: crate::flann::AutotunedIndexParamsTraitConst + crate::flann::IndexParamsTrait {
	fn as_raw_mut_AutotunedIndexParams(&mut self) -> *mut c_void;

}

// AutotunedIndexParams /usr/include/opencv2/flann/miniflann.hpp:116
pub struct AutotunedIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { AutotunedIndexParams }

impl Drop for AutotunedIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_AutotunedIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_AutotunedIndexParams_delete(self.as_raw_mut_AutotunedIndexParams()) };
	}
}

unsafe impl Send for AutotunedIndexParams {}

impl crate::flann::IndexParamsTraitConst for AutotunedIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for AutotunedIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::AutotunedIndexParamsTraitConst for AutotunedIndexParams {
	#[inline] fn as_raw_AutotunedIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::AutotunedIndexParamsTrait for AutotunedIndexParams {
	#[inline] fn as_raw_mut_AutotunedIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl AutotunedIndexParams {
	/// ## C++ default parameters
	/// * target_precision: 0.8f
	/// * build_weight: 0.01f
	/// * memory_weight: 0
	/// * sample_fraction: 0.1f
	// AutotunedIndexParams(float, float, float, float) /usr/include/opencv2/flann/miniflann.hpp:118
	#[inline]
	pub fn new(target_precision: f32, build_weight: f32, memory_weight: f32, sample_fraction: f32) -> Result<crate::flann::AutotunedIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_AutotunedIndexParams_AutotunedIndexParams_float_float_float_float(target_precision, build_weight, memory_weight, sample_fraction, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::AutotunedIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { AutotunedIndexParams, crate::flann::IndexParams, cv_AutotunedIndexParams_to_IndexParams }

// CompositeIndexParams /usr/include/opencv2/flann/miniflann.hpp:110
pub trait CompositeIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_CompositeIndexParams(&self) -> *const c_void;

}

pub trait CompositeIndexParamsTrait: crate::flann::CompositeIndexParamsTraitConst + crate::flann::IndexParamsTrait {
	fn as_raw_mut_CompositeIndexParams(&mut self) -> *mut c_void;

}

// CompositeIndexParams /usr/include/opencv2/flann/miniflann.hpp:110
pub struct CompositeIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { CompositeIndexParams }

impl Drop for CompositeIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_CompositeIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_CompositeIndexParams_delete(self.as_raw_mut_CompositeIndexParams()) };
	}
}

unsafe impl Send for CompositeIndexParams {}

impl crate::flann::IndexParamsTraitConst for CompositeIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for CompositeIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::CompositeIndexParamsTraitConst for CompositeIndexParams {
	#[inline] fn as_raw_CompositeIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::CompositeIndexParamsTrait for CompositeIndexParams {
	#[inline] fn as_raw_mut_CompositeIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CompositeIndexParams {
	/// ## C++ default parameters
	/// * trees: 4
	/// * branching: 32
	/// * iterations: 11
	/// * centers_init: cvflann::FLANN_CENTERS_RANDOM
	/// * cb_index: 0.2f
	// CompositeIndexParams(int, int, int, cvflann::flann_centers_init_t, float) /usr/include/opencv2/flann/miniflann.hpp:112
	#[inline]
	pub fn new(trees: i32, branching: i32, iterations: i32, centers_init: crate::flann::flann_centers_init_t, cb_index: f32) -> Result<crate::flann::CompositeIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_CompositeIndexParams_CompositeIndexParams_int_int_int_flann_centers_init_t_float(trees, branching, iterations, centers_init, cb_index, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::CompositeIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { CompositeIndexParams, crate::flann::IndexParams, cv_CompositeIndexParams_to_IndexParams }

// HierarchicalClusteringIndexParams /usr/include/opencv2/flann/miniflann.hpp:122
pub trait HierarchicalClusteringIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_HierarchicalClusteringIndexParams(&self) -> *const c_void;

}

pub trait HierarchicalClusteringIndexParamsTrait: crate::flann::HierarchicalClusteringIndexParamsTraitConst + crate::flann::IndexParamsTrait {
	fn as_raw_mut_HierarchicalClusteringIndexParams(&mut self) -> *mut c_void;

}

// HierarchicalClusteringIndexParams /usr/include/opencv2/flann/miniflann.hpp:122
pub struct HierarchicalClusteringIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { HierarchicalClusteringIndexParams }

impl Drop for HierarchicalClusteringIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_HierarchicalClusteringIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_HierarchicalClusteringIndexParams_delete(self.as_raw_mut_HierarchicalClusteringIndexParams()) };
	}
}

unsafe impl Send for HierarchicalClusteringIndexParams {}

impl crate::flann::IndexParamsTraitConst for HierarchicalClusteringIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for HierarchicalClusteringIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::HierarchicalClusteringIndexParamsTraitConst for HierarchicalClusteringIndexParams {
	#[inline] fn as_raw_HierarchicalClusteringIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::HierarchicalClusteringIndexParamsTrait for HierarchicalClusteringIndexParams {
	#[inline] fn as_raw_mut_HierarchicalClusteringIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl HierarchicalClusteringIndexParams {
	/// ## C++ default parameters
	/// * branching: 32
	/// * centers_init: cvflann::FLANN_CENTERS_RANDOM
	/// * trees: 4
	/// * leaf_size: 100
	// HierarchicalClusteringIndexParams(int, cvflann::flann_centers_init_t, int, int) /usr/include/opencv2/flann/miniflann.hpp:124
	#[inline]
	pub fn new(branching: i32, centers_init: crate::flann::flann_centers_init_t, trees: i32, leaf_size: i32) -> Result<crate::flann::HierarchicalClusteringIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_HierarchicalClusteringIndexParams_HierarchicalClusteringIndexParams_int_flann_centers_init_t_int_int(branching, centers_init, trees, leaf_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::HierarchicalClusteringIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { HierarchicalClusteringIndexParams, crate::flann::IndexParams, cv_HierarchicalClusteringIndexParams_to_IndexParams }

// Index /usr/include/opencv2/flann/miniflann.hpp:150
pub trait IndexTraitConst {
	fn as_raw_Index(&self) -> *const c_void;

	// save(const cv::String &) /usr/include/opencv2/flann/miniflann.hpp:165
	#[inline]
	fn save(&self, filename: &str) -> Result<()> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_save_const_const_StringR(self.as_raw_Index(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getDistance() /usr/include/opencv2/flann/miniflann.hpp:168
	#[inline]
	fn get_distance(&self) -> Result<crate::flann::flann_distance_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_getDistance_const(self.as_raw_Index(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getAlgorithm() /usr/include/opencv2/flann/miniflann.hpp:169
	#[inline]
	fn get_algorithm(&self) -> Result<crate::flann::flann_algorithm_t> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_getAlgorithm_const(self.as_raw_Index(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait IndexTrait: crate::flann::IndexTraitConst {
	fn as_raw_mut_Index(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * dist_type: cvflann::FLANN_DIST_L2
	// build(cv::InputArray, const cv::flann::IndexParams &, cvflann::flann_distance_t) /usr/include/opencv2/flann/miniflann.hpp:157
	#[inline]
	fn build(&mut self, features: &dyn core::ToInputArray, params: &crate::flann::IndexParams, dist_type: crate::flann::flann_distance_t) -> Result<()> {
		input_array_arg!(features);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_build_const__InputArrayR_const_IndexParamsR_flann_distance_t(self.as_raw_mut_Index(), features.as_raw__InputArray(), params.as_raw_IndexParams(), dist_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * params: SearchParams()
	// knnSearch(cv::InputArray, cv::OutputArray, cv::OutputArray, int, const cv::flann::SearchParams &) /usr/include/opencv2/flann/miniflann.hpp:158
	#[inline]
	fn knn_search(&mut self, query: &dyn core::ToInputArray, indices: &mut dyn core::ToOutputArray, dists: &mut dyn core::ToOutputArray, knn: i32, params: &crate::flann::SearchParams) -> Result<()> {
		input_array_arg!(query);
		output_array_arg!(indices);
		output_array_arg!(dists);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_knnSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_int_const_SearchParamsR(self.as_raw_mut_Index(), query.as_raw__InputArray(), indices.as_raw__OutputArray(), dists.as_raw__OutputArray(), knn, params.as_raw_SearchParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * params: SearchParams()
	// radiusSearch(cv::InputArray, cv::OutputArray, cv::OutputArray, double, int, const cv::flann::SearchParams &) /usr/include/opencv2/flann/miniflann.hpp:161
	#[inline]
	fn radius_search(&mut self, query: &dyn core::ToInputArray, indices: &mut dyn core::ToOutputArray, dists: &mut dyn core::ToOutputArray, radius: f64, max_results: i32, params: &crate::flann::SearchParams) -> Result<i32> {
		input_array_arg!(query);
		output_array_arg!(indices);
		output_array_arg!(dists);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_radiusSearch_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_double_int_const_SearchParamsR(self.as_raw_mut_Index(), query.as_raw__InputArray(), indices.as_raw__OutputArray(), dists.as_raw__OutputArray(), radius, max_results, params.as_raw_SearchParams(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// load(cv::InputArray, const cv::String &) /usr/include/opencv2/flann/miniflann.hpp:166
	#[inline]
	fn load(&mut self, features: &dyn core::ToInputArray, filename: &str) -> Result<bool> {
		input_array_arg!(features);
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_load_const__InputArrayR_const_StringR(self.as_raw_mut_Index(), features.as_raw__InputArray(), filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// release() /usr/include/opencv2/flann/miniflann.hpp:167
	#[inline]
	fn release(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_release(self.as_raw_mut_Index(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// Index /usr/include/opencv2/flann/miniflann.hpp:150
pub struct Index {
	ptr: *mut c_void
}

opencv_type_boxed! { Index }

impl Drop for Index {
	fn drop(&mut self) {
		extern "C" { fn cv_Index_delete(instance: *mut c_void); }
		unsafe { cv_Index_delete(self.as_raw_mut_Index()) };
	}
}

unsafe impl Send for Index {}

impl crate::flann::IndexTraitConst for Index {
	#[inline] fn as_raw_Index(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexTrait for Index {
	#[inline] fn as_raw_mut_Index(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Index {
	// Index() /usr/include/opencv2/flann/miniflann.hpp:153
	#[inline]
	pub fn default() -> Result<crate::flann::Index> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_Index(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::Index::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * dist_type: cvflann::FLANN_DIST_L2
	// Index(cv::InputArray, const cv::flann::IndexParams &, cvflann::flann_distance_t) /usr/include/opencv2/flann/miniflann.hpp:154
	#[inline]
	pub fn new(features: &dyn core::ToInputArray, params: &crate::flann::IndexParams, dist_type: crate::flann::flann_distance_t) -> Result<crate::flann::Index> {
		input_array_arg!(features);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_Index_Index_const__InputArrayR_const_IndexParamsR_flann_distance_t(features.as_raw__InputArray(), params.as_raw_IndexParams(), dist_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::Index::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// IndexParams /usr/include/opencv2/flann/miniflann.hpp:71
pub trait IndexParamsTraitConst {
	fn as_raw_IndexParams(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * default_val: String()
	// getString(const cv::String &, const cv::String &) /usr/include/opencv2/flann/miniflann.hpp:76
	#[inline]
	fn get_string(&self, key: &str, default_val: &str) -> Result<String> {
		extern_container_arg!(key);
		extern_container_arg!(default_val);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_getString_const_const_StringR_const_StringR(self.as_raw_IndexParams(), key.opencv_as_extern(), default_val.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { String::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * default_val: -1
	// getInt(const cv::String &, int) /usr/include/opencv2/flann/miniflann.hpp:77
	#[inline]
	fn get_int(&self, key: &str, default_val: i32) -> Result<i32> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_getInt_const_const_StringR_int(self.as_raw_IndexParams(), key.opencv_as_extern(), default_val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * default_val: -1
	// getDouble(const cv::String &, double) /usr/include/opencv2/flann/miniflann.hpp:78
	#[inline]
	fn get_double(&self, key: &str, default_val: f64) -> Result<f64> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_getDouble_const_const_StringR_double(self.as_raw_IndexParams(), key.opencv_as_extern(), default_val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getAll(std::vector<String> &, std::vector<FlannIndexType> &, std::vector<String> &, std::vector<double> &) /usr/include/opencv2/flann/miniflann.hpp:88
	#[inline]
	fn get_all(&self, names: &mut core::Vector<String>, types: &mut core::Vector<crate::flann::FlannIndexType>, str_values: &mut core::Vector<String>, num_values: &mut core::Vector<f64>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_getAll_const_vector_String_R_vector_FlannIndexType_R_vector_String_R_vector_double_R(self.as_raw_IndexParams(), names.as_raw_mut_VectorOfString(), types.as_raw_mut_VectorOfFlannIndexType(), str_values.as_raw_mut_VectorOfString(), num_values.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait IndexParamsTrait: crate::flann::IndexParamsTraitConst {
	fn as_raw_mut_IndexParams(&mut self) -> *mut c_void;

	// params /usr/include/opencv2/flann/miniflann.hpp:93
	#[inline]
	fn params(&mut self) -> *mut c_void {
		let ret = unsafe { sys::cv_flann_IndexParams_getPropParams(self.as_raw_mut_IndexParams()) };
		ret
	}
	
	// params /usr/include/opencv2/flann/miniflann.hpp:93
	#[inline]
	unsafe fn set_params(&mut self, val: *mut c_void) {
		let ret = { sys::cv_flann_IndexParams_setPropParams_voidX(self.as_raw_mut_IndexParams(), val) };
		ret
	}
	
	// setString(const cv::String &, const cv::String &) /usr/include/opencv2/flann/miniflann.hpp:80
	#[inline]
	fn set_string(&mut self, key: &str, value: &str) -> Result<()> {
		extern_container_arg!(key);
		extern_container_arg!(value);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_setString_const_StringR_const_StringR(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setInt(const cv::String &, int) /usr/include/opencv2/flann/miniflann.hpp:81
	#[inline]
	fn set_int(&mut self, key: &str, value: i32) -> Result<()> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_setInt_const_StringR_int(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setDouble(const cv::String &, double) /usr/include/opencv2/flann/miniflann.hpp:82
	#[inline]
	fn set_double(&mut self, key: &str, value: f64) -> Result<()> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_setDouble_const_StringR_double(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setFloat(const cv::String &, float) /usr/include/opencv2/flann/miniflann.hpp:83
	#[inline]
	fn set_float(&mut self, key: &str, value: f32) -> Result<()> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_setFloat_const_StringR_float(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setBool(const cv::String &, bool) /usr/include/opencv2/flann/miniflann.hpp:84
	#[inline]
	fn set_bool(&mut self, key: &str, value: bool) -> Result<()> {
		extern_container_arg!(key);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_setBool_const_StringR_bool(self.as_raw_mut_IndexParams(), key.opencv_as_extern(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setAlgorithm(int) /usr/include/opencv2/flann/miniflann.hpp:85
	#[inline]
	fn set_algorithm(&mut self, value: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_setAlgorithm_int(self.as_raw_mut_IndexParams(), value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// IndexParams /usr/include/opencv2/flann/miniflann.hpp:71
pub struct IndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { IndexParams }

impl Drop for IndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_IndexParams_delete(instance: *mut c_void); }
		unsafe { cv_IndexParams_delete(self.as_raw_mut_IndexParams()) };
	}
}

unsafe impl Send for IndexParams {}

impl crate::flann::IndexParamsTraitConst for IndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for IndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl IndexParams {
	// IndexParams() /usr/include/opencv2/flann/miniflann.hpp:73
	#[inline]
	pub fn default() -> Result<crate::flann::IndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_IndexParams_IndexParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::IndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

// KDTreeIndexParams /usr/include/opencv2/flann/miniflann.hpp:100
pub trait KDTreeIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_KDTreeIndexParams(&self) -> *const c_void;

}

pub trait KDTreeIndexParamsTrait: crate::flann::IndexParamsTrait + crate::flann::KDTreeIndexParamsTraitConst {
	fn as_raw_mut_KDTreeIndexParams(&mut self) -> *mut c_void;

}

// KDTreeIndexParams /usr/include/opencv2/flann/miniflann.hpp:100
pub struct KDTreeIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { KDTreeIndexParams }

impl Drop for KDTreeIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_KDTreeIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_KDTreeIndexParams_delete(self.as_raw_mut_KDTreeIndexParams()) };
	}
}

unsafe impl Send for KDTreeIndexParams {}

impl crate::flann::IndexParamsTraitConst for KDTreeIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for KDTreeIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::KDTreeIndexParamsTraitConst for KDTreeIndexParams {
	#[inline] fn as_raw_KDTreeIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::KDTreeIndexParamsTrait for KDTreeIndexParams {
	#[inline] fn as_raw_mut_KDTreeIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KDTreeIndexParams {
	/// ## C++ default parameters
	/// * trees: 4
	// KDTreeIndexParams(int) /usr/include/opencv2/flann/miniflann.hpp:102
	#[inline]
	pub fn new(trees: i32) -> Result<crate::flann::KDTreeIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_KDTreeIndexParams_KDTreeIndexParams_int(trees, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::KDTreeIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { KDTreeIndexParams, crate::flann::IndexParams, cv_KDTreeIndexParams_to_IndexParams }

// KMeansIndexParams /usr/include/opencv2/flann/miniflann.hpp:128
pub trait KMeansIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_KMeansIndexParams(&self) -> *const c_void;

}

pub trait KMeansIndexParamsTrait: crate::flann::IndexParamsTrait + crate::flann::KMeansIndexParamsTraitConst {
	fn as_raw_mut_KMeansIndexParams(&mut self) -> *mut c_void;

}

// KMeansIndexParams /usr/include/opencv2/flann/miniflann.hpp:128
pub struct KMeansIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { KMeansIndexParams }

impl Drop for KMeansIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_KMeansIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_KMeansIndexParams_delete(self.as_raw_mut_KMeansIndexParams()) };
	}
}

unsafe impl Send for KMeansIndexParams {}

impl crate::flann::IndexParamsTraitConst for KMeansIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for KMeansIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::KMeansIndexParamsTraitConst for KMeansIndexParams {
	#[inline] fn as_raw_KMeansIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::KMeansIndexParamsTrait for KMeansIndexParams {
	#[inline] fn as_raw_mut_KMeansIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl KMeansIndexParams {
	/// ## C++ default parameters
	/// * branching: 32
	/// * iterations: 11
	/// * centers_init: cvflann::FLANN_CENTERS_RANDOM
	/// * cb_index: 0.2f
	// KMeansIndexParams(int, int, cvflann::flann_centers_init_t, float) /usr/include/opencv2/flann/miniflann.hpp:130
	#[inline]
	pub fn new(branching: i32, iterations: i32, centers_init: crate::flann::flann_centers_init_t, cb_index: f32) -> Result<crate::flann::KMeansIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_KMeansIndexParams_KMeansIndexParams_int_int_flann_centers_init_t_float(branching, iterations, centers_init, cb_index, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::KMeansIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { KMeansIndexParams, crate::flann::IndexParams, cv_KMeansIndexParams_to_IndexParams }

// LinearIndexParams /usr/include/opencv2/flann/miniflann.hpp:105
pub trait LinearIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_LinearIndexParams(&self) -> *const c_void;

}

pub trait LinearIndexParamsTrait: crate::flann::IndexParamsTrait + crate::flann::LinearIndexParamsTraitConst {
	fn as_raw_mut_LinearIndexParams(&mut self) -> *mut c_void;

}

// LinearIndexParams /usr/include/opencv2/flann/miniflann.hpp:105
pub struct LinearIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { LinearIndexParams }

impl Drop for LinearIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_LinearIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_LinearIndexParams_delete(self.as_raw_mut_LinearIndexParams()) };
	}
}

unsafe impl Send for LinearIndexParams {}

impl crate::flann::IndexParamsTraitConst for LinearIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for LinearIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::LinearIndexParamsTraitConst for LinearIndexParams {
	#[inline] fn as_raw_LinearIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::LinearIndexParamsTrait for LinearIndexParams {
	#[inline] fn as_raw_mut_LinearIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LinearIndexParams {
	// LinearIndexParams() /usr/include/opencv2/flann/miniflann.hpp:107
	#[inline]
	pub fn default() -> Result<crate::flann::LinearIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_LinearIndexParams_LinearIndexParams(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::LinearIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { LinearIndexParams, crate::flann::IndexParams, cv_LinearIndexParams_to_IndexParams }

// LshIndexParams /usr/include/opencv2/flann/miniflann.hpp:134
pub trait LshIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_LshIndexParams(&self) -> *const c_void;

}

pub trait LshIndexParamsTrait: crate::flann::IndexParamsTrait + crate::flann::LshIndexParamsTraitConst {
	fn as_raw_mut_LshIndexParams(&mut self) -> *mut c_void;

}

// LshIndexParams /usr/include/opencv2/flann/miniflann.hpp:134
pub struct LshIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { LshIndexParams }

impl Drop for LshIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_LshIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_LshIndexParams_delete(self.as_raw_mut_LshIndexParams()) };
	}
}

unsafe impl Send for LshIndexParams {}

impl crate::flann::IndexParamsTraitConst for LshIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for LshIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::LshIndexParamsTraitConst for LshIndexParams {
	#[inline] fn as_raw_LshIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::LshIndexParamsTrait for LshIndexParams {
	#[inline] fn as_raw_mut_LshIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LshIndexParams {
	// LshIndexParams(int, int, int) /usr/include/opencv2/flann/miniflann.hpp:136
	#[inline]
	pub fn new(table_number: i32, key_size: i32, multi_probe_level: i32) -> Result<crate::flann::LshIndexParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_LshIndexParams_LshIndexParams_int_int_int(table_number, key_size, multi_probe_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::LshIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { LshIndexParams, crate::flann::IndexParams, cv_LshIndexParams_to_IndexParams }

// SavedIndexParams /usr/include/opencv2/flann/miniflann.hpp:139
pub trait SavedIndexParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_SavedIndexParams(&self) -> *const c_void;

}

pub trait SavedIndexParamsTrait: crate::flann::IndexParamsTrait + crate::flann::SavedIndexParamsTraitConst {
	fn as_raw_mut_SavedIndexParams(&mut self) -> *mut c_void;

}

// SavedIndexParams /usr/include/opencv2/flann/miniflann.hpp:139
pub struct SavedIndexParams {
	ptr: *mut c_void
}

opencv_type_boxed! { SavedIndexParams }

impl Drop for SavedIndexParams {
	fn drop(&mut self) {
		extern "C" { fn cv_SavedIndexParams_delete(instance: *mut c_void); }
		unsafe { cv_SavedIndexParams_delete(self.as_raw_mut_SavedIndexParams()) };
	}
}

unsafe impl Send for SavedIndexParams {}

impl crate::flann::IndexParamsTraitConst for SavedIndexParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for SavedIndexParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::SavedIndexParamsTraitConst for SavedIndexParams {
	#[inline] fn as_raw_SavedIndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::SavedIndexParamsTrait for SavedIndexParams {
	#[inline] fn as_raw_mut_SavedIndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SavedIndexParams {
	// SavedIndexParams(const cv::String &) /usr/include/opencv2/flann/miniflann.hpp:141
	#[inline]
	pub fn new(filename: &str) -> Result<crate::flann::SavedIndexParams> {
		extern_container_arg!(filename);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_SavedIndexParams_SavedIndexParams_const_StringR(filename.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::SavedIndexParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SavedIndexParams, crate::flann::IndexParams, cv_SavedIndexParams_to_IndexParams }

// SearchParams /usr/include/opencv2/flann/miniflann.hpp:144
pub trait SearchParamsTraitConst: crate::flann::IndexParamsTraitConst {
	fn as_raw_SearchParams(&self) -> *const c_void;

}

pub trait SearchParamsTrait: crate::flann::IndexParamsTrait + crate::flann::SearchParamsTraitConst {
	fn as_raw_mut_SearchParams(&mut self) -> *mut c_void;

}

// SearchParams /usr/include/opencv2/flann/miniflann.hpp:144
pub struct SearchParams {
	ptr: *mut c_void
}

opencv_type_boxed! { SearchParams }

impl Drop for SearchParams {
	fn drop(&mut self) {
		extern "C" { fn cv_SearchParams_delete(instance: *mut c_void); }
		unsafe { cv_SearchParams_delete(self.as_raw_mut_SearchParams()) };
	}
}

unsafe impl Send for SearchParams {}

impl crate::flann::IndexParamsTraitConst for SearchParams {
	#[inline] fn as_raw_IndexParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::IndexParamsTrait for SearchParams {
	#[inline] fn as_raw_mut_IndexParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::flann::SearchParamsTraitConst for SearchParams {
	#[inline] fn as_raw_SearchParams(&self) -> *const c_void { self.as_raw() }
}

impl crate::flann::SearchParamsTrait for SearchParams {
	#[inline] fn as_raw_mut_SearchParams(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SearchParams {
	// SearchParams(int, float, bool, bool) /usr/include/opencv2/flann/miniflann.hpp:146
	#[inline]
	pub fn new(checks: i32, eps: f32, sorted: bool, explore_all_trees: bool) -> Result<crate::flann::SearchParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_SearchParams_SearchParams_int_float_bool_bool(checks, eps, sorted, explore_all_trees, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::SearchParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * checks: 32
	/// * eps: 0
	/// * sorted: true
	// SearchParams(int, float, bool) /usr/include/opencv2/flann/miniflann.hpp:147
	#[inline]
	pub fn new_1(checks: i32, eps: f32, sorted: bool) -> Result<crate::flann::SearchParams> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_flann_SearchParams_SearchParams_int_float_bool(checks, eps, sorted, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::flann::SearchParams::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SearchParams, crate::flann::IndexParams, cv_SearchParams_to_IndexParams }
