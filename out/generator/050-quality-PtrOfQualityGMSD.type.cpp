extern "C" {
	cv::Ptr<cv::quality::QualityGMSD>* cv_PtrOfQualityGMSD_new(cv::quality::QualityGMSD* val) {
		return new cv::Ptr<cv::quality::QualityGMSD>(val);
	}
	
	void cv_PtrOfQualityGMSD_delete(cv::Ptr<cv::quality::QualityGMSD>* instance) {
		delete instance;
	}

	const cv::quality::QualityGMSD* cv_PtrOfQualityGMSD_get_inner_ptr(const cv::Ptr<cv::quality::QualityGMSD>* instance) {
		return instance->get();
	}

	cv::quality::QualityGMSD* cv_PtrOfQualityGMSD_get_inner_ptr_mut(cv::Ptr<cv::quality::QualityGMSD>* instance) {
		return instance->get();
	}
}

