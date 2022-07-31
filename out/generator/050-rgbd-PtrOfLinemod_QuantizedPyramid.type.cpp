extern "C" {
	void cv_PtrOfLinemod_QuantizedPyramid_delete(cv::Ptr<cv::linemod::QuantizedPyramid>* instance) {
		delete instance;
	}

	const cv::linemod::QuantizedPyramid* cv_PtrOfLinemod_QuantizedPyramid_get_inner_ptr(const cv::Ptr<cv::linemod::QuantizedPyramid>* instance) {
		return instance->get();
	}

	cv::linemod::QuantizedPyramid* cv_PtrOfLinemod_QuantizedPyramid_get_inner_ptr_mut(cv::Ptr<cv::linemod::QuantizedPyramid>* instance) {
		return instance->get();
	}
}

