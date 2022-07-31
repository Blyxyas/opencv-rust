extern "C" {
	void cv_PtrOfDetail_SeamFinder_delete(cv::Ptr<cv::detail::SeamFinder>* instance) {
		delete instance;
	}

	const cv::detail::SeamFinder* cv_PtrOfDetail_SeamFinder_get_inner_ptr(const cv::Ptr<cv::detail::SeamFinder>* instance) {
		return instance->get();
	}

	cv::detail::SeamFinder* cv_PtrOfDetail_SeamFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::SeamFinder>* instance) {
		return instance->get();
	}
}

