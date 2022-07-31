extern "C" {
	void cv_PtrOfCalibrateDebevec_delete(cv::Ptr<cv::CalibrateDebevec>* instance) {
		delete instance;
	}

	const cv::CalibrateDebevec* cv_PtrOfCalibrateDebevec_get_inner_ptr(const cv::Ptr<cv::CalibrateDebevec>* instance) {
		return instance->get();
	}

	cv::CalibrateDebevec* cv_PtrOfCalibrateDebevec_get_inner_ptr_mut(cv::Ptr<cv::CalibrateDebevec>* instance) {
		return instance->get();
	}
}

