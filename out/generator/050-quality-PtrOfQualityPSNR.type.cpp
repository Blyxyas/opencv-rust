extern "C" {
	cv::Ptr<cv::quality::QualityPSNR>* cv_PtrOfQualityPSNR_new(cv::quality::QualityPSNR* val) {
		return new cv::Ptr<cv::quality::QualityPSNR>(val);
	}
	
	void cv_PtrOfQualityPSNR_delete(cv::Ptr<cv::quality::QualityPSNR>* instance) {
		delete instance;
	}

	const cv::quality::QualityPSNR* cv_PtrOfQualityPSNR_get_inner_ptr(const cv::Ptr<cv::quality::QualityPSNR>* instance) {
		return instance->get();
	}

	cv::quality::QualityPSNR* cv_PtrOfQualityPSNR_get_inner_ptr_mut(cv::Ptr<cv::quality::QualityPSNR>* instance) {
		return instance->get();
	}
}

