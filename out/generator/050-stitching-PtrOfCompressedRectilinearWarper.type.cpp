extern "C" {
	cv::Ptr<cv::CompressedRectilinearWarper>* cv_PtrOfCompressedRectilinearWarper_new(cv::CompressedRectilinearWarper* val) {
		return new cv::Ptr<cv::CompressedRectilinearWarper>(val);
	}
	
	void cv_PtrOfCompressedRectilinearWarper_delete(cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
		delete instance;
	}

	const cv::CompressedRectilinearWarper* cv_PtrOfCompressedRectilinearWarper_get_inner_ptr(const cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
		return instance->get();
	}

	cv::CompressedRectilinearWarper* cv_PtrOfCompressedRectilinearWarper_get_inner_ptr_mut(cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfCompressedRectilinearWarper_to_PtrOfWarperCreator(cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

