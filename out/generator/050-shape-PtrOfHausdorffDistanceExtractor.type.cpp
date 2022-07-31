extern "C" {
	void cv_PtrOfHausdorffDistanceExtractor_delete(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
		delete instance;
	}

	const cv::HausdorffDistanceExtractor* cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr(const cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
		return instance->get();
	}

	cv::HausdorffDistanceExtractor* cv_PtrOfHausdorffDistanceExtractor_get_inner_ptr_mut(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
		return instance->get();
	}
}

