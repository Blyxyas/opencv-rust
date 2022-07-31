extern "C" {
	cv::Ptr<cv::mcc::DetectorParameters>* cv_PtrOfMCC_DetectorParameters_new(cv::mcc::DetectorParameters* val) {
		return new cv::Ptr<cv::mcc::DetectorParameters>(val);
	}
	
	void cv_PtrOfMCC_DetectorParameters_delete(cv::Ptr<cv::mcc::DetectorParameters>* instance) {
		delete instance;
	}

	const cv::mcc::DetectorParameters* cv_PtrOfMCC_DetectorParameters_get_inner_ptr(const cv::Ptr<cv::mcc::DetectorParameters>* instance) {
		return instance->get();
	}

	cv::mcc::DetectorParameters* cv_PtrOfMCC_DetectorParameters_get_inner_ptr_mut(cv::Ptr<cv::mcc::DetectorParameters>* instance) {
		return instance->get();
	}
}

