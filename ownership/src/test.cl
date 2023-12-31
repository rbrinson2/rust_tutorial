
// DAXPY in OpenCL
__kernel void daxpy(__global double *x, __global double *y, double a, int n) {
    int i = get_global_id(0);
    if (i < n) 
    {
        y[i] = a * x[i] + y[i];
    }
}
