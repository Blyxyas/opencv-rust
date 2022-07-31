extern "C" {
	cv::Ptr<cv::quality::QualityMSE>* cv_PtrOfQualityMSE_new(cv::quality::QualityMSE* val) {
		return new cv::Ptr<cv::quality::QualityMSE>(val);
	}
	
	void cv_PtrOfQualityMSE_delete(cv::Ptr<cv::quality::QualityMSE>* instance) {
		delete instance;
	}

	const cv::quality::QualityMSE* cv_PtrOfQualityMSE_get_inner_ptr(const cv::Ptr<cv::quality::QualityMSE>* instance) {
		return instance->get();
	}

	cv::quality::QualityMSE* cv_PtrOfQualityMSE_get_inner_ptr_mut(cv::Ptr<cv::quality::QualityMSE>* instance) {
		return instance->get();
	}
}

