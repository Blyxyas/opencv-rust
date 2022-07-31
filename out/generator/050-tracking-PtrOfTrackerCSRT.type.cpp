extern "C" {
	void cv_PtrOfTrackerCSRT_delete(cv::Ptr<cv::tracking::TrackerCSRT>* instance) {
		delete instance;
	}

	const cv::tracking::TrackerCSRT* cv_PtrOfTrackerCSRT_get_inner_ptr(const cv::Ptr<cv::tracking::TrackerCSRT>* instance) {
		return instance->get();
	}

	cv::tracking::TrackerCSRT* cv_PtrOfTrackerCSRT_get_inner_ptr_mut(cv::Ptr<cv::tracking::TrackerCSRT>* instance) {
		return instance->get();
	}
}

