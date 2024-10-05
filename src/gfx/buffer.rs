/// A buffer object that can be used to store data on the GPU.
pub struct Buffer<T> {
    handle: u32,
    length: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> !Send for Buffer<T> {}
impl<T> !Sync for Buffer<T> {}

impl<T> Buffer<T> {
    /// Create a new buffer with the given length (in elements, not bytes).
    pub(crate) fn __from_slice(data: &[T]) -> Self {
        let mut handle = 0;
        let length = data.len();

        unsafe {
            // Generate a buffer
            gl::GenBuffers(1, &mut handle);

            // Create the buffer's immutable storage
            gl::NamedBufferStorage(
                handle,
                length as isize * std::mem::size_of::<T>() as isize,
                data.as_ptr() as *const _,
                0
            );
        }

        Self { handle, length, _phantom: std::marker::PhantomData }
    }

    /// Get the length of the buffer.
    /// This is the number of elements in the buffer, not the number of bytes.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Get the GL handle.
    pub fn handle(&self) -> u32 {
        self.handle
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.handle);
        }
    }
}