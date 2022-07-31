extern "C" {
	void cv_PtrOfTrackerDaSiamRPN_delete(cv::Ptr<cv::TrackerDaSiamRPN>* instance) {
		delete instance;
	}

	const cv::TrackerDaSiamRPN* cv_PtrOfTrackerDaSiamRPN_get_inner_ptr(const cv::Ptr<cv::TrackerDaSiamRPN>* instance) {
		return instance->get();
	}

	cv::TrackerDaSiamRPN* cv_PtrOfTrackerDaSiamRPN_get_inner_ptr_mut(cv::Ptr<cv::TrackerDaSiamRPN>* instance) {
		return instance->get();
	}
}

