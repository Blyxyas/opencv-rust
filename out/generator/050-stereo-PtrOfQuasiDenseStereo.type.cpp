extern "C" {
	void cv_PtrOfQuasiDenseStereo_delete(cv::Ptr<cv::stereo::QuasiDenseStereo>* instance) {
		delete instance;
	}

	const cv::stereo::QuasiDenseStereo* cv_PtrOfQuasiDenseStereo_get_inner_ptr(const cv::Ptr<cv::stereo::QuasiDenseStereo>* instance) {
		return instance->get();
	}

	cv::stereo::QuasiDenseStereo* cv_PtrOfQuasiDenseStereo_get_inner_ptr_mut(cv::Ptr<cv::stereo::QuasiDenseStereo>* instance) {
		return instance->get();
	}
}

