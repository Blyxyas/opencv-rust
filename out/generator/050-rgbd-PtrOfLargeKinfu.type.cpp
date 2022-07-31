extern "C" {
	void cv_PtrOfLargeKinfu_delete(cv::Ptr<cv::large_kinfu::LargeKinfu>* instance) {
		delete instance;
	}

	const cv::large_kinfu::LargeKinfu* cv_PtrOfLargeKinfu_get_inner_ptr(const cv::Ptr<cv::large_kinfu::LargeKinfu>* instance) {
		return instance->get();
	}

	cv::large_kinfu::LargeKinfu* cv_PtrOfLargeKinfu_get_inner_ptr_mut(cv::Ptr<cv::large_kinfu::LargeKinfu>* instance) {
		return instance->get();
	}
}

