#include <cuda.h>
#include <stdio.h>
#include "../output/ptx_code.h"

int main() {
    CUdevice device;
    CUcontext context;
    CUmodule module;
    CUfunction function;

    printf("Initializing CUDA\n");
    cuInit(0);
    cuDeviceGet(&device, 0);
    cuCtxCreate(&context, 0, device);

    printf("Loading PTX\n");
    cuModuleLoadData(&module, ptx_code);
    cuModuleGetFunction(&function, module, "add");

    // Allocate GPU memory
    printf("Allocating GPU memory\n");
    CUdeviceptr d_result;
    cuMemAlloc(&d_result, sizeof(int));
    
    int a = 2, b = 2;
    
    // Pass the device pointer directly, not pointer to it
    void* args[] = { &d_result, &a, &b };

    printf("Launching kernel\n");
    cuLaunchKernel(function, 1, 1, 1, 1, 1, 1, 0, NULL, args, NULL);
    cuCtxSynchronize();

    int result;
    cuMemcpyDtoH(&result, d_result, sizeof(int));
    printf("Result: %d\n", result);

    cuMemFree(d_result);
    return 0;
}