extern "C" {
	void cv_PtrOfGeneralizedHoughBallard_delete(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
		delete instance;
	}

	const cv::GeneralizedHoughBallard* cv_PtrOfGeneralizedHoughBallard_get_inner_ptr(const cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
		return instance->get();
	}

	cv::GeneralizedHoughBallard* cv_PtrOfGeneralizedHoughBallard_get_inner_ptr_mut(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
		return instance->get();
	}
}

