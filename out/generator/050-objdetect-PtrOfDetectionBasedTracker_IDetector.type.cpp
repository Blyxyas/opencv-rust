extern "C" {
	void cv_PtrOfDetectionBasedTracker_IDetector_delete(cv::Ptr<cv::DetectionBasedTracker::IDetector>* instance) {
		delete instance;
	}

	const cv::DetectionBasedTracker::IDetector* cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr(const cv::Ptr<cv::DetectionBasedTracker::IDetector>* instance) {
		return instance->get();
	}

	cv::DetectionBasedTracker::IDetector* cv_PtrOfDetectionBasedTracker_IDetector_get_inner_ptr_mut(cv::Ptr<cv::DetectionBasedTracker::IDetector>* instance) {
		return instance->get();
	}
}

