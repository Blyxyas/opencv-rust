extern "C" {
	void cv_PtrOfKinfu_KinFu_delete(cv::Ptr<cv::kinfu::KinFu>* instance) {
		delete instance;
	}

	const cv::kinfu::KinFu* cv_PtrOfKinfu_KinFu_get_inner_ptr(const cv::Ptr<cv::kinfu::KinFu>* instance) {
		return instance->get();
	}

	cv::kinfu::KinFu* cv_PtrOfKinfu_KinFu_get_inner_ptr_mut(cv::Ptr<cv::kinfu::KinFu>* instance) {
		return instance->get();
	}
}

