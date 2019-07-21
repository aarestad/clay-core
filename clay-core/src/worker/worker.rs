use std::{
    marker::PhantomData,
};
use ocl::{self, prm};
use vecmat::{
    vec::*,
    mat::*,
};
use crate::{
    Shape,
    Context, Screen,
    buffer::ObjectBuffer,
};
use super::Program;


#[allow(dead_code)]
pub struct Worker<T: Shape> {
    kernel: ocl::Kernel,
    queue: ocl::Queue,
    phantom: PhantomData<T>,
}

impl<T: Shape> Worker<T> {
    pub fn new(context: &Context) -> crate::Result<Self> {
        let queue = context.queue().clone();

        let program = Program::<T>::new()?.build(context)?;

        // build kernel
        let kernel = ocl::Kernel::builder()
        .program(&program)
        .name("fill")
        .queue(queue.clone())
        .arg(&prm::Int2::zero())
        .arg(None::<&ocl::Buffer<u8>>)
        .arg(&prm::Float3::zero())
        .arg(&prm::Float16::zero())
        .arg(None::<&ocl::Buffer<i32>>)
        .arg(None::<&ocl::Buffer<f32>>)
        .arg(&0i32)
        .arg(&0i32)
        .arg(&0i32)
        .build()?;

        Ok(Self { kernel, queue, phantom: PhantomData })
    }

    pub fn render(
        &self,
        screen: &mut Screen,
        pos: Vec3<f64>,
        map: Mat3<f64>,
        objects: &ObjectBuffer<T>,
    ) -> crate::Result<()> {
        let dims = screen.dims();
        let dims = prm::Int2::new(dims.0 as i32, dims.1 as i32);
        self.kernel.set_arg(0, &dims)?;
        self.kernel.set_arg(1, screen.buffer_mut())?;

        let mapf = map.map(|x| x as f32);
        let mut map16 = [0f32; 16];
        map16[0..3].copy_from_slice(&mapf.row(0).data);
        map16[4..7].copy_from_slice(&mapf.row(1).data);
        map16[8..11].copy_from_slice(&mapf.row(2).data);
        self.kernel.set_arg(2, &prm::Float3::from(pos.map(|e| e as f32).data))?;
        self.kernel.set_arg(3, &prm::Float16::from(map16))?;

        self.kernel.set_arg(4, objects.buffer_int())?;
        self.kernel.set_arg(5, objects.buffer_float())?;
        self.kernel.set_arg(6, T::size_int() as i32)?;
        self.kernel.set_arg(7, T::size_float() as i32)?;
        self.kernel.set_arg(8, objects.count() as i32)?;

        unsafe {
            self.kernel
            .cmd()
            .global_work_size(screen.dims())
            .enq()?;
        }

        Ok(())
    }
}