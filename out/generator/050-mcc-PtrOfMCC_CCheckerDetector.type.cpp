extern "C" {
	void cv_PtrOfMCC_CCheckerDetector_delete(cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
		delete instance;
	}

	const cv::mcc::CCheckerDetector* cv_PtrOfMCC_CCheckerDetector_get_inner_ptr(const cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
		return instance->get();
	}

	cv::mcc::CCheckerDetector* cv_PtrOfMCC_CCheckerDetector_get_inner_ptr_mut(cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
		return instance->get();
	}
}

