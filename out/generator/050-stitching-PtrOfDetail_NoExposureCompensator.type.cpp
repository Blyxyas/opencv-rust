extern "C" {
	cv::Ptr<cv::detail::NoExposureCompensator>* cv_PtrOfDetail_NoExposureCompensator_new(cv::detail::NoExposureCompensator* val) {
		return new cv::Ptr<cv::detail::NoExposureCompensator>(val);
	}
	
	void cv_PtrOfDetail_NoExposureCompensator_delete(cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
		delete instance;
	}

	const cv::detail::NoExposureCompensator* cv_PtrOfDetail_NoExposureCompensator_get_inner_ptr(const cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
		return instance->get();
	}

	cv::detail::NoExposureCompensator* cv_PtrOfDetail_NoExposureCompensator_get_inner_ptr_mut(cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrOfDetail_NoExposureCompensator_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
		return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
}

