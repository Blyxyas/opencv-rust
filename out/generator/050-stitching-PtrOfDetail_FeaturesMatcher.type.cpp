extern "C" {
	void cv_PtrOfDetail_FeaturesMatcher_delete(cv::Ptr<cv::detail::FeaturesMatcher>* instance) {
		delete instance;
	}

	const cv::detail::FeaturesMatcher* cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr(const cv::Ptr<cv::detail::FeaturesMatcher>* instance) {
		return instance->get();
	}

	cv::detail::FeaturesMatcher* cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr_mut(cv::Ptr<cv::detail::FeaturesMatcher>* instance) {
		return instance->get();
	}
}

