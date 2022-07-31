extern "C" {
	cv::Ptr<cv::detail::ChannelsCompensator>* cv_PtrOfDetail_ChannelsCompensator_new(cv::detail::ChannelsCompensator* val) {
		return new cv::Ptr<cv::detail::ChannelsCompensator>(val);
	}
	
	void cv_PtrOfDetail_ChannelsCompensator_delete(cv::Ptr<cv::detail::ChannelsCompensator>* instance) {
		delete instance;
	}

	const cv::detail::ChannelsCompensator* cv_PtrOfDetail_ChannelsCompensator_get_inner_ptr(const cv::Ptr<cv::detail::ChannelsCompensator>* instance) {
		return instance->get();
	}

	cv::detail::ChannelsCompensator* cv_PtrOfDetail_ChannelsCompensator_get_inner_ptr_mut(cv::Ptr<cv::detail::ChannelsCompensator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrOfDetail_ChannelsCompensator_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::ChannelsCompensator>* instance) {
		return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}
}

