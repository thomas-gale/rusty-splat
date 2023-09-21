# rusty-splat
Random educational exercise because 3D Gaussian Spatting is interesting and writing stuff in rust/webgpu is fun.

## TODO
- [x] Read https://huggingface.co/blog/gaussian-splatting
- [x] Read & experiment with https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/
- [ ] Investigate https://github.com/cvlab-epfl/gaussian-splatting-web
- [ ] Follow thread https://xi.zulipchat.com/#narrow/stream/197075-gpu/topic/Gaussian.20splatting (see if radix sort for webgpu is implemented...)
  - [ ] Associated note on radix sort: https://developer.download.nvidia.com/video/gputechconf/gtc/2020/presentations/s21572-a-faster-radix-sort-implementation.pdf
- [ ] Investigate https://github.com/aras-p/UnityGaussianSplatting
- [ ] Integrate with https://bevyengine.org/ (wgpu adaptor / custom shader bits)
- [ ] Initially use imported pre-generated gaussians like https://aras-p.info/blog/2023/09/05/Gaussian-Splatting-is-pretty-cool/
- [ ] Maybe use https://docs.rs/opencv/latest/opencv/sfm/index.html#:~:text=Structure%20From%20Motion,Sameer%20Agarwal%20and%20Keir%20Mierle. to directly convert images to points?
- [ ] Adapt the training phase to use https://docs.rs/dfdx/latest/dfdx/
