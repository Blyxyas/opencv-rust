extern "C" {
	cv::Ptr<cv::quality::QualityBRISQUE>* cv_PtrOfQualityBRISQUE_new(cv::quality::QualityBRISQUE* val) {
		return new cv::Ptr<cv::quality::QualityBRISQUE>(val);
	}
	
	void cv_PtrOfQualityBRISQUE_delete(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
		delete instance;
	}

	const cv::quality::QualityBRISQUE* cv_PtrOfQualityBRISQUE_get_inner_ptr(const cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
		return instance->get();
	}

	cv::quality::QualityBRISQUE* cv_PtrOfQualityBRISQUE_get_inner_ptr_mut(cv::Ptr<cv::quality::QualityBRISQUE>* instance) {
		return instance->get();
	}
}

