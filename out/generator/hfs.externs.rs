extern "C" {
	// setSegEgbThresholdI(float) /usr/include/opencv2/hfs.hpp:54
	pub fn cv_hfs_HfsSegment_setSegEgbThresholdI_float(instance: *mut c_void, c: f32, ocvrs_return: *mut Result_void);
	// getSegEgbThresholdI() /usr/include/opencv2/hfs.hpp:55
	pub fn cv_hfs_HfsSegment_getSegEgbThresholdI(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// setMinRegionSizeI(int) /usr/include/opencv2/hfs.hpp:63
	pub fn cv_hfs_HfsSegment_setMinRegionSizeI_int(instance: *mut c_void, n: i32, ocvrs_return: *mut Result_void);
	// getMinRegionSizeI() /usr/include/opencv2/hfs.hpp:64
	pub fn cv_hfs_HfsSegment_getMinRegionSizeI(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setSegEgbThresholdII(float) /usr/include/opencv2/hfs.hpp:73
	pub fn cv_hfs_HfsSegment_setSegEgbThresholdII_float(instance: *mut c_void, c: f32, ocvrs_return: *mut Result_void);
	// getSegEgbThresholdII() /usr/include/opencv2/hfs.hpp:74
	pub fn cv_hfs_HfsSegment_getSegEgbThresholdII(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// setMinRegionSizeII(int) /usr/include/opencv2/hfs.hpp:81
	pub fn cv_hfs_HfsSegment_setMinRegionSizeII_int(instance: *mut c_void, n: i32, ocvrs_return: *mut Result_void);
	// getMinRegionSizeII() /usr/include/opencv2/hfs.hpp:82
	pub fn cv_hfs_HfsSegment_getMinRegionSizeII(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setSpatialWeight(float) /usr/include/opencv2/hfs.hpp:94
	pub fn cv_hfs_HfsSegment_setSpatialWeight_float(instance: *mut c_void, w: f32, ocvrs_return: *mut Result_void);
	// getSpatialWeight() /usr/include/opencv2/hfs.hpp:95
	pub fn cv_hfs_HfsSegment_getSpatialWeight(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// setSlicSpixelSize(int) /usr/include/opencv2/hfs.hpp:105
	pub fn cv_hfs_HfsSegment_setSlicSpixelSize_int(instance: *mut c_void, n: i32, ocvrs_return: *mut Result_void);
	// getSlicSpixelSize() /usr/include/opencv2/hfs.hpp:106
	pub fn cv_hfs_HfsSegment_getSlicSpixelSize(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// setNumSlicIter(int) /usr/include/opencv2/hfs.hpp:113
	pub fn cv_hfs_HfsSegment_setNumSlicIter_int(instance: *mut c_void, n: i32, ocvrs_return: *mut Result_void);
	// getNumSlicIter() /usr/include/opencv2/hfs.hpp:114
	pub fn cv_hfs_HfsSegment_getNumSlicIter(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// performSegmentGpu(cv::InputArray, bool) /usr/include/opencv2/hfs.hpp:125
	pub fn cv_hfs_HfsSegment_performSegmentGpu_const__InputArrayR_bool(instance: *mut c_void, src: *const c_void, if_draw: bool, ocvrs_return: *mut Result<*mut c_void>);
	// performSegmentCpu(cv::InputArray, bool) /usr/include/opencv2/hfs.hpp:131
	pub fn cv_hfs_HfsSegment_performSegmentCpu_const__InputArrayR_bool(instance: *mut c_void, src: *const c_void, if_draw: bool, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, float, int, float, int, float, int, int) /usr/include/opencv2/hfs.hpp:144
	pub fn cv_hfs_HfsSegment_create_int_int_float_int_float_int_float_int_int(height: i32, width: i32, seg_egb_threshold_i: f32, min_region_size_i: i32, seg_egb_threshold_ii: f32, min_region_size_ii: i32, spatial_weight: f32, slic_spixel_size: i32, num_slic_iter: i32, ocvrs_return: *mut Result<*mut c_void>);
}
