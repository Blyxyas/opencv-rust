extern "C" {
	void cv_PtrOfTrackerMIL_delete(cv::Ptr<cv::TrackerMIL>* instance) {
		delete instance;
	}

	const cv::TrackerMIL* cv_PtrOfTrackerMIL_get_inner_ptr(const cv::Ptr<cv::TrackerMIL>* instance) {
		return instance->get();
	}

	cv::TrackerMIL* cv_PtrOfTrackerMIL_get_inner_ptr_mut(cv::Ptr<cv::TrackerMIL>* instance) {
		return instance->get();
	}
}

