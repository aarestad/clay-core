use std::{
    io::Write,
    fs::File,
};
use ocl::{Platform, Device};
use vecmat::{vec::*, mat::*};
use clay_core::{
    Context, Worker,
    scene::ListScene, view::ProjView, map::*,
    shape::*, shape_select,
    material::*, object::Covered,
};
use clay_gui::{Window};


shape_select!(MySelect, {
    Sphere(Sphere),
    Cube(Cube),
});
type MyShape = Covered<Mapper<MySelect, Affine>, Colored<Diffuse>>;
type MyScene = ListScene<MyShape>;
type MyView = ProjView;

fn main_() -> Result<(), clay_core::Error> {
    let platform = Platform::default();
    let device = Device::first(platform)?;

    let context = Context::new(platform, device)?;
    let mut worker = Worker::<MyScene, MyView>::new(&context)?;
    File::create("__gen__kernel.c")?.write_all(worker.programs().render.source().as_bytes())?;

    let objects = vec![
        MySelect::Cube(Cube::new())
        .map(Affine::from(
            Mat3::<f64>::from(
                5.0, 0.0, 0.0,
                0.0, 5.0, 0.0,
                0.0, 0.0, 0.1,
            ),
            Vec3::from(0.0, 0.0, -0.1)),
        )
        .cover(Diffuse {}.color_with(Vec3::from(0.9, 0.9, 0.9))),
        
        MySelect::Cube(Cube::new())
        .map(Affine::from(
            0.5*Mat3::<f64>::one(),
            Vec3::from(1.0, 0.0, 0.5)),
        )
        .cover(Diffuse {}.color_with(Vec3::from(0.5, 0.5, 0.9))),
        
        MySelect::Sphere(Sphere::new())
        .map(Affine::from(
            0.5*Mat3::<f64>::one(),
            Vec3::from(0.0, 1.0, 0.5)),
        )
        .cover(Diffuse {}.color_with(Vec3::from(0.9, 0.5, 0.5))),
        
        MySelect::Sphere(Sphere::new())
        .map(Affine::from(
            0.25*Mat3::<f64>::one(),
            Vec3::from(0.0, 0.0, 0.25)),
        )
        .cover(Diffuse {}.color_with(Vec3::from(0.5, 0.9, 0.5))),
        
    ];
    let scene = MyScene::new(objects, &context)?;

    let mut window = Window::new((1000, 800))?;

    window.start(&context, |screen, pos, map| {
        let view = ProjView { pos, ori: map };
        worker.render(screen, &scene, &view)
    })?;

    Ok(())
}

fn main() {
    match main_() {
        Ok(()) => (),
        Err(err) => panic!("{}", err),
    } 
}
