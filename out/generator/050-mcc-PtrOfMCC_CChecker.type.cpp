extern "C" {
	void cv_PtrOfMCC_CChecker_delete(cv::Ptr<cv::mcc::CChecker>* instance) {
		delete instance;
	}

	const cv::mcc::CChecker* cv_PtrOfMCC_CChecker_get_inner_ptr(const cv::Ptr<cv::mcc::CChecker>* instance) {
		return instance->get();
	}

	cv::mcc::CChecker* cv_PtrOfMCC_CChecker_get_inner_ptr_mut(cv::Ptr<cv::mcc::CChecker>* instance) {
		return instance->get();
	}
}

