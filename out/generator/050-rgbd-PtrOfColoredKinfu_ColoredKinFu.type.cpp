extern "C" {
	void cv_PtrOfColoredKinfu_ColoredKinFu_delete(cv::Ptr<cv::colored_kinfu::ColoredKinFu>* instance) {
		delete instance;
	}

	const cv::colored_kinfu::ColoredKinFu* cv_PtrOfColoredKinfu_ColoredKinFu_get_inner_ptr(const cv::Ptr<cv::colored_kinfu::ColoredKinFu>* instance) {
		return instance->get();
	}

	cv::colored_kinfu::ColoredKinFu* cv_PtrOfColoredKinfu_ColoredKinFu_get_inner_ptr_mut(cv::Ptr<cv::colored_kinfu::ColoredKinFu>* instance) {
		return instance->get();
	}
}

