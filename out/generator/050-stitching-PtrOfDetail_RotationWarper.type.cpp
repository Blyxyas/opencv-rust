extern "C" {
	void cv_PtrOfDetail_RotationWarper_delete(cv::Ptr<cv::detail::RotationWarper>* instance) {
		delete instance;
	}

	const cv::detail::RotationWarper* cv_PtrOfDetail_RotationWarper_get_inner_ptr(const cv::Ptr<cv::detail::RotationWarper>* instance) {
		return instance->get();
	}

	cv::detail::RotationWarper* cv_PtrOfDetail_RotationWarper_get_inner_ptr_mut(cv::Ptr<cv::detail::RotationWarper>* instance) {
		return instance->get();
	}
}

