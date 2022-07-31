extern "C" {
	void cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_delete(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
		delete instance;
	}

	const cv::videostab::MoreAccurateMotionWobbleSuppressorBase* cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_get_inner_ptr(const cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
		return instance->get();
	}

	cv::videostab::MoreAccurateMotionWobbleSuppressorBase* cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::WobbleSuppressorBase>* cv_PtrOfMoreAccurateMotionWobbleSuppressorBase_to_PtrOfWobbleSuppressorBase(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
		return new cv::Ptr<cv::videostab::WobbleSuppressorBase>(instance->dynamicCast<cv::videostab::WobbleSuppressorBase>());
	}
}

