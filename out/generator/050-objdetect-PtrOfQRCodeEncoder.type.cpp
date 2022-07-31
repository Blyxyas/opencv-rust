extern "C" {
	void cv_PtrOfQRCodeEncoder_delete(cv::Ptr<cv::QRCodeEncoder>* instance) {
		delete instance;
	}

	const cv::QRCodeEncoder* cv_PtrOfQRCodeEncoder_get_inner_ptr(const cv::Ptr<cv::QRCodeEncoder>* instance) {
		return instance->get();
	}

	cv::QRCodeEncoder* cv_PtrOfQRCodeEncoder_get_inner_ptr_mut(cv::Ptr<cv::QRCodeEncoder>* instance) {
		return instance->get();
	}
}

