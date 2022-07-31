extern "C" {
	void cv_PtrOfGeneralizedHoughGuil_delete(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
		delete instance;
	}

	const cv::GeneralizedHoughGuil* cv_PtrOfGeneralizedHoughGuil_get_inner_ptr(const cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
		return instance->get();
	}

	cv::GeneralizedHoughGuil* cv_PtrOfGeneralizedHoughGuil_get_inner_ptr_mut(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
		return instance->get();
	}
}

