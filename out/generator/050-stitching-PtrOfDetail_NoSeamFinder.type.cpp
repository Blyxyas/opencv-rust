extern "C" {
	cv::Ptr<cv::detail::NoSeamFinder>* cv_PtrOfDetail_NoSeamFinder_new(cv::detail::NoSeamFinder* val) {
		return new cv::Ptr<cv::detail::NoSeamFinder>(val);
	}
	
	void cv_PtrOfDetail_NoSeamFinder_delete(cv::Ptr<cv::detail::NoSeamFinder>* instance) {
		delete instance;
	}

	const cv::detail::NoSeamFinder* cv_PtrOfDetail_NoSeamFinder_get_inner_ptr(const cv::Ptr<cv::detail::NoSeamFinder>* instance) {
		return instance->get();
	}

	cv::detail::NoSeamFinder* cv_PtrOfDetail_NoSeamFinder_get_inner_ptr_mut(cv::Ptr<cv::detail::NoSeamFinder>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrOfDetail_NoSeamFinder_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::NoSeamFinder>* instance) {
		return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
}

