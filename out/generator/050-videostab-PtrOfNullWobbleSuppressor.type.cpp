extern "C" {
	cv::Ptr<cv::videostab::NullWobbleSuppressor>* cv_PtrOfNullWobbleSuppressor_new(cv::videostab::NullWobbleSuppressor* val) {
		return new cv::Ptr<cv::videostab::NullWobbleSuppressor>(val);
	}
	
	void cv_PtrOfNullWobbleSuppressor_delete(cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
		delete instance;
	}

	const cv::videostab::NullWobbleSuppressor* cv_PtrOfNullWobbleSuppressor_get_inner_ptr(const cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
		return instance->get();
	}

	cv::videostab::NullWobbleSuppressor* cv_PtrOfNullWobbleSuppressor_get_inner_ptr_mut(cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::WobbleSuppressorBase>* cv_PtrOfNullWobbleSuppressor_to_PtrOfWobbleSuppressorBase(cv::Ptr<cv::videostab::NullWobbleSuppressor>* instance) {
		return new cv::Ptr<cv::videostab::WobbleSuppressorBase>(instance->dynamicCast<cv::videostab::WobbleSuppressorBase>());
	}
}

