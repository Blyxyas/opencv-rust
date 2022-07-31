extern "C" {
	cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* cv_PtrOfStaticSaliencySpectralResidual_new(cv::saliency::StaticSaliencySpectralResidual* val) {
		return new cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>(val);
	}
	
	void cv_PtrOfStaticSaliencySpectralResidual_delete(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
		delete instance;
	}

	const cv::saliency::StaticSaliencySpectralResidual* cv_PtrOfStaticSaliencySpectralResidual_get_inner_ptr(const cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
		return instance->get();
	}

	cv::saliency::StaticSaliencySpectralResidual* cv_PtrOfStaticSaliencySpectralResidual_get_inner_ptr_mut(cv::Ptr<cv::saliency::StaticSaliencySpectralResidual>* instance) {
		return instance->get();
	}
}

