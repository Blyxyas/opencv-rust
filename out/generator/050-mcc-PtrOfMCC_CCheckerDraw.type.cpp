extern "C" {
	void cv_PtrOfMCC_CCheckerDraw_delete(cv::Ptr<cv::mcc::CCheckerDraw>* instance) {
		delete instance;
	}

	const cv::mcc::CCheckerDraw* cv_PtrOfMCC_CCheckerDraw_get_inner_ptr(const cv::Ptr<cv::mcc::CCheckerDraw>* instance) {
		return instance->get();
	}

	cv::mcc::CCheckerDraw* cv_PtrOfMCC_CCheckerDraw_get_inner_ptr_mut(cv::Ptr<cv::mcc::CCheckerDraw>* instance) {
		return instance->get();
	}
}

