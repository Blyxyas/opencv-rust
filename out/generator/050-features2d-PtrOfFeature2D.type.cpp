extern "C" {
	cv::Ptr<cv::Feature2D>* cv_PtrOfFeature2D_new(cv::Feature2D* val) {
		return new cv::Ptr<cv::Feature2D>(val);
	}
	
	void cv_PtrOfFeature2D_delete(cv::Ptr<cv::Feature2D>* instance) {
		delete instance;
	}

	const cv::Feature2D* cv_PtrOfFeature2D_get_inner_ptr(const cv::Ptr<cv::Feature2D>* instance) {
		return instance->get();
	}

	cv::Feature2D* cv_PtrOfFeature2D_get_inner_ptr_mut(cv::Ptr<cv::Feature2D>* instance) {
		return instance->get();
	}
}

