extern "C" {
	void cv_PtrOfWobbleSuppressorBase_delete(cv::Ptr<cv::videostab::WobbleSuppressorBase>* instance) {
		delete instance;
	}

	const cv::videostab::WobbleSuppressorBase* cv_PtrOfWobbleSuppressorBase_get_inner_ptr(const cv::Ptr<cv::videostab::WobbleSuppressorBase>* instance) {
		return instance->get();
	}

	cv::videostab::WobbleSuppressorBase* cv_PtrOfWobbleSuppressorBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::WobbleSuppressorBase>* instance) {
		return instance->get();
	}
}

