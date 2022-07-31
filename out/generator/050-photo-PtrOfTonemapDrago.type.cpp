extern "C" {
	void cv_PtrOfTonemapDrago_delete(cv::Ptr<cv::TonemapDrago>* instance) {
		delete instance;
	}

	const cv::TonemapDrago* cv_PtrOfTonemapDrago_get_inner_ptr(const cv::Ptr<cv::TonemapDrago>* instance) {
		return instance->get();
	}

	cv::TonemapDrago* cv_PtrOfTonemapDrago_get_inner_ptr_mut(cv::Ptr<cv::TonemapDrago>* instance) {
		return instance->get();
	}
}

