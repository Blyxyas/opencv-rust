extern "C" {
	cv::Ptr<cv::detail::GainCompensator>* cv_PtrOfDetail_GainCompensator_new(cv::detail::GainCompensator* val) {
		return new cv::Ptr<cv::detail::GainCompensator>(val);
	}
	
	void cv_PtrOfDetail_GainCompensator_delete(cv::Ptr<cv::detail::GainCompensator>* instance) {
		delete instance;
	}

	const cv::detail::GainCompensator* cv_PtrOfDetail_GainCompensator_get_inner_ptr(const cv::Ptr<cv::detail::GainCompensator>* instance) {
		return instance->get();
	}

	cv::detail::GainCompensator* cv_PtrOfDetail_GainCompensator_get_inner_ptr_mut(cv::Ptr<cv::detail::GainCompensator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrOfDetail_GainCompensator_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::GainCompensator>* instance) {
		return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
}

