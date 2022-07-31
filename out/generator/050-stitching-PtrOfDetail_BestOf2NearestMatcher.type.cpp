extern "C" {
	cv::Ptr<cv::detail::BestOf2NearestMatcher>* cv_PtrOfDetail_BestOf2NearestMatcher_new(cv::detail::BestOf2NearestMatcher* val) {
		return new cv::Ptr<cv::detail::BestOf2NearestMatcher>(val);
	}
	
	void cv_PtrOfDetail_BestOf2NearestMatcher_delete(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
		delete instance;
	}

	const cv::detail::BestOf2NearestMatcher* cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr(const cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
		return instance->get();
	}

	cv::detail::BestOf2NearestMatcher* cv_PtrOfDetail_BestOf2NearestMatcher_get_inner_ptr_mut(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::FeaturesMatcher>* cv_PtrOfDetail_BestOf2NearestMatcher_to_PtrOfDetail_FeaturesMatcher(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
		return new cv::Ptr<cv::detail::FeaturesMatcher>(instance->dynamicCast<cv::detail::FeaturesMatcher>());
	}
}

