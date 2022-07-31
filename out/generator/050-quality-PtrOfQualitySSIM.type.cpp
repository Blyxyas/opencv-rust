extern "C" {
	cv::Ptr<cv::quality::QualitySSIM>* cv_PtrOfQualitySSIM_new(cv::quality::QualitySSIM* val) {
		return new cv::Ptr<cv::quality::QualitySSIM>(val);
	}
	
	void cv_PtrOfQualitySSIM_delete(cv::Ptr<cv::quality::QualitySSIM>* instance) {
		delete instance;
	}

	const cv::quality::QualitySSIM* cv_PtrOfQualitySSIM_get_inner_ptr(const cv::Ptr<cv::quality::QualitySSIM>* instance) {
		return instance->get();
	}

	cv::quality::QualitySSIM* cv_PtrOfQualitySSIM_get_inner_ptr_mut(cv::Ptr<cv::quality::QualitySSIM>* instance) {
		return instance->get();
	}
}

