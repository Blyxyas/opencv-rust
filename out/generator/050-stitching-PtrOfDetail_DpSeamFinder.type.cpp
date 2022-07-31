extern "C" {
	cv::Ptr<cv::detail::DpSeamFinder>* cv_PtrOfDetail_DpSeamFinder_new(cv::detail::DpSeamFinder* val) {
		return new cv::Ptr<cv::detail::DpSeamFinder>(val);
	}
	
	void cv_PtrOfDetail_DpSeamFinder_delete(cv::Ptr<cv::detail::DpSeamFinder>* instance) {
		delete instance;
	}

	const cv::detail::DpSeamFinder* cv_PtrOfDetail_DpSeamFinder_get_inner_ptr(const cv::Ptr<cv::detail::DpSeamFinder>* instance) {
		return instance->get();
	}

	cv::detail::DpSeamFinder* cv_PtrOfDetail_DpSeamFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::DpSeamFinder>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrOfDetail_DpSeamFinder_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::DpSeamFinder>* instance) {
		return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
}

