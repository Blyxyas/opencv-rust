extern "C" {
	void cv_PtrOfDetail_BlocksCompensator_delete(cv::Ptr<cv::detail::BlocksCompensator>* instance) {
		delete instance;
	}

	const cv::detail::BlocksCompensator* cv_PtrOfDetail_BlocksCompensator_get_inner_ptr(const cv::Ptr<cv::detail::BlocksCompensator>* instance) {
		return instance->get();
	}

	cv::detail::BlocksCompensator* cv_PtrOfDetail_BlocksCompensator_get_inner_ptr_mut(cv::Ptr<cv::detail::BlocksCompensator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrOfDetail_BlocksCompensator_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::BlocksCompensator>* instance) {
		return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
}

