extern "C" {
	void cv_VectorOfGpuMat_delete(std::vector<cv::cuda::GpuMat>* instance) {
		delete instance;
	}

	std::vector<cv::cuda::GpuMat>* cv_VectorOfGpuMat_new() {
		return new std::vector<cv::cuda::GpuMat>();
	}

	size_t cv_VectorOfGpuMat_len(const std::vector<cv::cuda::GpuMat>* instance) {
		return instance->size();
	}

	bool cv_VectorOfGpuMat_is_empty(const std::vector<cv::cuda::GpuMat>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfGpuMat_capacity(const std::vector<cv::cuda::GpuMat>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfGpuMat_resize(std::vector<cv::cuda::GpuMat>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfGpuMat_shrink_to_fit(std::vector<cv::cuda::GpuMat>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfGpuMat_reserve(std::vector<cv::cuda::GpuMat>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfGpuMat_remove(std::vector<cv::cuda::GpuMat>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfGpuMat_swap(std::vector<cv::cuda::GpuMat>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfGpuMat_clear(std::vector<cv::cuda::GpuMat>* instance) {
		instance->clear();
	}

	void cv_VectorOfGpuMat_push(std::vector<cv::cuda::GpuMat>* instance, cv::cuda::GpuMat* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfGpuMat_insert(std::vector<cv::cuda::GpuMat>* instance, size_t index, cv::cuda::GpuMat* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfGpuMat_get(const std::vector<cv::cuda::GpuMat>* instance, size_t index, cv::cuda::GpuMat** ocvrs_return) {
		*ocvrs_return = new cv::cuda::GpuMat((*instance)[index]);
	}

	void cv_VectorOfGpuMat_set(std::vector<cv::cuda::GpuMat>* instance, size_t index, cv::cuda::GpuMat* val) {
		(*instance)[index] = *val;
	}

}


