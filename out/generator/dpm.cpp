#include "ocvrs_common.hpp"
#include <opencv2/dpm.hpp>
#include "dpm_types.hpp"

extern "C" {
	// isEmpty() /usr/include/opencv2/dpm.hpp:117
	void cv_dpm_DPMDetector_isEmpty_const(const cv::dpm::DPMDetector* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isEmpty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// detect(cv::Mat &, std::vector<ObjectDetection> &) /usr/include/opencv2/dpm.hpp:124
	void cv_dpm_DPMDetector_detect_MatR_vector_ObjectDetection_R(cv::dpm::DPMDetector* instance, cv::Mat* image, std::vector<cv::dpm::DPMDetector::ObjectDetection>* objects, Result_void* ocvrs_return) {
		try {
			instance->detect(*image, *objects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getClassNames() /usr/include/opencv2/dpm.hpp:129
	void cv_dpm_DPMDetector_getClassNames_const(const cv::dpm::DPMDetector* instance, Result<std::vector<std::string>*>* ocvrs_return) {
		try {
			const std::vector<std::string> ret = instance->getClassNames();
			Ok(new const std::vector<std::string>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<std::string>*>))
	}
	
	// getClassCount() /usr/include/opencv2/dpm.hpp:133
	void cv_dpm_DPMDetector_getClassCount_const(const cv::dpm::DPMDetector* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getClassCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// create(const std::vector<std::string> &, const std::vector<std::string> &) /usr/include/opencv2/dpm.hpp:142
	void cv_dpm_DPMDetector_create_const_vector_string_R_const_vector_string_R(const std::vector<std::string>* filenames, const std::vector<std::string>* classNames, Result<cv::Ptr<cv::dpm::DPMDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::dpm::DPMDetector> ret = cv::dpm::DPMDetector::create(*filenames, *classNames);
			Ok(new cv::Ptr<cv::dpm::DPMDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::dpm::DPMDetector>*>))
	}
	
	// rect /usr/include/opencv2/dpm.hpp:112
	void cv_dpm_DPMDetector_ObjectDetection_getPropRect_const(const cv::dpm::DPMDetector::ObjectDetection* instance, cv::Rect* ocvrs_return) {
			cv::Rect ret = instance->rect;
			*ocvrs_return = ret;
	}
	
	// rect /usr/include/opencv2/dpm.hpp:112
	void cv_dpm_DPMDetector_ObjectDetection_setPropRect_Rect(cv::dpm::DPMDetector::ObjectDetection* instance, cv::Rect* val) {
			instance->rect = *val;
	}
	
	// score /usr/include/opencv2/dpm.hpp:113
	float cv_dpm_DPMDetector_ObjectDetection_getPropScore_const(const cv::dpm::DPMDetector::ObjectDetection* instance) {
			float ret = instance->score;
			return ret;
	}
	
	// score /usr/include/opencv2/dpm.hpp:113
	void cv_dpm_DPMDetector_ObjectDetection_setPropScore_float(cv::dpm::DPMDetector::ObjectDetection* instance, float val) {
			instance->score = val;
	}
	
	// classID /usr/include/opencv2/dpm.hpp:114
	int cv_dpm_DPMDetector_ObjectDetection_getPropClassID_const(const cv::dpm::DPMDetector::ObjectDetection* instance) {
			int ret = instance->classID;
			return ret;
	}
	
	// classID /usr/include/opencv2/dpm.hpp:114
	void cv_dpm_DPMDetector_ObjectDetection_setPropClassID_int(cv::dpm::DPMDetector::ObjectDetection* instance, int val) {
			instance->classID = val;
	}
	
	void cv_DPMDetector_ObjectDetection_delete(cv::dpm::DPMDetector::ObjectDetection* instance) {
		delete instance;
	}
	// ObjectDetection() /usr/include/opencv2/dpm.hpp:110
	void cv_dpm_DPMDetector_ObjectDetection_ObjectDetection(Result<cv::dpm::DPMDetector::ObjectDetection*>* ocvrs_return) {
		try {
			cv::dpm::DPMDetector::ObjectDetection* ret = new cv::dpm::DPMDetector::ObjectDetection();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dpm::DPMDetector::ObjectDetection*>))
	}
	
	// ObjectDetection(const cv::Rect &, float, int) /usr/include/opencv2/dpm.hpp:111
	void cv_dpm_DPMDetector_ObjectDetection_ObjectDetection_const_RectR_float_int(const cv::Rect* rect, float score, int classID, Result<cv::dpm::DPMDetector::ObjectDetection*>* ocvrs_return) {
		try {
			cv::dpm::DPMDetector::ObjectDetection* ret = new cv::dpm::DPMDetector::ObjectDetection(*rect, score, classID);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::dpm::DPMDetector::ObjectDetection*>))
	}
	
}
