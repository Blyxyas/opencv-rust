extern "C" {
	void cv_PtrOfDetail_Estimator_delete(cv::Ptr<cv::detail::Estimator>* instance) {
		delete instance;
	}

	const cv::detail::Estimator* cv_PtrOfDetail_Estimator_get_inner_ptr(const cv::Ptr<cv::detail::Estimator>* instance) {
		return instance->get();
	}

	cv::detail::Estimator* cv_PtrOfDetail_Estimator_get_inner_ptr_mut(cv::Ptr<cv::detail::Estimator>* instance) {
		return instance->get();
	}
}

