#include "ocvrs_common.hpp"
#include <opencv2/xobjdetect.hpp>
#include "xobjdetect_types.hpp"

extern "C" {
	// read(const cv::FileNode &) /usr/include/opencv2/xobjdetect.hpp:66
	void cv_xobjdetect_WBDetector_read_const_FileNodeR(cv::xobjdetect::WBDetector* instance, const cv::FileNode* node, Result_void* ocvrs_return) {
		try {
			instance->read(*node);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/xobjdetect.hpp:71
	void cv_xobjdetect_WBDetector_write_const_FileStorageR(const cv::xobjdetect::WBDetector* instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// train(const std::string &, const std::string &) /usr/include/opencv2/xobjdetect.hpp:77
	void cv_xobjdetect_WBDetector_train_const_stringR_const_stringR(cv::xobjdetect::WBDetector* instance, const char* pos_samples, const char* neg_imgs, Result_void* ocvrs_return) {
		try {
			instance->train(std::string(pos_samples), std::string(neg_imgs));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detect(const cv::Mat &, std::vector<Rect> &, std::vector<double> &) /usr/include/opencv2/xobjdetect.hpp:86
	void cv_xobjdetect_WBDetector_detect_const_MatR_vector_Rect_R_vector_double_R(cv::xobjdetect::WBDetector* instance, const cv::Mat* img, std::vector<cv::Rect>* bboxes, std::vector<double>* confidences, Result_void* ocvrs_return) {
		try {
			instance->detect(*img, *bboxes, *confidences);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create() /usr/include/opencv2/xobjdetect.hpp:93
	void cv_xobjdetect_WBDetector_create(Result<cv::Ptr<cv::xobjdetect::WBDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::xobjdetect::WBDetector> ret = cv::xobjdetect::WBDetector::create();
			Ok(new cv::Ptr<cv::xobjdetect::WBDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::xobjdetect::WBDetector>*>))
	}
	
}
