extern "C" {
	void cv_PtrOfTonemapDurand_delete(cv::Ptr<cv::xphoto::TonemapDurand>* instance) {
		delete instance;
	}

	const cv::xphoto::TonemapDurand* cv_PtrOfTonemapDurand_get_inner_ptr(const cv::Ptr<cv::xphoto::TonemapDurand>* instance) {
		return instance->get();
	}

	cv::xphoto::TonemapDurand* cv_PtrOfTonemapDurand_get_inner_ptr_mut(cv::Ptr<cv::xphoto::TonemapDurand>* instance) {
		return instance->get();
	}
}

