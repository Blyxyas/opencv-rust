#include "ocvrs_common.hpp"
#include <opencv2/hfs.hpp>
#include "hfs_types.hpp"

extern "C" {
	// setSegEgbThresholdI(float) /usr/include/opencv2/hfs.hpp:54
	void cv_hfs_HfsSegment_setSegEgbThresholdI_float(cv::hfs::HfsSegment* instance, float c, Result_void* ocvrs_return) {
		try {
			instance->setSegEgbThresholdI(c);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSegEgbThresholdI() /usr/include/opencv2/hfs.hpp:55
	void cv_hfs_HfsSegment_getSegEgbThresholdI(cv::hfs::HfsSegment* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSegEgbThresholdI();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setMinRegionSizeI(int) /usr/include/opencv2/hfs.hpp:63
	void cv_hfs_HfsSegment_setMinRegionSizeI_int(cv::hfs::HfsSegment* instance, int n, Result_void* ocvrs_return) {
		try {
			instance->setMinRegionSizeI(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinRegionSizeI() /usr/include/opencv2/hfs.hpp:64
	void cv_hfs_HfsSegment_getMinRegionSizeI(cv::hfs::HfsSegment* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinRegionSizeI();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setSegEgbThresholdII(float) /usr/include/opencv2/hfs.hpp:73
	void cv_hfs_HfsSegment_setSegEgbThresholdII_float(cv::hfs::HfsSegment* instance, float c, Result_void* ocvrs_return) {
		try {
			instance->setSegEgbThresholdII(c);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSegEgbThresholdII() /usr/include/opencv2/hfs.hpp:74
	void cv_hfs_HfsSegment_getSegEgbThresholdII(cv::hfs::HfsSegment* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSegEgbThresholdII();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setMinRegionSizeII(int) /usr/include/opencv2/hfs.hpp:81
	void cv_hfs_HfsSegment_setMinRegionSizeII_int(cv::hfs::HfsSegment* instance, int n, Result_void* ocvrs_return) {
		try {
			instance->setMinRegionSizeII(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinRegionSizeII() /usr/include/opencv2/hfs.hpp:82
	void cv_hfs_HfsSegment_getMinRegionSizeII(cv::hfs::HfsSegment* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinRegionSizeII();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setSpatialWeight(float) /usr/include/opencv2/hfs.hpp:94
	void cv_hfs_HfsSegment_setSpatialWeight_float(cv::hfs::HfsSegment* instance, float w, Result_void* ocvrs_return) {
		try {
			instance->setSpatialWeight(w);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSpatialWeight() /usr/include/opencv2/hfs.hpp:95
	void cv_hfs_HfsSegment_getSpatialWeight(cv::hfs::HfsSegment* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSpatialWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setSlicSpixelSize(int) /usr/include/opencv2/hfs.hpp:105
	void cv_hfs_HfsSegment_setSlicSpixelSize_int(cv::hfs::HfsSegment* instance, int n, Result_void* ocvrs_return) {
		try {
			instance->setSlicSpixelSize(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSlicSpixelSize() /usr/include/opencv2/hfs.hpp:106
	void cv_hfs_HfsSegment_getSlicSpixelSize(cv::hfs::HfsSegment* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSlicSpixelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNumSlicIter(int) /usr/include/opencv2/hfs.hpp:113
	void cv_hfs_HfsSegment_setNumSlicIter_int(cv::hfs::HfsSegment* instance, int n, Result_void* ocvrs_return) {
		try {
			instance->setNumSlicIter(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNumSlicIter() /usr/include/opencv2/hfs.hpp:114
	void cv_hfs_HfsSegment_getNumSlicIter(cv::hfs::HfsSegment* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumSlicIter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// performSegmentGpu(cv::InputArray, bool) /usr/include/opencv2/hfs.hpp:125
	void cv_hfs_HfsSegment_performSegmentGpu_const__InputArrayR_bool(cv::hfs::HfsSegment* instance, const cv::_InputArray* src, bool ifDraw, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->performSegmentGpu(*src, ifDraw);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// performSegmentCpu(cv::InputArray, bool) /usr/include/opencv2/hfs.hpp:131
	void cv_hfs_HfsSegment_performSegmentCpu_const__InputArrayR_bool(cv::hfs::HfsSegment* instance, const cv::_InputArray* src, bool ifDraw, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->performSegmentCpu(*src, ifDraw);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// create(int, int, float, int, float, int, float, int, int) /usr/include/opencv2/hfs.hpp:144
	void cv_hfs_HfsSegment_create_int_int_float_int_float_int_float_int_int(int height, int width, float segEgbThresholdI, int minRegionSizeI, float segEgbThresholdII, int minRegionSizeII, float spatialWeight, int slicSpixelSize, int numSlicIter, Result<cv::Ptr<cv::hfs::HfsSegment>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::hfs::HfsSegment> ret = cv::hfs::HfsSegment::create(height, width, segEgbThresholdI, minRegionSizeI, segEgbThresholdII, minRegionSizeII, spatialWeight, slicSpixelSize, numSlicIter);
			Ok(new cv::Ptr<cv::hfs::HfsSegment>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::hfs::HfsSegment>*>))
	}
	
}
