extern "C" {
	void cv_PtrOfDPMDetector_delete(cv::Ptr<cv::dpm::DPMDetector>* instance) {
		delete instance;
	}

	const cv::dpm::DPMDetector* cv_PtrOfDPMDetector_get_inner_ptr(const cv::Ptr<cv::dpm::DPMDetector>* instance) {
		return instance->get();
	}

	cv::dpm::DPMDetector* cv_PtrOfDPMDetector_get_inner_ptr_mut(cv::Ptr<cv::dpm::DPMDetector>* instance) {
		return instance->get();
	}
}

