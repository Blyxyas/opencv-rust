extern "C" {
	cv::Ptr<cv::detail::GraphCutSeamFinder>* cv_PtrOfDetail_GraphCutSeamFinder_new(cv::detail::GraphCutSeamFinder* val) {
		return new cv::Ptr<cv::detail::GraphCutSeamFinder>(val);
	}
	
	void cv_PtrOfDetail_GraphCutSeamFinder_delete(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
		delete instance;
	}

	const cv::detail::GraphCutSeamFinder* cv_PtrOfDetail_GraphCutSeamFinder_get_inner_ptr(const cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
		return instance->get();
	}

	cv::detail::GraphCutSeamFinder* cv_PtrOfDetail_GraphCutSeamFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrOfDetail_GraphCutSeamFinder_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
		return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
}

