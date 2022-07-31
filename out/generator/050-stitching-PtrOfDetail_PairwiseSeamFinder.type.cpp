extern "C" {
	void cv_PtrOfDetail_PairwiseSeamFinder_delete(cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
		delete instance;
	}

	const cv::detail::PairwiseSeamFinder* cv_PtrOfDetail_PairwiseSeamFinder_get_inner_ptr(const cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
		return instance->get();
	}

	cv::detail::PairwiseSeamFinder* cv_PtrOfDetail_PairwiseSeamFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrOfDetail_PairwiseSeamFinder_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
		return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
}

