extern "C" {
	void cv_PtrOfCalibrateRobertson_delete(cv::Ptr<cv::CalibrateRobertson>* instance) {
		delete instance;
	}

	const cv::CalibrateRobertson* cv_PtrOfCalibrateRobertson_get_inner_ptr(const cv::Ptr<cv::CalibrateRobertson>* instance) {
		return instance->get();
	}

	cv::CalibrateRobertson* cv_PtrOfCalibrateRobertson_get_inner_ptr_mut(cv::Ptr<cv::CalibrateRobertson>* instance) {
		return instance->get();
	}
}

