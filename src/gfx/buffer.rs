use std::rc::Rc;

use super::vertex_layout::{VertexComponent, VertexLayout};

/// A buffer object that can be used to store data on the GPU.
pub struct Buffer<T> {
    handle: u32,
    length: usize,
    vertex_layout: Option<Rc<VertexLayout>>,
    _phantom: std::marker::PhantomData<T>,
}

pub type VertexBuffer = Buffer<VertexComponent>;
pub type IndexBuffer = Buffer<u32>;

impl<T> !Send for Buffer<T> {}
impl<T> !Sync for Buffer<T> {}

impl<T> Buffer<T> {
    /// Create a new buffer with the given length (in elements, not bytes).
    pub(crate) fn __from_slice(data: &[T], vertex_layout: Option<Rc<VertexLayout>>) -> Self {
        let mut handle = 0;
        let length = data.len();

        unsafe {
            // Generate a buffer
            gl::GenBuffers(1, &mut handle);

            // Upload the data to the buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, handle);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<T>() * length) as isize,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            // Set the buffer's label
            let label = format!("Buffer<{}>", std::any::type_name::<T>());
            gl::ObjectLabel(
                gl::BUFFER,
                handle,
                label.len() as i32,
                label.as_ptr() as *const _,
            );
        }

        Self {
            handle,
            length,
            vertex_layout,
            _phantom: std::marker::PhantomData,
        }
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

    /// Get the vertex layout of the buffer.
    /// Returns None if the buffer is not a vertex buffer.
    pub fn vertex_layout(&self) -> Option<Rc<VertexLayout>> {
        self.vertex_layout.clone()
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.handle);
        }
    }
}
