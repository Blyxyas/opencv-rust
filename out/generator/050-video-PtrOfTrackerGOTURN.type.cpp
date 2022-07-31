extern "C" {
	void cv_PtrOfTrackerGOTURN_delete(cv::Ptr<cv::TrackerGOTURN>* instance) {
		delete instance;
	}

	const cv::TrackerGOTURN* cv_PtrOfTrackerGOTURN_get_inner_ptr(const cv::Ptr<cv::TrackerGOTURN>* instance) {
		return instance->get();
	}

	cv::TrackerGOTURN* cv_PtrOfTrackerGOTURN_get_inner_ptr_mut(cv::Ptr<cv::TrackerGOTURN>* instance) {
		return instance->get();
	}
}

